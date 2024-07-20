extern crate libc;

use libc::{c_char, c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_int;

// Define the struct that will be used in C
#[repr(C)]
pub struct CInfoResponse {
    connection_status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[link(name = "whatsapp")]
extern "C" {
    fn initialize() -> ();
    fn info() -> *mut CInfoResponse;
}

pub fn initialize_whatsapp() {
    unsafe {
        initialize();
    }
}

pub struct InfoResponse {
    pub connection_status: String,
    pub error_message: String,
}

pub fn get_info() -> InfoResponse {
    unsafe {
        let response = info();

        if !response.is_null() {
            let connection_status = CStr::from_ptr((*response).connection_status)
                .to_str()
                .unwrap();
            let error_message = CStr::from_ptr((*response).error_message).to_str().unwrap();

            let info_response = InfoResponse {
                connection_status: connection_status.to_string(),
                error_message: error_message.to_string(),
            };

            libc::free((*response).connection_status as *mut c_void);
            libc::free((*response).error_message as *mut c_void);

            info_response
        } else {
            InfoResponse {
                connection_status: "whatsapp_library_error".to_string(),
                error_message: "Unhandled error while communicating with WhatsApp library"
                    .to_string(),
            }
        }
    }
}
