package main

import (
	"net/http"
	whatsapp "whatsapp-bot/whatsapp"

	"github.com/gin-gonic/gin"
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
			"error_message": "Malformed reqeust body, must contain message and phone_number",
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
			"error_message": "Not signed in and conencted to WhatsApp",
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
		panic(err)
	}

	// Set global variable
	whatsAppBot = bot
	whatsAppBot.InitializeClient()

	// Initialize routes
	gin.SetMode(gin.ReleaseMode)
	router := gin.Default()
	router.GET("/info", info)
	router.POST("/send_message", sendMessage)
	router.POST("/start_connection", startConnection)
	router.Run("0.0.0.0:5003")
}
