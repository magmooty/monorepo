extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use tokio::sync::oneshot;

// Define the structs that will be used in C
#[repr(C)]
pub struct CInfoResponse {
    status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[repr(C)]
pub struct CStartConnectionResponse {
    code: *mut libc::c_char,
    status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[repr(C)]
pub struct CSendMessageResponse {
    status: *mut libc::c_char,
    error_message: *mut libc::c_char,
}

#[link(name = "whatsapp", kind = "static")]
extern "C" {
    fn wa_initialize() -> ();
    fn wa_info(handle: libc::uintptr_t) -> *mut CInfoResponse;
    fn wa_start_connection(handle: libc::uintptr_t) -> *mut CStartConnectionResponse;
    fn wa_send_message(
        handle: libc::uintptr_t,
        phone_number: *const libc::c_char,
        message: *const libc::c_char,
    ) -> *mut CSendMessageResponse;
}

#[no_mangle]
extern "C" fn wa_info_callback(handle: usize, result: *mut CInfoResponse) {
    let tx = unsafe { Box::from_raw(handle as *mut oneshot::Sender<WAInfoResponse>) };

    unsafe {
        if !result.is_null() {
            let status = CStr::from_ptr((*result).status).to_str().unwrap();
            let error_message = CStr::from_ptr((*result).error_message).to_str().unwrap();

            let info_response = WAInfoResponse {
                status: parse_status(status),
                error_message: error_message.to_string(),
            };

            let _ = tx
                .send(info_response)
                .expect("Failed to send back response from WhatsApp library");
        } else {
            let _ = tx
                .send(WAInfoResponse {
                    status: WhatsAppStatus::WhatsAppLibraryError,
                    error_message: "Unhandled error while communicating with WhatsApp library"
                        .to_string(),
                })
                .expect("Failed to send back response from WhatsApp library");
        }
    }
}

#[no_mangle]
extern "C" fn wa_start_connection_callback(handle: usize, result: *mut CStartConnectionResponse) {
    let tx = unsafe { Box::from_raw(handle as *mut oneshot::Sender<WAStartConnectionResponse>) };

    unsafe {
        if !result.is_null() {
            let code = CStr::from_ptr((*result).code).to_str().unwrap();
            let status = CStr::from_ptr((*result).status).to_str().unwrap();
            let error_message = CStr::from_ptr((*result).error_message).to_str().unwrap();

            let start_connection_response = WAStartConnectionResponse {
                code: code.to_string(),
                status: parse_status(status),
                error_message: error_message.to_string(),
            };

            let _ = tx
                .send(start_connection_response)
                .expect("Failed to send back response from WhatsApp library");
        } else {
            let _ = tx
                .send(WAStartConnectionResponse {
                    code: "".to_string(),
                    status: WhatsAppStatus::WhatsAppLibraryError,
                    error_message: "Unhandled error while communicating with WhatsApp library"
                        .to_string(),
                })
                .expect("Failed to send back response from WhatsApp library");
        }
    }
}

#[no_mangle]
extern "C" fn wa_send_message_callback(handle: usize, result: *mut CSendMessageResponse) {
    let tx = unsafe { Box::from_raw(handle as *mut oneshot::Sender<WASendMessageResponse>) };

    unsafe {
        if !result.is_null() {
            let status = CStr::from_ptr((*result).status).to_str().unwrap();
            let error_message = CStr::from_ptr((*result).error_message).to_str().unwrap();

            let send_message_response = WASendMessageResponse {
                status: parse_status(status),
                error_message: error_message.to_string(),
            };

            let _ = tx
                .send(send_message_response)
                .expect("Failed to send back response from WhatsApp library");
        } else {
            let _ = tx
                .send(WASendMessageResponse {
                    status: WhatsAppStatus::WhatsAppLibraryError,
                    error_message: "Unhandled error while communicating with WhatsApp library"
                        .to_string(),
                })
                .expect("Failed to send back response from WhatsApp library");
        }
    }
}

#[derive(Debug)]
pub enum WhatsAppStatus {
    SignedIn,
    SignedOut,
    QRCodeGenerated,
    WhatsAppLibraryError,
    TargetNotOnWhatsApp,
    MessageSent,
}

fn parse_status(status: &str) -> WhatsAppStatus {
    match status {
        "signed_in" => WhatsAppStatus::SignedIn,
        "signed_out" => WhatsAppStatus::SignedOut,
        "qr_code_generated" => WhatsAppStatus::QRCodeGenerated,
        "whatsapp_library_error" => WhatsAppStatus::WhatsAppLibraryError,
        "target_not_on_whatsapp" => WhatsAppStatus::TargetNotOnWhatsApp,
        "message_sent" => WhatsAppStatus::MessageSent,
        _ => WhatsAppStatus::WhatsAppLibraryError,
    }
}

pub fn initialize_whatsapp() {
    unsafe {
        // Leave it as a blocking function because it is run on initialization, we need it to be blocking
        wa_initialize();
    }
}

#[derive(Debug)]
pub struct WAInfoResponse {
    pub status: WhatsAppStatus,
    pub error_message: String,
}

pub async fn get_info() -> WAInfoResponse {
    let (tx, rx) = oneshot::channel();
    let boxed_tx = Box::new(tx);
    let handle = Box::into_raw(boxed_tx) as usize;

    unsafe {
        wa_info(handle as libc::uintptr_t);
    }

    rx.await
        .expect("Failed to communicate with WhatsApp library")
}

#[derive(Debug)]
pub struct WAStartConnectionResponse {
    pub code: String,
    pub status: WhatsAppStatus,
    pub error_message: String,
}

pub async fn start_connection() -> WAStartConnectionResponse {
    let (tx, rx) = oneshot::channel();
    let boxed_tx = Box::new(tx);
    let handle = Box::into_raw(boxed_tx) as usize;

    unsafe {
        wa_start_connection(handle as libc::uintptr_t);
    }

    let result = rx
        .await
        .expect("Failed to communicate with WhatsApp library");

    result
}

#[derive(Debug)]
pub struct WASendMessageResponse {
    pub status: WhatsAppStatus,
    pub error_message: String,
}

pub async fn send_message(phone_number: String, message: String) -> WASendMessageResponse {
    let (tx, rx) = oneshot::channel();
    let boxed_tx = Box::new(tx);
    let handle = Box::into_raw(boxed_tx) as usize;

    unsafe {
        let c_phone_number = CString::new(phone_number).unwrap();
        let c_message = CString::new(message).unwrap();

        wa_send_message(
            handle as libc::uintptr_t,
            c_phone_number.as_ptr(),
            c_message.as_ptr(),
        );
    }

    rx.await
        .expect("Failed to communicate with WhatsApp library")
}
