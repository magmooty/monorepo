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

func (wb *WhatsAppBot) IsSignedIn() bool {
	// Get the first device from the container
	deviceStore, err := wb.container.GetFirstDevice()
	if err != nil {
		return false
	}

	clientLog := waLog.Stdout("Client", "INFO", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)

	// Check if the client is connected to WhatsApp
	if client.Store.ID == nil {
		return false
	} else {
		return true
	}
}

func (wb *WhatsAppBot) SendMessage(phoneNumber string, message string) bool {
	responses, err := wb.client.IsOnWhatsApp([]string{phoneNumber})
	if err != nil {
		return false
	}
	for _, response := range responses {
		_, err = wb.client.SendMessage(context.Background(), response.JID, &waProto.Message{
			Conversation: proto.String(message),
		})
		if err != nil {
			return false
		}
	}
	return true
}

func (wb *WhatsAppBot) InitializeClient() bool {
	deviceStore, err := wb.container.GetFirstDevice()
	if err != nil {
		panic(err)
	}
	clientLog := waLog.Stdout("Client", "DEBUG", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)
	if client.Store.ID == nil {
		return false
	} else {
		err = client.Connect()
		if err != nil {
			panic(err)
		}
	}
	wb.client = client
	return true
}

type ConnectionStatus string

const (
	SignedIn      ConnectionStatus = "SignedIn"
	CodeGenerated ConnectionStatus = "CodeGenerated"
	Error         ConnectionStatus = "Error"
)

func (wb *WhatsAppBot) StartConnect() (string, ConnectionStatus) {
	devices, err := wb.container.GetAllDevices()
	if err != nil {
		panic(err)
	}

	for _, device := range devices {
		wb.container.DeleteDevice(device)
	}

	if wb.client != nil {
		wb.client.Disconnect()
	}

	deviceStore, err := wb.container.GetFirstDevice()
	if err != nil {
		panic(err)
	}

	clientLog := waLog.Stdout("Client", "INFO", true)
	client := whatsmeow.NewClient(deviceStore, clientLog)
	if client.Store.ID == nil {
		qrChan, _ := client.GetQRChannel(context.Background())
		err = client.Connect()
		if err != nil {
			panic(err)
		}
		for evt := range qrChan {
			if evt.Event == "code" {
				wb.client = client
				return evt.Code, CodeGenerated
			}
		}
		return "", Error
	} else {
		return "", SignedIn
	}
}
