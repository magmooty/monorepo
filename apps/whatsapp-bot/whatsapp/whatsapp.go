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
	logLevel  string
}

func New(logLevel string) (*WhatsAppBot, error) {
	dbLog := waLog.Stdout("Database", logLevel, true)
	container, err := sqlstore.New("sqlite3", "file:whatsapp.db?_foreign_keys=on", dbLog)
	if err != nil {
		return nil, err
	}
	return &WhatsAppBot{
		container: container,
		logLevel:  logLevel,
	}, nil
}

type ConnectionStatus string

const (
	SignedIn             ConnectionStatus = "signed_in"
	SignedOut            ConnectionStatus = "signed_out"
	QRCodeGenerated      ConnectionStatus = "qr_code_generated"
	WhatsAppLibraryError ConnectionStatus = "whatsapp_library_error"
	MessageSent          ConnectionStatus = "message_sent"
	TargetNotOnWhatsApp  ConnectionStatus = "target_not_on_whatsapp"
	NotConnected         ConnectionStatus = "not_connected"
)

func (wb *WhatsAppBot) IsSignedIn() (ConnectionStatus, string) {
	if wb.client == nil {
		return SignedOut, ""
	}

	if !wb.client.IsConnected() {
		return NotConnected, ""
	}

	if wb.client.Store.ID == nil {
		return SignedOut, ""
	}

	return SignedIn, ""
}

func (wb *WhatsAppBot) SendMessage(phoneNumber string, message string) (ConnectionStatus, string) {
	status, _ := wb.IsSignedIn()

	if status != SignedIn {
		return status, "Not signed in and connected to WhatsApp"
	}

	responses, err := wb.client.IsOnWhatsApp([]string{phoneNumber})

	if err != nil {
		return WhatsAppLibraryError, err.Error()
	}

	if len(responses) <= 0 {
		return TargetNotOnWhatsApp, "Target is not on WhatsApp"
	}

	for _, response := range responses {
		_, err = wb.client.SendMessage(context.Background(), response.JID, &waProto.Message{
			Conversation: proto.String(message),
		})
		if err != nil {
			return WhatsAppLibraryError, err.Error()
		}
	}

	return MessageSent, ""
}

func (wb *WhatsAppBot) InitializeClient() (ConnectionStatus, error) {
	deviceStore, err := wb.container.GetFirstDevice()

	if err != nil {
		return WhatsAppLibraryError, err
	}

	log := waLog.Stdout("Client", wb.logLevel, true)
	client := whatsmeow.NewClient(deviceStore, log)

	if client.Store.ID == nil {
		return SignedOut, err
	} else {
		err = client.Connect()

		if err != nil {
			return NotConnected, err
		}
	}
	wb.client = client

	return SignedIn, nil
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

	log := waLog.Stdout("Client", wb.logLevel, true)
	client := whatsmeow.NewClient(deviceStore, log)
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
