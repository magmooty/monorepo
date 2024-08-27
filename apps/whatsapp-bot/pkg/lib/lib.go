package main

/*
#include <unistd.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    char* status;
    char* error_message;
} InfoResponse;

typedef struct {
    char* code;
    char* status;
    char* error_message;
} StartConnectionResponse;

typedef struct {
    char* status;
    char* error_message;
} SendMessageResponse;

extern void wa_info_callback(uintptr_t handle, InfoResponse* result);
extern void wa_start_connection_callback(uintptr_t handle, StartConnectionResponse* result);
extern void wa_send_message_callback(uintptr_t handle, SendMessageResponse* result);
*/
import "C"

import (
	"unsafe"
	whatsapp "whatsapp-bot/whatsapp"
)

var whatsAppBot *whatsapp.WhatsAppBot

//export wa_initialize
func wa_initialize() {
	bot, err := whatsapp.New("ERROR")
	if err != nil {
		panic(err)
	}

	// Set global variable
	whatsAppBot = bot
	whatsAppBot.InitializeClient()
}

//export wa_info
func wa_info(handle C.uintptr_t) {
	go func() {
		status, errorMessage := whatsAppBot.IsSignedIn()

		cStatus := C.CString(string(status))
		cErrorMessage := C.CString(string(errorMessage))

		response := C.InfoResponse{
			status:        cStatus,
			error_message: cErrorMessage,
		}

		// Allocate memory for the struct in C and copy the struct into it
		pResponse := (*C.InfoResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
		*pResponse = response

		C.wa_info_callback(handle, pResponse)

		// Free the C strings
		C.free(unsafe.Pointer(cStatus))
		C.free(unsafe.Pointer(cErrorMessage))

		// Free the struct
		C.free(unsafe.Pointer(pResponse))
	}()
}

//export wa_start_connection
func wa_start_connection(handle C.uintptr_t) {
	go func() {
		code, status, errorMessage := whatsAppBot.StartConnect()

		cCode := C.CString(string(code))
		cStatus := C.CString(string(status))
		cErrorMessage := C.CString(string(errorMessage))

		response := C.StartConnectionResponse{
			code:          cCode,
			status:        cStatus,
			error_message: cErrorMessage,
		}

		// Allocate memory for the struct in C and copy the struct into it
		pResponse := (*C.StartConnectionResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
		*pResponse = response

		C.wa_start_connection_callback(handle, pResponse)

		// Free the C strings
		C.free(unsafe.Pointer(cCode))
		C.free(unsafe.Pointer(cStatus))
		C.free(unsafe.Pointer(cErrorMessage))

		// Free the struct
		C.free(unsafe.Pointer(pResponse))
	}()
}

//export wa_send_message
func wa_send_message(handle C.uintptr_t, phoneNumber *C.char, message *C.char) {
	goPhoneNumber := C.GoString(phoneNumber)
	goMessage := C.GoString(message)

	go func() {
		status, errorMessage := whatsAppBot.SendMessage(goPhoneNumber, goMessage)

		cStatus := C.CString(string(status))
		cErrorMessage := C.CString(string(errorMessage))

		response := C.SendMessageResponse{
			status:        cStatus,
			error_message: cErrorMessage,
		}

		// Allocate memory for the struct in C and copy the struct into it
		pResponse := (*C.SendMessageResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
		*pResponse = response

		C.wa_send_message_callback(handle, pResponse)

		// Free the C strings
		C.free(unsafe.Pointer(cStatus))
		C.free(unsafe.Pointer(cErrorMessage))

		// Free the struct
		C.free(unsafe.Pointer(pResponse))
	}()
}

func main() {}
