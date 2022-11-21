use std::cell::Cell;

mod binding {
    use lazy_static::lazy_static;

    const OUT_DIR: &str = env!("OUT_DIR");

    lazy_static! {
        static ref TONLIB: libloading::Library = unsafe {
            let lib_path = std::env::var("TON_LIB_PATH").unwrap_or(format!(
                "{OUT_DIR}/distlib/{}/libtonlibjson.{}.so",
                std::env::consts::OS,
                std::env::consts::ARCH
            ));
            libloading::Library::new(lib_path).unwrap()
        };
        pub static ref TONLIB_CLIENT_SET_VERBOSITY_LEVEL: libloading::Symbol<'static, unsafe extern "C" fn(log_level: i32)> =
            unsafe { TONLIB.get(b"tonlib_client_set_verbosity_level").unwrap() };
        pub static ref TONLIB_CLIENT_JSON_CREATE: libloading::Symbol<'static, unsafe extern "C" fn() -> *mut std::ffi::c_void> =
            unsafe { TONLIB.get(b"tonlib_client_json_create").unwrap() };
        pub static ref TONLIB_CLIENT_JSON_SEND: libloading::Symbol<
            'static,
            unsafe extern "C" fn(client: *mut std::ffi::c_void, request: *const std::ffi::c_char),
        > = unsafe { TONLIB.get(b"tonlib_client_json_send").unwrap() };
        pub static ref TONLIB_CLIENT_JSON_RECEIVE: libloading::Symbol<
            'static,
            unsafe extern "C" fn(
                client: *mut std::ffi::c_void,
                timeout: std::ffi::c_double,
            ) -> *const std::ffi::c_char,
        > = unsafe { TONLIB.get(b"tonlib_client_json_receive").unwrap() };
        pub static ref TONLIB_CLIENT_JSON_EXECUTE: libloading::Symbol<
            'static,
            unsafe extern "C" fn(
                client: *mut std::ffi::c_void,
                request: *const std::ffi::c_char,
            ) -> *const std::ffi::c_char,
        > = unsafe { TONLIB.get(b"tonlib_client_json_execute").unwrap() };
        pub static ref TONLIB_CLIENT_JSON_DESTROY: libloading::Symbol<'static, unsafe extern "C" fn(client: *mut std::ffi::c_void)> =
            unsafe { TONLIB.get(b"tonlib_client_json_destroy").unwrap() };
    }
}

pub struct RawClient {
    timeout: std::time::Duration,
    client: Cell<*mut std::ffi::c_void>,
}

impl RawClient {
    pub fn new(log_level: i8) -> Self {
        Self {
            timeout: std::time::Duration::from_millis(1),
            client: Cell::new(unsafe {
                binding::TONLIB_CLIENT_SET_VERBOSITY_LEVEL(log_level as i32);
                binding::TONLIB_CLIENT_JSON_CREATE()
            }),
        }
    }

    pub fn send(&mut self, req: &str) {
        self._send(req);
    }

    pub fn receive(&mut self) -> Option<String> {
        self._receive()
    }

    fn _send(&mut self, req: &str) {
        unsafe {
            let item = std::ffi::CString::new(req).unwrap();
            binding::TONLIB_CLIENT_JSON_SEND(self.client.get(), item.as_ptr())
        };
    }

    fn _execute(&mut self, req: &str) -> Option<String> {
        unsafe {
            let item = std::ffi::CString::new(req).unwrap();
            let response_buf: *const std::ffi::c_char =
                binding::TONLIB_CLIENT_JSON_EXECUTE(self.client.get(), item.as_ptr());

            if response_buf.is_null() {
                return None;
            }

            let response_str = std::ffi::CStr::from_ptr(response_buf);
            return Some(response_str.to_str().unwrap().to_owned());
        }
    }

    fn _receive(&mut self) -> Option<String> {
        unsafe {
            let response_buf: *const std::ffi::c_char =
                binding::TONLIB_CLIENT_JSON_RECEIVE(self.client.get(), self.timeout.as_secs_f64());
            if response_buf.is_null() {
                return None;
            }

            let response_str = std::ffi::CStr::from_ptr(response_buf);
            return Some(response_str.to_str().unwrap().to_owned());
        }
    }
}

impl Drop for RawClient {
    fn drop(&mut self) {
        drop(self.timeout);
        unsafe {
            binding::TONLIB_CLIENT_JSON_DESTROY(self.client.get());
        }
        drop(self.client.get_mut());
    }
}

unsafe impl Send for RawClient {}
