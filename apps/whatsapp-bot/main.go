package main

// #include <stdlib.h>
// #include <stdbool.h>
// typedef struct {
//     char* connection_status;
//     char* error_message;
// } InfoResponse;
//
// typedef struct {
//     char* code;
//     char* connection_status;
//     char* error_message;
// } StartConnectionResponse;
//
// typedef struct {
//     char* message_status;
//     char* connection_status;
//     char* error_message;
// } SendMessageResponse;
import "C"

import (
	"unsafe"
	whatsapp "whatsapp-bot/whatsapp"
)

var whatsappBot *whatsapp.WhatsAppBot

//export wa_initialize
func wa_initialize() {
	bot, err := whatsapp.New()
	if err != nil {
		panic(err)
	}

	// Set global variable
	whatsappBot = bot
	whatsappBot.InitializeClient()
}

//export wa_info
func wa_info() *C.InfoResponse {
	connectionStatus, errorMessage := whatsappBot.IsSignedIn()

	cConnectionStatus := C.CString(string(connectionStatus))
	cErrorMessage := C.CString(string(errorMessage))

	response := C.InfoResponse{
		connection_status: cConnectionStatus,
		error_message:     cErrorMessage,
	}

	// Allocate memory for the struct in C and copy the struct into it
	pResponse := (*C.InfoResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
	*pResponse = response

	return pResponse
}

//export wa_start_connection
func wa_start_connection() *C.StartConnectionResponse {
	code, connectionStatus, errorMessage := whatsappBot.StartConnect()

	cCode := C.CString(string(code))
	cConnectionStatus := C.CString(string(connectionStatus))
	cErrorMessage := C.CString(string(errorMessage))

	response := C.StartConnectionResponse{
		code:              cCode,
		connection_status: cConnectionStatus,
		error_message:     cErrorMessage,
	}

	// Allocate memory for the struct in C and copy the struct into it
	pResponse := (*C.StartConnectionResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
	*pResponse = response

	return pResponse
}

//export wa_send_message
func wa_send_message(phoneNumber *C.char, message *C.char) *C.SendMessageResponse {
	sent, connectionStatus, errorMessage := whatsappBot.SendMessage(C.GoString(phoneNumber), C.GoString(message))

	messageStatus := ""

	if sent {
		messageStatus = "successful"
	} else {
		messageStatus = "failed"
	}

	cMessageStatus := C.CString(messageStatus)
	cConnectionStatus := C.CString(string(connectionStatus))
	cErrorMessage := C.CString(string(errorMessage))

	response := C.SendMessageResponse{
		message_status:    cMessageStatus,
		connection_status: cConnectionStatus,
		error_message:     cErrorMessage,
	}

	// Allocate memory for the struct in C and copy the struct into it
	pResponse := (*C.SendMessageResponse)(C.malloc(C.size_t(unsafe.Sizeof(response))))
	*pResponse = response

	return pResponse
}

func main() {}
