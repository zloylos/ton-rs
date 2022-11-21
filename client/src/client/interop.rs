use lazy_static::lazy_static;

const OUT_DIR: &str = env!("OUT_DIR");

lazy_static! {
    // tonlib
    static ref TONLIB: libloading::Library = unsafe {
        let lib_path = std::env::var("TON_LIB_PATH").unwrap_or(format!(
            "{OUT_DIR}/distlib/{}/libtonlibjson.{}.so",
            std::env::consts::OS,
            std::env::consts::ARCH
        ));
        libloading::Library::new(lib_path).unwrap()
    };
    // tonlib_client_set_verbosity_level
    pub static ref TONLIB_CLIENT_SET_VERBOSITY_LEVEL: libloading::Symbol<'static, unsafe extern "C" fn(log_level: i32)> =
        unsafe { TONLIB.get(b"tonlib_client_set_verbosity_level").unwrap() };
    // tonlib_client_json_create
    pub static ref TONLIB_CLIENT_JSON_CREATE: libloading::Symbol<'static, unsafe extern "C" fn() -> *mut std::ffi::c_void> =
        unsafe { TONLIB.get(b"tonlib_client_json_create").unwrap() };
    // tonlib_client_json_send
    pub static ref TONLIB_CLIENT_JSON_SEND: libloading::Symbol<
        'static,
        unsafe extern "C" fn(client: *mut std::ffi::c_void, request: *const std::ffi::c_char),
    > = unsafe { TONLIB.get(b"tonlib_client_json_send").unwrap() };
    // tonlib_client_json_receive
    pub static ref TONLIB_CLIENT_JSON_RECEIVE: libloading::Symbol<
        'static,
        unsafe extern "C" fn(
            client: *mut std::ffi::c_void,
            timeout: std::ffi::c_double,
        ) -> *const std::ffi::c_char,
    > = unsafe { TONLIB.get(b"tonlib_client_json_receive").unwrap() };
    // tonlib_client_json_execute
    pub static ref TONLIB_CLIENT_JSON_EXECUTE: libloading::Symbol<
        'static,
        unsafe extern "C" fn(
            client: *mut std::ffi::c_void,
            request: *const std::ffi::c_char,
        ) -> *const std::ffi::c_char,
    > = unsafe { TONLIB.get(b"tonlib_client_json_execute").unwrap() };
    // tonlib_client_json_destroy
    pub static ref TONLIB_CLIENT_JSON_DESTROY: libloading::Symbol<'static, unsafe extern "C" fn(client: *mut std::ffi::c_void)> =
        unsafe { TONLIB.get(b"tonlib_client_json_destroy").unwrap() };
}
