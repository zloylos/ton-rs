use std::cell::Cell;

use super::interop;

pub struct RawClient {
    timeout: std::time::Duration,
    client: Cell<*mut std::ffi::c_void>,
}

impl RawClient {
    pub fn new(log_level: i8) -> Self {
        Self {
            timeout: std::time::Duration::from_millis(1),
            client: Cell::new(unsafe {
                interop::TONLIB_CLIENT_SET_VERBOSITY_LEVEL(log_level as i32);
                interop::TONLIB_CLIENT_JSON_CREATE()
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
            interop::TONLIB_CLIENT_JSON_SEND(self.client.get(), item.as_ptr())
        };
    }

    fn _execute(&mut self, req: &str) -> Option<String> {
        unsafe {
            let item = std::ffi::CString::new(req).unwrap();
            let response_buf: *const std::ffi::c_char =
                interop::TONLIB_CLIENT_JSON_EXECUTE(self.client.get(), item.as_ptr());

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
                interop::TONLIB_CLIENT_JSON_RECEIVE(self.client.get(), self.timeout.as_secs_f64());
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
            interop::TONLIB_CLIENT_JSON_DESTROY(self.client.get());
        }
        drop(self.client.get_mut());
    }
}

unsafe impl Send for RawClient {}
