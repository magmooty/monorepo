package main

// #include <stdlib.h>
// typedef struct {
//     char* connection_status;
//     char* error_message;
// } InfoResponse;
import "C"

import (
	"unsafe"
	whatsapp "whatsapp-bot/whatsapp"
)

var whatsappBot *whatsapp.WhatsAppBot

//export initialize
func initialize() {
	bot, err := whatsapp.New()
	if err != nil {
		panic(err)
	}

	// Set global variable
	whatsappBot = bot
	whatsappBot.InitializeClient()
}

//export info
func info() *C.InfoResponse {
	connectionStatus, errorMessage := whatsappBot.IsSignedIn()

	cConnectionStatus := C.CString(string(connectionStatus))
	cErrorMessage := C.CString(string(errorMessage))

	info := C.InfoResponse{
		connection_status: cConnectionStatus,
		error_message:     cErrorMessage,
	}

	// Allocate memory for the struct in C and copy the struct into it
	pInfo := (*C.InfoResponse)(C.malloc(C.size_t(unsafe.Sizeof(info))))
	*pInfo = info

	return pInfo
}

func main() {}
