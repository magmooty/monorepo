package main

import (
	"github.com/containers/winquit/pkg/winquit"
	"github.com/gin-gonic/gin"
	"net"
	"net/http"
	"os"
	"os/signal"
	"runtime"
	"syscall"
	whatsapp "whatsapp-bot/whatsapp"
)

var whatsAppBot *whatsapp.WhatsAppBot

func info(c *gin.Context) {
	status, err := whatsAppBot.IsSignedIn()

	switch status {
	case whatsapp.WhatsAppLibraryError:
		c.JSON(http.StatusInternalServerError, gin.H{
			"status":        status,
			"error_message": err,
		})
	default:
		c.JSON(http.StatusOK, gin.H{
			"status": status,
		})
	}
}

func sendMessage(c *gin.Context) {
	var requestBody struct {
		Message     string `json:"message"`
		PhoneNumber string `json:"phone_number"`
	}

	if err := c.ShouldBindJSON(&requestBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error_message": "Malformed request body, must contain message and phone_number",
		})
		return
	}

	status, err := whatsAppBot.SendMessage(requestBody.PhoneNumber, requestBody.Message)

	switch status {
	case whatsapp.WhatsAppLibraryError:
		c.JSON(http.StatusInternalServerError, gin.H{
			"status":        status,
			"error_message": err,
		})
	case whatsapp.NotConnected:
		c.JSON(http.StatusInternalServerError, gin.H{
			"status":        status,
			"error_message": "Not connected to WhatsApp",
		})
	case whatsapp.SignedOut:
		c.JSON(http.StatusBadRequest, gin.H{
			"status":        status,
			"error_message": "Not signed in and connected to WhatsApp",
		})
	case whatsapp.TargetNotOnWhatsApp:
		c.JSON(http.StatusConflict, gin.H{
			"status":        status,
			"error_message": "Target is not using WhatsApp",
		})
	case whatsapp.MessageSent:
		c.JSON(http.StatusCreated, gin.H{
			"status": status,
		})
	}
}

func startConnection(c *gin.Context) {
	code, status, err := whatsAppBot.StartConnect()

	switch status {
	case whatsapp.QRCodeGenerated:
		c.JSON(http.StatusAccepted, gin.H{
			"status": status,
			"code":   code,
		})
	case whatsapp.SignedIn:
		c.JSON(http.StatusInternalServerError, gin.H{
			"status":        status,
			"error_message": "Already signed in, failed to sign out existing account",
		})
	case whatsapp.WhatsAppLibraryError:
		c.JSON(http.StatusInternalServerError, gin.H{
			"status":        status,
			"error_message": err,
		})
	}
}

func main() {
	bot, err := whatsapp.New("INFO")

	if err != nil {
		println("[ERROR] Failed to create WhatsApp bot: %s", err.Error())
	}

	// Set global variable
	whatsAppBot = bot

	status, err := whatsAppBot.InitializeClient()

	if status == whatsapp.WhatsAppLibraryError && err != nil {
		println("[ERROR] Failed to create WhatsApp bot: %s", err.Error())
	}

	// Handle graceful exit
	go func() {
		if runtime.GOOS == "windows" {
			winQuit := make(chan bool, 1)
			winquit.NotifyOnQuit(winQuit)
			<-winQuit
		} else {
			quit := make(chan os.Signal, 1)
			signal.Notify(quit, os.Interrupt, syscall.SIGTERM, syscall.SIGINT)
			<-quit
		}

		println("Closing WhatsApp database file")
		whatsAppBot.Shutdown()

		os.Exit(0)
	}()

	// Initialize routes
	gin.SetMode(gin.ReleaseMode)
	router := gin.Default()
	router.GET("/info", info)
	router.POST("/send_message", sendMessage)
	router.POST("/start_connection", startConnection)
	println("Starting HTTP server on port 5003...")

	listener, err := net.Listen("tcp", "0.0.0.0:5003")

	if err != nil {
		println("[ERROR] Failed to start HTTP server on port 5003: %v", err)
		os.Exit(1)
	} else {
		println("Started web server on port 5003")
	}

	err = router.RunListener(listener)

	if err != nil {
		println("[ERROR] Failed to start HTTP server on port 5003: %v", err)
	}
}
