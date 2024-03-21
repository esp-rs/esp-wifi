//! WORK IN PROGRESS
//!
//! Currently just empty stubs and logging

#[repr(C)]
struct WifiWpaIeT {
    proto: i32,
    pairwise_cipher: i32,
    group_cipher: i32,
    key_mgmt: i32,
    capabilities: i32,
    num_pmkid: u32,
    pmkid: *const u8,
    mgmt_group_cipher: i32,
    rsnxe_capa: u8,
}

#[repr(C)]
struct WpaFuncs {
    wpa_sta_init: Option<extern "C" fn() -> bool>,
    wpa_sta_deinit: Option<extern "C" fn() -> bool>,
    wpa_sta_connect: Option<extern "C" fn(bssid: *const u8) -> i32>,
    wpa_sta_connected_cb: Option<extern "C" fn(bssid: *mut u8)>,
    wpa_sta_disconnected_cb: Option<extern "C" fn(reason_code: u8)>,
    wpa_sta_rx_eapol: Option<extern "C" fn(src_addr: *const u8, buf: *mut u8, len: u32) -> i32>,
    wpa_sta_in_4way_handshake: Option<extern "C" fn() -> bool>,
    wpa_ap_init: Option<extern "C" fn()>,
    wpa_ap_deinit: Option<extern "C" fn(*const u8) -> bool>,
    wpa_ap_join: Option<
        extern "C" fn(
            sm: *const *const (),
            bssid: *const u8,
            wpa_ie: *const u8,
            wpa_ie_len: u8,
            rsnxe: *const u8,
            rsnxe_len: u8,
            pmf_enable: *const bool,
            subtype: i32,
        ) -> bool,
    >,
    wpa_ap_remove: Option<extern "C" fn(bssid: *const u8)>,
    wpa_ap_get_wpa_ie: Option<extern "C" fn(len: *mut u8) -> u8>,
    wpa_ap_rx_eapol: Option<
        extern "C" fn(hapd_data: *mut (), sm: *mut (), data: *mut u8, data_len: u32) -> bool,
    >,
    wpa_ap_get_peer_spp_msg:
        Option<extern "C" fn(sm: *const (), spp_cap: *mut bool, spp_req: *mut bool)>,
    wpa_config_parse_string: Option<extern "C" fn(value: *const u8, len: u32) -> *mut u8>,
    wpa_parse_wpa_ie:
        Option<extern "C" fn(wpa_ie: *const u8, wpa_ie_len: u32, data: *mut WifiWpaIeT) -> i32>,
    wpa_config_bss: Option<extern "C" fn(bssid: *mut u8) -> i32>,
    wpa_michael_mic_failure: Option<extern "C" fn(is_unicast: u16) -> i32>,
    wpa3_build_sae_msg: Option<extern "C" fn(bssid: *const u8, type_: u32, len: u32) -> *mut u8>,
    wpa3_parse_sae_msg:
        Option<extern "C" fn(buf: *mut u8, len: u32, type_: u32, status: u16) -> i32>,
    wpa3_hostap_handle_auth: Option<
        extern "C" fn(buf: *mut u8, len: u32, type_: u32, status: u16, bssid: *mut u8) -> i32,
    >,
    wpa_sta_rx_mgmt: Option<
        extern "C" fn(
            type_: u8,
            frame: *mut u8,
            len: u32,
            sender: *mut u8,
            rssi: u32,
            channel: u8,
            current_tsf: u64,
        ) -> i32,
    >,
    wpa_config_done: Option<extern "C" fn()>,
    owe_build_dhie: Option<extern "C" fn(group: u16) -> *mut u8>,
    owe_process_assoc_resp: Option<
        extern "C" fn(rsn_ie: *const u8, rsn_len: u32, dh_ie: *const u8, dh_len: u32) -> i32,
    >,
    wpa_sta_set_ap_rsnxe: Option<extern "C" fn(rsnxe: *const u8, rsnxe_ie_len: u32) -> i32>,
}

static mut WPA_FUN: WpaFuncs = WpaFuncs {
    wpa_sta_init: Some(stainit),
    wpa_sta_deinit: Some(stadeinit),
    wpa_sta_connect: Some(staconnect),
    wpa_sta_connected_cb: Some(sta_connected_cb),
    wpa_sta_disconnected_cb: Some(stadisconnect),
    wpa_sta_rx_eapol: Some(wpa_sta_rx_eapol),
    wpa_sta_in_4way_handshake: Some(wpa_sta_in_4way_handshake),
    wpa_ap_init: Some(wpa_ap_init),
    wpa_ap_deinit: Some(wpa_ap_deinit),
    wpa_ap_join: Some(wpa_ap_join),
    wpa_ap_remove: Some(wpa_ap_remove),
    wpa_ap_get_wpa_ie: Some(wpa_ap_get_wpa_ie),
    wpa_ap_rx_eapol: Some(wpa_ap_rx_eapol),
    wpa_ap_get_peer_spp_msg: Some(wpa_ap_get_peer_spp_msg),
    wpa_config_parse_string: Some(wpa_config_parse_string),
    wpa_parse_wpa_ie: Some(wpa_parse_wpa_ie),
    wpa_config_bss: Some(wpa_config_bss),
    wpa_michael_mic_failure: Some(wpa_michael_mic_failure),
    wpa3_build_sae_msg: Some(wpa3_build_sae_msg),
    wpa3_parse_sae_msg: Some(wpa3_parse_sae_msg),
    wpa3_hostap_handle_auth: Some(wpa3_hostap_handle_auth),
    wpa_sta_rx_mgmt: Some(wpa_sta_rx_mgmt),
    wpa_config_done: Some(wpa_config_done),
    owe_build_dhie: Some(owe_build_dhie),
    owe_process_assoc_resp: Some(owe_process_assoc_resp),
    wpa_sta_set_ap_rsnxe: Some(wpa_sta_set_ap_rsnxe),
};

// Internal data structure for wpabuf
#[allow(unused)]
#[repr(C)]
struct WpaBuf {
    size: u32,    // total size of the allocated buffer
    used: u32,    // length of data in the buffer
    buf: *mut u8, // pointer to the head of the buffer
    flags: u32,   // optionally followed by the allocated buffer
}

extern "C" fn sta_connected_cb(bssid: *mut u8) {
    info!("empty sta-connected-cb {:x?}", unsafe {
        core::slice::from_raw_parts(bssid, 6)
    });
}

extern "C" fn wpa_ap_join(
    _sm: *const *const (),
    bssid: *const u8,
    _wpa_ie: *const u8,
    _wpa_ie_len: u8,
    _rsnxe: *const u8,
    _rsnxe_len: u8,
    _pmf_enable: *const bool,
    _subtype: i32,
) -> bool {
    info!("empty stub wpa_ap_join {:x?}", unsafe {
        core::slice::from_raw_parts(bssid, 6)
    });
    false
}

extern "C" fn wpa_ap_remove(bssid: *const u8) {
    info!("empty stub wpa_ap_remove {:x?}", unsafe {
        core::slice::from_raw_parts(bssid, 6)
    });
}

extern "C" fn wpa_ap_get_wpa_ie(_len: *mut u8) -> u8 {
    info!("empty stub wpa_ap_get_wpa_ie");
    0
}

extern "C" fn wpa_ap_rx_eapol(
    _hapd_data: *mut (),
    _sm: *mut (),
    _data: *mut u8,
    _data_len: u32,
) -> bool {
    info!("empty stub wpa_ap_rx_eapol");
    false
}

extern "C" fn wpa_ap_get_peer_spp_msg(_sm: *const (), _spp_cap: *mut bool, _spp_req: *mut bool) {
    info!("empty stub wpa_ap_get_peer_spp_msg");
}

extern "C" fn wpa_config_parse_string(_value: *const u8, _len: u32) -> *mut u8 {
    info!("empty stub wpa_config_parse_string");
    core::ptr::null_mut()
}

extern "C" fn wpa_parse_wpa_ie(wpa_ie: *const u8, wpa_ie_len: u32, _data: *mut WifiWpaIeT) -> i32 {
    info!("empty stub wpa_parse_wpa_ie");
    unsafe {
        let data = core::slice::from_raw_parts(wpa_ie, wpa_ie_len as usize);
        info!("{:02x?}", data);
    }
    0
}

extern "C" fn wpa_config_bss(bssid: *mut u8) -> i32 {
    info!("empty stub wpa_config_bss");
    let bssid = unsafe { core::slice::from_raw_parts(bssid, 6) };
    info!("bssid = {:02x?}", bssid);
    0
}

extern "C" fn wpa_michael_mic_failure(_is_unicast: u16) -> i32 {
    info!("empty stub wpa_michael_mic_failure");
    0
}

extern "C" fn wpa3_build_sae_msg(bssid: *const u8, _type_: u32, _len: u32) -> *mut u8 {
    info!("empty stub wpa3_build_sae_msg {:x?}", unsafe {
        core::slice::from_raw_parts(bssid, 6)
    });
    core::ptr::null_mut()
}

extern "C" fn wpa3_parse_sae_msg(_buf: *mut u8, _len: u32, _type_: u32, _status: u16) -> i32 {
    info!("empty stub pa3_parse_sae_msg");
    0
}

extern "C" fn wpa3_hostap_handle_auth(
    _buf: *mut u8,
    _len: u32,
    _type_: u32,
    _status: u16,
    bssid: *mut u8,
) -> i32 {
    info!("empty stub pa3_hostap_handle_auth {:x?}", unsafe {
        core::slice::from_raw_parts(bssid, 6)
    });
    0
}

extern "C" fn wpa_sta_rx_mgmt(
    type_: u8,
    frame: *mut u8,
    len: u32,
    sender: *mut u8,
    rssi: u32,
    channel: u8,
    current_tsf: u64,
) -> i32 {
    info!("empty stub wpa_sta_rx_mgmt");
    unsafe {
        let data = core::slice::from_raw_parts(frame, len as usize);
        let sender = core::slice::from_raw_parts(sender, 6);
        info!(
            "type {}, frame {:02x?} sender {:02x?} rssi {} channel {} tsf {}",
            type_, data, sender, rssi, channel, current_tsf
        );
    }
    0
}

extern "C" fn wpa_config_done() {
    info!("empty stub wpa_config_done");
}

extern "C" fn owe_build_dhie(group: u16) -> *mut u8 {
    info!("empty stub owe_build_dhie group={group}");

    static mut SOMETHING: [u8; 1024] = [0u8; 1024];

    //core::ptr::null_mut()
    unsafe { &mut SOMETHING as *mut _ as *mut u8 }
}

extern "C" fn owe_process_assoc_resp(
    _rsn_ie: *const u8,
    _rsn_len: u32,
    _dh_ie: *const u8,
    _dh_len: u32,
) -> i32 {
    info!("empty stub owe_process_assoc_resp");
    0
}

extern "C" fn wpa_sta_set_ap_rsnxe(_rsnxe: *const u8, _rsnxe_ie_len: u32) -> i32 {
    info!("empty stub wpa_sta_set_ap_rsnxe");
    0
}

extern "C" fn wpa_ap_deinit(_arg: *const u8) -> bool {
    info!("wpa_ap_deinit");

    true
}

extern "C" fn wpa_ap_init() {
    info!("wpa_ap_init");
}

extern "C" fn wpa_sta_in_4way_handshake() -> bool {
    info!("wpa_sta_in_4way_handshake");

    true
}
extern "C" fn stainit() -> bool {
    info!("stainit");

    true
}

extern "C" fn stadeinit() -> bool {
    info!("stadeinit");

    true
}

extern "C" fn staconnect(arg: *const u8) -> i32 {
    info!("staconnect {:x?}", unsafe {
        core::slice::from_raw_parts(arg, 6)
    });
    0
}

extern "C" fn stadisconnect(arg: u8) {
    info!("stadisconnect reason = {arg}");
}

extern "C" fn wpa_sta_rx_eapol(src_addr: *const u8, buf: *mut u8, len: u32) -> i32 {
    info!("wpa_sta_rx_eapol");
    info!("src ptr {:p} data ptr {:p}, len = {}", src_addr, buf, len);
    0
}

#[no_mangle]
pub(crate) static mut g_wifi_default_wpa_crypto_funcs: esp_wifi_sys::include::wpa_crypto_funcs_t =
    esp_wifi_sys::include::wpa_crypto_funcs_t {
        size: core::mem::size_of::<esp_wifi_sys::include::wpa_crypto_funcs_t>() as u32,
        version: 0x00000001,
        aes_wrap: Some(aes_wrap_fn),
        aes_unwrap: Some(aes_unwrap_fn),
        hmac_sha256_vector: Some(hmac_sha256_vector_fn),
        sha256_prf: Some(sha256_prf_fn),
        hmac_md5: Some(hmac_md5_fn),
        hamc_md5_vector: Some(hamc_md5_vector_fn),
        hmac_sha1: Some(hmac_sha1_fn),
        hmac_sha1_vector: Some(hmac_sha1_vector_fn),
        sha1_prf: Some(sha1_prf_fn),
        sha1_vector: Some(sha1_vector_fn),
        pbkdf2_sha1: Some(pbkdf2_sha1_fn),
        rc4_skip: Some(rc4_skip_fn),
        md5_vector: Some(md5_vector_fn),
        aes_encrypt: Some(aes_encrypt_fn),
        aes_encrypt_init: Some(aes_encrypt_init_fn),
        aes_encrypt_deinit: Some(aes_encrypt_deinit_fn),
        aes_decrypt: Some(aes_decrypt_fn),
        aes_decrypt_init: Some(aes_decrypt_init_fn),
        aes_decrypt_deinit: Some(aes_decrypt_deinit_fn),
        aes_128_encrypt: Some(aes_128_encrypt_fn),
        aes_128_decrypt: Some(aes_128_decrypt_fn),
        omac1_aes_128: Some(omac1_aes_128_fn),
        ccmp_decrypt: Some(ccmp_decrypt_fn),
        ccmp_encrypt: Some(ccmp_encrypt_fn),
        aes_gmac: Some(aes_gmac_fn),
        sha256_vector: Some(sha256_vector_fn),
        crc32: Some(crc32_fn),
    };

unsafe extern "C" fn aes_wrap_fn(
    _kek: *const esp_wifi_sys::c_types::c_uchar,
    _n: esp_wifi_sys::c_types::c_int,
    _plain: *const esp_wifi_sys::c_types::c_uchar,
    _cipher: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("aes_wrap");
    0
}

unsafe extern "C" fn aes_unwrap_fn(
    _kek: *const esp_wifi_sys::c_types::c_uchar,
    _n: esp_wifi_sys::c_types::c_int,
    _cipher: *const esp_wifi_sys::c_types::c_uchar,
    _plain: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("aes_wrap");
    0
}

unsafe extern "C" fn hmac_sha256_vector_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_int,
    _num_elem: esp_wifi_sys::c_types::c_int,
    _addr: *mut *const esp_wifi_sys::c_types::c_uchar,
    _len: *const esp_wifi_sys::c_types::c_int,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("hmac_sha256_vector");
    0
}

unsafe extern "C" fn sha256_prf_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_int,
    _label: *const esp_wifi_sys::c_types::c_char,
    _data: *const esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_int,
    _buf: *mut esp_wifi_sys::c_types::c_uchar,
    _buf_len: esp_wifi_sys::c_types::c_int,
) -> esp_wifi_sys::c_types::c_int {
    info!("sha256_prf");
    0
}

unsafe extern "C" fn hmac_md5_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_uint,
    _data: *const esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("hmac_md5");
    0
}

unsafe extern "C" fn hamc_md5_vector_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_uint,
    _num_elem: esp_wifi_sys::c_types::c_uint,
    _addr: *mut *const esp_wifi_sys::c_types::c_uchar,
    _len: *const esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("hamc_md5_vector");
    0
}

unsafe extern "C" fn hmac_sha1_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_uint,
    _data: *const esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("hmac_sha1");
    0
}

unsafe extern "C" fn hmac_sha1_vector_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_uint,
    _num_elem: esp_wifi_sys::c_types::c_uint,
    _addr: *mut *const esp_wifi_sys::c_types::c_uchar,
    _len: *const esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("hmac_sha1_vector");
    0
}

unsafe extern "C" fn sha1_prf_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _key_len: esp_wifi_sys::c_types::c_uint,
    _label: *const esp_wifi_sys::c_types::c_char,
    _data: *const esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_uint,
    _buf: *mut esp_wifi_sys::c_types::c_uchar,
    _buf_len: esp_wifi_sys::c_types::c_uint,
) -> esp_wifi_sys::c_types::c_int {
    info!("sha1_prf");
    0
}

unsafe extern "C" fn sha1_vector_fn(
    _num_elem: esp_wifi_sys::c_types::c_uint,
    _addr: *mut *const esp_wifi_sys::c_types::c_uchar,
    _len: *const esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("sha1_vector");
    0
}

unsafe extern "C" fn pbkdf2_sha1_fn(
    _passphrase: *const esp_wifi_sys::c_types::c_char,
    _ssid: *const esp_wifi_sys::c_types::c_char,
    _ssid_len: esp_wifi_sys::c_types::c_uint,
    _iterations: esp_wifi_sys::c_types::c_int,
    _buf: *mut esp_wifi_sys::c_types::c_uchar,
    _buflen: esp_wifi_sys::c_types::c_uint,
) -> esp_wifi_sys::c_types::c_int {
    info!("pbkdf2_sha1");
    0
}

unsafe extern "C" fn rc4_skip_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _keylen: esp_wifi_sys::c_types::c_uint,
    _skip: esp_wifi_sys::c_types::c_uint,
    _data: *mut esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_uint,
) -> esp_wifi_sys::c_types::c_int {
    info!("rc4_skip");
    0
}

unsafe extern "C" fn md5_vector_fn(
    _num_elem: esp_wifi_sys::c_types::c_uint,
    _addr: *mut *const esp_wifi_sys::c_types::c_uchar,
    _len: *const esp_wifi_sys::c_types::c_uint,
    _mac: *mut esp_wifi_sys::c_types::c_uchar,
) -> esp_wifi_sys::c_types::c_int {
    info!("md5_vector");
    0
}

unsafe extern "C" fn aes_encrypt_fn(
    _ctx: *mut esp_wifi_sys::c_types::c_void,
    _plain: *const esp_wifi_sys::c_types::c_uchar,
    _crypt: *mut esp_wifi_sys::c_types::c_uchar,
) {
    info!("aes_encrypt");
}

unsafe extern "C" fn aes_encrypt_init_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _len: esp_wifi_sys::c_types::c_uint,
) -> *mut esp_wifi_sys::c_types::c_void {
    info!("aes_encrypt_init");
    core::ptr::null_mut()
}

unsafe extern "C" fn aes_encrypt_deinit_fn(_ctx: *mut esp_wifi_sys::c_types::c_void) {
    info!("aes_encrypt_deinit");
}

unsafe extern "C" fn aes_decrypt_fn(
    _ctx: *mut esp_wifi_sys::c_types::c_void,
    _crypt: *const esp_wifi_sys::c_types::c_uchar,
    _plain: *mut esp_wifi_sys::c_types::c_uchar,
) {
    info!("aes_decrypt");
}

unsafe extern "C" fn aes_decrypt_init_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _len: esp_wifi_sys::c_types::c_uint,
) -> *mut esp_wifi_sys::c_types::c_void {
    info!("aes_decrypt_init");
    core::ptr::null_mut()
}

unsafe extern "C" fn aes_decrypt_deinit_fn(_ctx: *mut esp_wifi_sys::c_types::c_void) {
    info!("aes_decrypt_deinit");
}

unsafe extern "C" fn aes_128_encrypt_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _iv: *const esp_wifi_sys::c_types::c_uchar,
    _data: *mut esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_int,
) -> esp_wifi_sys::c_types::c_int {
    info!("aes_128_encrypt");
    0
}

unsafe extern "C" fn aes_128_decrypt_fn(
    _key: *const esp_wifi_sys::c_types::c_uchar,
    _iv: *const esp_wifi_sys::c_types::c_uchar,
    _data: *mut esp_wifi_sys::c_types::c_uchar,
    _data_len: esp_wifi_sys::c_types::c_int,
) -> esp_wifi_sys::c_types::c_int {
    info!("aes_128_decrypt");
    0
}

unsafe extern "C" fn omac1_aes_128_fn(
    _key: *const u8,
    _data: *const u8,
    _data_len: usize,
    _mic: *mut u8,
) -> esp_wifi_sys::c_types::c_int {
    info!("omac1_aes_128");
    0
}

unsafe extern "C" fn ccmp_decrypt_fn(
    _tk: *const u8,
    _ieee80211_hdr: *const u8,
    _data: *const u8,
    _data_len: usize,
    _decrypted_len: *mut usize,
    _espnow_pkt: bool,
) -> *mut u8 {
    info!("ccmp_decrypt");
    core::ptr::null_mut()
}

unsafe extern "C" fn ccmp_encrypt_fn(
    _tk: *const u8,
    _frame: *mut u8,
    _len: usize,
    _hdrlen: usize,
    _pn: *mut u8,
    _keyid: esp_wifi_sys::c_types::c_int,
    _encrypted_len: *mut usize,
) -> *mut u8 {
    info!("ccmp_encrypt");
    core::ptr::null_mut()
}

unsafe extern "C" fn aes_gmac_fn(
    _key: *const u8,
    _keylen: usize,
    _iv: *const u8,
    _iv_len: usize,
    _aad: *const u8,
    _aad_len: usize,
    _mic: *mut u8,
) -> esp_wifi_sys::c_types::c_int {
    info!("aes_gmac");
    0
}

unsafe extern "C" fn sha256_vector_fn(
    _num_elem: usize,
    _addr: *mut *const u8,
    _len: *const usize,
    _buf: *mut u8,
) -> esp_wifi_sys::c_types::c_int {
    info!("sha256_vector");
    0
}

unsafe extern "C" fn crc32_fn(_crc: u32, _buf: *const u8, _len: u32) -> u32 {
    info!("crc32");
    0
}

#[no_mangle]
pub(crate) extern "C" fn esp_supplicant_init() -> i32 {
    info!("esp_supplicant_init called");

    const WLAN_FC_STYPE_ASSOC_RESP: u32 = 1;
    const WLAN_FC_STYPE_REASSOC_RESP: u32 = 3;
    const WLAN_FC_STYPE_AUTH: u32 = 11;

    extern "C" {
        fn esp_wifi_register_wpa_cb_internal(arg: *const WpaFuncs) -> i32;

        fn esp_wifi_register_mgmt_frame_internal(type_: u32, subtype: u32) -> i32;
    }

    unsafe {        
        esp_wifi_register_mgmt_frame_internal(
            1 << WLAN_FC_STYPE_ASSOC_RESP
            | 1 << WLAN_FC_STYPE_REASSOC_RESP
            | 1 << WLAN_FC_STYPE_AUTH,
            0,
        );

        esp_wifi_register_wpa_cb_internal(&WPA_FUN as *const WpaFuncs);
    }

    0
}

#[no_mangle]
extern "C" fn hexstr2bin() {
    info!("hexstr2bin called");
}
