#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_net::{
    tcp::TcpSocket,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_net::{
    Config, IpListenEndpoint, Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4,
};
#[path = "../../examples-util/util.rs"]
mod examples_util;
use examples_util::hal;

use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::{AccessPointConfiguration, Configuration, Wifi};
use esp_backtrace as _;
use esp_println::{print, println};
use esp_wifi::wifi::{WifiApDevice, WifiController, WifiDevice, WifiEvent, WifiState};
use esp_wifi::{initialize, EspWifiInitFor};
use hal::clock::ClockControl;
use hal::Rng;
use hal::{embassy, peripherals::Peripherals, prelude::*, timer::TimerGroup};
use static_cell::make_static;

#[main]
async fn main(spawner: Spawner) -> ! {
    #[cfg(feature = "log")]
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    #[cfg(target_arch = "xtensa")]
    let timer = hal::timer::TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    #[cfg(target_arch = "riscv32")]
    let timer = hal::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let wifi = peripherals.WIFI;
    let (wifi_interface, controller) =
        esp_wifi::wifi::new_with_mode(&init, wifi, WifiApDevice).unwrap();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0.timer0);

    let config = Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 2, 1), 24),
        gateway: Some(Ipv4Address::from_bytes(&[192, 168, 2, 1])),
        dns_servers: Default::default(),
    });

    let seed = 1234; // very random, very secure seed

    // Init network stack
    let stack = &*make_static!(Stack::new(
        wifi_interface,
        config,
        make_static!(StackResources::<7>::new()),
        seed
    ));

    spawner.must_spawn(connection(controller));
    spawner.must_spawn(net_task(&stack));

    loop {
        if stack.is_link_up() {
            break;
        }
        yield_now().await;
    }

    spawner.must_spawn(dhcp_server(&stack));
    spawner.must_spawn(dns_server(&stack));
    spawner.must_spawn(web_server(&stack, 1));
    spawner.must_spawn(web_server(&stack, 2));
    spawner.must_spawn(web_server(&stack, 3));

    loop {
        Timer::after(Duration::from_millis(10000)).await;
    }
}

#[embassy_executor::task(pool_size = 3)]
async fn web_server(stack: &'static Stack<WifiDevice<'static, WifiApDevice>>, id: u8) {
    log::info!("Web server {id} starting…");

    let mut rx_buffer = [0; 2048];
    let mut tx_buffer = [0; 2048];

    let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    loop {
        log::info!("Web server {id} waiting for HTTP connection…");
        let r = socket
            .accept(IpListenEndpoint {
                addr: None,
                port: 80,
            })
            .await;
        log::info!("Web server {id} connected.");

        if let Err(e) = r {
            log::error!("Web server {id} connect error: {:?}", e);
            continue;
        }

        use embedded_io_async::Write;

        let mut buffer = [0u8; 2048];
        let mut pos = 0;
        loop {
            match socket.read(&mut buffer).await {
                Ok(0) => {
                    log::info!("Web server {id} read EOF.");
                    break;
                }
                Ok(len) => {
                    let to_print =
                        unsafe { core::str::from_utf8_unchecked(&buffer[..(pos + len)]) };

                    if to_print.contains("\r\n\r\n") {
                        print!("{}", to_print);
                        println!();
                        break;
                    }

                    pos += len;
                }
                Err(e) => {
                    log::error!("Web server {id} read error: {:?}", e);
                    break;
                }
            };
        }

        log::info!("Web server {id} got HTTP request, sending response.");

        let r = socket
            .write_all(
                b"HTTP/1.0 200 OK\r\n\r\n\
          <html>\
            <meta charset='utf-8'>\
            <meta name='viewport' content='width=device-width, initial-scale=1, shrink-to-fit=no'>\
            <body>\
              <h1>Hello Rust!<br>Hello <code>esp-wifi</code>!</h1>\
              <a href='.'>RELOAD</a>\
            </body>\
          </html>\r\n\
          ",
            )
            .await;
        if let Err(e) = r {
            log::error!("Web server {id} write error: {:?}", e);
        }

        log::info!("Web server {id} closing socket.");
        socket.close();

        log::info!("Web server {id} flushing socket.");
        if let Err(e) = socket.flush().await {
            log::error!("Web server {id} flush error: {:?}", e);
        }

        log::info!("Web server {id} aborting socket.");
        socket.abort();
    }
}

#[embassy_executor::task]
async fn dhcp_server(stack: &'static Stack<WifiDevice<'static, WifiApDevice>>) {
    log::info!("DHCP server starting…");

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; 1500];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; 1500];
    let mut socket = UdpSocket::new(
        &stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket
        .bind(IpListenEndpoint {
            addr: None,
            port: 67,
        })
        .unwrap();
    log::info!("DHCP server bound to port 67.");

    let mut server: edge_dhcp::server::Server<10> = edge_dhcp::server::Server {
        range_start: edge_dhcp::Ipv4Addr::new(192, 168, 2, 2),
        range_end: edge_dhcp::Ipv4Addr::new(192, 168, 2, 254),
        leases: Default::default(),
    };

    let server_options = edge_dhcp::server::ServerOptions {
        ip: edge_dhcp::Ipv4Addr::new(192, 168, 2, 1),
        gateways: &[edge_dhcp::Ipv4Addr::new(192, 168, 2, 1)],
        subnet: Some(edge_dhcp::Ipv4Addr::new(255, 255, 255, 0)),
        dns: &[edge_dhcp::Ipv4Addr::new(192, 168, 2, 1)],
        lease_duration_secs: 7200,
    };

    loop {
        let mut buf = [0; 1500];
        match socket.recv_from(&mut buf).await {
            Ok((size, _)) => {
                log::info!("DHCP packet size: {size}");

                let request = match edge_dhcp::Packet::decode(&buf[..size]) {
                    Ok(packet) => packet,
                    Err(err) => {
                        log::error!("Invalid DHCP packet: {err:?}");
                        continue;
                    }
                };

                // log::info!("Received DHCP request from {endpoint:?}:\n{:?}", request);
                //
                // log::info!("Options:");
                // for option in request.options.iter() {
                //   println!("{option:?}");
                // }

                let mut dhcp_options_buf = edge_dhcp::Options::buf();

                if let Some(response) =
                    server.handle_request(&mut dhcp_options_buf, &server_options, &request)
                {
                    // According to RFC 2131, section 4.1.
                    let response_endpoint = if !request.giaddr.is_unspecified() {
                        embassy_net::IpEndpoint {
                            addr: embassy_net::IpAddress::Ipv4(embassy_net::Ipv4Address(
                                request.giaddr.octets(),
                            )),
                            port: 67,
                        }
                    } else {
                        let is_nak = {
                            let mut is_nak = false;

                            for o in request.options.iter() {
                                if matches!(
                                    o,
                                    edge_dhcp::DhcpOption::MessageType(edge_dhcp::MessageType::Nak)
                                ) {
                                    is_nak = true;
                                    break;
                                }
                            }

                            is_nak
                        };

                        if is_nak {
                            embassy_net::IpEndpoint {
                                addr: embassy_net::IpAddress::Ipv4(
                                    embassy_net::Ipv4Address::BROADCAST,
                                ),
                                port: 68,
                            }
                        } else if !request.ciaddr.is_unspecified() {
                            embassy_net::IpEndpoint {
                                addr: embassy_net::IpAddress::Ipv4(embassy_net::Ipv4Address(
                                    request.ciaddr.octets(),
                                )),
                                port: 68,
                            }
                        } else if request.broadcast {
                            embassy_net::IpEndpoint {
                                addr: embassy_net::IpAddress::Ipv4(
                                    embassy_net::Ipv4Address::BROADCAST,
                                ),
                                port: 68,
                            }
                        } else {
                            embassy_net::IpEndpoint {
                                // FIXME: Unicast doesn't seem to work. How could it if the client doesn't have an IP yet?
                                // addr: embassy_net::IpAddress::Ipv4(embassy_net::Ipv4Address(response.yiaddr.octets())),
                                addr: embassy_net::IpAddress::Ipv4(
                                    embassy_net::Ipv4Address::BROADCAST,
                                ),
                                port: 68,
                            }
                        }
                    };

                    println!("Sending DHCP response to {response_endpoint}.");
                    //
                    // println!("Options:");
                    // for option in response.options.iter() {
                    //   println!("{option:?}");
                    // }

                    let response_bytes = match response.encode(&mut buf) {
                        Ok(response) => response,
                        Err(err) => {
                            log::error!("Failed to encode DHCP response: {err:?}");
                            continue;
                        }
                    };

                    match socket.send_to(response_bytes, response_endpoint).await {
                        Ok(()) => (),
                        Err(err) => log::error!("Error sending response: {err:?}"),
                    }
                }
            }
            Err(err) => {
                log::error!("Failed to receive packet: {err:?}");
            }
        };
    }
}

#[embassy_executor::task]
async fn dns_server(stack: &'static Stack<WifiDevice<'static, WifiApDevice>>) {
    log::info!("Starting DNS server…");

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; 512];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; 512];
    let mut socket = UdpSocket::new(
        &stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket
        .bind(IpListenEndpoint {
            addr: None,
            port: 53,
        })
        .unwrap();
    log::info!("DNS server bound to port 53.");

    loop {
        let mut buf = [0; 512];
        match socket.recv_from(&mut buf).await {
            Ok((size, endpoint)) => {
                log::info!("DNS packet size: {size}");

                let request = &buf[..size];

                log::info!("Received DNS request from {endpoint:?}: {:?}", request);
                let response = match edge_captive::process_dns_request(
                    request,
                    &[192, 168, 2, 1],
                    core::time::Duration::from_secs(30),
                ) {
                    Ok(response) => response,
                    Err(err) => {
                        log::error!("Failed to process DNS request: {err}");
                        continue;
                    }
                };

                match socket.send_to(response.as_ref(), endpoint).await {
                    Ok(()) => (),
                    Err(err) => log::error!("Error sending response: {err:?}"),
                }
            }
            Err(err) => {
                log::error!("Failed to receive packet: {err:?}");
            }
        };
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    log::info!("Starting connection task.");
    log::info!("Device capabilities: {:?}", controller.get_capabilities());
    loop {
        match esp_wifi::wifi::get_wifi_state() {
            WifiState::ApStarted => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::AccessPoint(AccessPointConfiguration {
                ssid: "esp-wifi".try_into().unwrap(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            log::info!("Starting WiFi…");
            controller.start().await.unwrap();
            log::info!("WiFi started.");
        }
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<WifiDevice<'static, WifiApDevice>>) {
    log::info!("Initializing network stack.");
    stack.run().await
}
