package whatsapp

import (
	"context"

	"go.mau.fi/whatsmeow"
	waProto "go.mau.fi/whatsmeow/binary/proto"
	"go.mau.fi/whatsmeow/store/sqlstore"
	waLog "go.mau.fi/whatsmeow/util/log"
	"google.golang.org/protobuf/proto"

	_ "github.com/mattn/go-sqlite3"
)

type WhatsAppBot struct {
	container *sqlstore.Container
	client    *whatsmeow.Client
}

func New() (*WhatsAppBot, error) {
	dbLog := waLog.Stdout("Database", "DEBUG", true)
	container, err := sqlstore.New("sqlite3", "file:whatsapp.db?_foreign_keys=on", dbLog)
	if err != nil {
		return nil, err
	}
	return &WhatsAppBot{
		container: container,
	}, nil
}

type ConnectionStatus string

const (
	SignedIn             ConnectionStatus = "signed_in"
	SignedOut            ConnectionStatus = "signed_out"
	QRCodeGenerated      ConnectionStatus = "qr_code_generated"
	WhatsAppLibraryError ConnectionStatus = "whatsapp_library_error"
	TargetNotOnWhatsApp  ConnectionStatus = "target_not_on_whatsapp"
)

func (wb *WhatsAppBot) IsSignedIn() (ConnectionStatus, string) {
	// Get the first device from the container
	deviceStore, err := wb.container.GetFirstDevice()

	if err != nil {
		return WhatsAppLibraryError, err.Error()
	}

	client := whatsmeow.NewClient(deviceStore, nil)

	if client.Store.ID == nil {
		return SignedOut, ""
	} else {
		return SignedIn, ""
	}
}

func (wb *WhatsAppBot) SendMessage(phoneNumber string, message string) (bool, ConnectionStatus, string) {
	responses, err := wb.client.IsOnWhatsApp([]string{phoneNumber})

	if err != nil {
		return false, WhatsAppLibraryError, err.Error()
	}

	if len(responses) <= 0 {
		return false, TargetNotOnWhatsApp, "Target is not on WhatsApp"
	}

	for _, response := range responses {
		_, err = wb.client.SendMessage(context.Background(), response.JID, &waProto.Message{
			Conversation: proto.String(message),
		})
		if err != nil {
			return false, WhatsAppLibraryError, err.Error()
		}
	}
	return true, SignedIn, ""
}

func (wb *WhatsAppBot) InitializeClient() (ConnectionStatus, string) {
	deviceStore, err := wb.container.GetFirstDevice()
	if err != nil {
		return WhatsAppLibraryError, err.Error()
	}
	clientLog := waLog.Stdout("Client", "DEBUG", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)
	if client.Store.ID == nil {
		return SignedOut, ""
	} else {
		err = client.Connect()
		if err != nil {
			return WhatsAppLibraryError, err.Error()
		}
	}
	wb.client = client
	return SignedIn, ""
}

func (wb *WhatsAppBot) StartConnect() (string, ConnectionStatus, string) {
	devices, err := wb.container.GetAllDevices()
	if err != nil {
		return "", WhatsAppLibraryError, err.Error()
	}

	for _, device := range devices {
		wb.container.DeleteDevice(device)
	}

	if wb.client != nil {
		wb.client.Disconnect()
	}

	deviceStore, err := wb.container.GetFirstDevice()
	if err != nil {
		return "", WhatsAppLibraryError, err.Error()
	}

	clientLog := waLog.Stdout("Client", "INFO", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)
	if client.Store.ID == nil {
		qrChan, _ := client.GetQRChannel(context.Background())
		err = client.Connect()

		if err != nil {
			return "", WhatsAppLibraryError, err.Error()
		}

		for evt := range qrChan {
			if evt.Event == "code" {
				wb.client = client
				return evt.Code, QRCodeGenerated, ""
			}
		}

		return "", WhatsAppLibraryError, "Unable to consume QR code channel"
	} else {
		return "", SignedIn, ""
	}
}
