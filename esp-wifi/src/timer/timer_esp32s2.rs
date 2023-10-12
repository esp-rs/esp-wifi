use crate::hal::{
    interrupt,
    macros::interrupt,
    peripherals::{self, TIMG1},
    timer::{Timer, Timer0},
};

pub use super::arch_specific::{get_systimer_count, yield_task, TICKS_PER_SECOND};
use super::arch_specific::{setup_multitasking, setup_timer};

pub fn setup_timer_isr(timg1_timer0: Timer<Timer0<TIMG1>>) {
    unwrap!(interrupt::enable(
        peripherals::Interrupt::TG1_T0_LEVEL,
        interrupt::Priority::Priority2,
    ));

    #[cfg(feature = "wifi")]
    {
        unwrap!(interrupt::enable(
            peripherals::Interrupt::WIFI_MAC,
            interrupt::Priority::Priority1,
        ));
        unwrap!(interrupt::enable(
            peripherals::Interrupt::WIFI_PWR,
            interrupt::Priority::Priority1,
        ));
    }

    setup_timer(timg1_timer0);

    setup_multitasking();
}

#[cfg(feature = "wifi")]
#[interrupt]
fn WIFI_MAC() {
    unsafe {
        let (fnc, arg) = crate::wifi::os_adapter::ISR_INTERRUPT_1;
        trace!("interrupt WIFI_MAC {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }
    }
}

#[cfg(feature = "wifi")]
#[interrupt]
fn WIFI_PWR() {
    unsafe {
        let (fnc, arg) = crate::wifi::os_adapter::ISR_INTERRUPT_1;

        trace!("interrupt WIFI_PWR {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }

        trace!("interrupt 1 done");
    };
}
