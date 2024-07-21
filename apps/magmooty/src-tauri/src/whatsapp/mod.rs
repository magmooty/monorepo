extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

// Define the structs that will be used in C
#[repr(C)]
pub struct CInfoResponse {
    connection_status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[repr(C)]
pub struct CStartConnectionResponse {
    code: *mut libc::c_char,
    connection_status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[repr(C)]
pub struct CSendMessageResponse {
    message_status: *mut libc::c_char,
    connection_status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[link(name = "whatsapp")]
extern "C" {
    fn wa_initialize() -> ();
    fn wa_info() -> *mut CInfoResponse;
    fn wa_start_connection() -> *mut CStartConnectionResponse;
    fn wa_send_message(
        phone_number: *const libc::c_char,
        message: *const libc::c_char,
    ) -> *mut CSendMessageResponse;
}

pub fn initialize_whatsapp() {
    unsafe {
        wa_initialize();
    }
}

pub struct WAInfoResponse {
    pub connection_status: String,
    pub error_message: String,
}

pub fn get_info() -> WAInfoResponse {
    unsafe {
        let response = wa_info();

        if !response.is_null() {
            let connection_status = CStr::from_ptr((*response).connection_status)
                .to_str()
                .unwrap();
            let error_message = CStr::from_ptr((*response).error_message).to_str().unwrap();

            let info_response = WAInfoResponse {
                connection_status: connection_status.to_string(),
                error_message: error_message.to_string(),
            };

            libc::free((*response).connection_status as *mut libc::c_void);
            libc::free((*response).error_message as *mut libc::c_void);

            info_response
        } else {
            WAInfoResponse {
                connection_status: "whatsapp_library_error".to_string(),
                error_message: "Unhandled error while communicating with WhatsApp library"
                    .to_string(),
            }
        }
    }
}

pub struct WAStartConnectionResponse {
    pub code: String,
    pub connection_status: String,
    pub error_message: String,
}

pub fn start_connection() -> WAStartConnectionResponse {
    unsafe {
        let response = wa_start_connection();

        if !response.is_null() {
            let code = CStr::from_ptr((*response).code).to_str().unwrap();
            let connection_status = CStr::from_ptr((*response).connection_status)
                .to_str()
                .unwrap();
            let error_message = CStr::from_ptr((*response).error_message).to_str().unwrap();

            let start_connection_response = WAStartConnectionResponse {
                code: code.to_string(),
                connection_status: connection_status.to_string(),
                error_message: error_message.to_string(),
            };

            libc::free((*response).code as *mut libc::c_void);
            libc::free((*response).connection_status as *mut libc::c_void);
            libc::free((*response).error_message as *mut libc::c_void);

            start_connection_response
        } else {
            WAStartConnectionResponse {
                code: "".to_string(),
                connection_status: "whatsapp_library_error".to_string(),
                error_message: "Unhandled error while communicating with WhatsApp library"
                    .to_string(),
            }
        }
    }
}

pub struct WASendMessageResponse {
    pub message_status: String,
    pub connection_status: String,
    pub error_message: String,
}

pub fn send_message(phone_number: String, message: String) -> WASendMessageResponse {
    unsafe {
        // Create CStrings from Rust strings
        let c_phone_number = CString::new(phone_number).unwrap();
        let c_message = CString::new(message).unwrap();

        let response = wa_send_message(c_phone_number.as_ptr(), c_message.as_ptr());

        if !response.is_null() {
            let message_status = CStr::from_ptr((*response).message_status).to_str().unwrap();
            let connection_status = CStr::from_ptr((*response).connection_status)
                .to_str()
                .unwrap();
            let error_message = CStr::from_ptr((*response).error_message).to_str().unwrap();

            let send_message_response = WASendMessageResponse {
                message_status: message_status.to_string(),
                connection_status: connection_status.to_string(),
                error_message: error_message.to_string(),
            };

            libc::free((*response).message_status as *mut libc::c_void);
            libc::free((*response).connection_status as *mut libc::c_void);
            libc::free((*response).error_message as *mut libc::c_void);

            send_message_response
        } else {
            WASendMessageResponse {
                message_status: "failed".to_string(),
                connection_status: "whatsapp_library_error".to_string(),
                error_message: "Unhandled error while communicating with WhatsApp library"
                    .to_string(),
            }
        }
    }
}
