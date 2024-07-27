package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
	whatsapp "whatsapp-bot/whatsapp"
)

var whatsappBot *whatsapp.WhatsAppBot

func info(c *gin.Context) {
	signed_in := whatsappBot.IsSignedIn()
	c.JSON(http.StatusOK, gin.H{
		"signed_in": signed_in,
	})
}

func sendMessage(c *gin.Context) {
	if !whatsappBot.IsSignedIn() {
		c.JSON(http.StatusBadRequest, gin.H{
			"signed_in": false,
			"sent":      false,
		})
		return
	}

	var requestBody struct {
		Message     string `json:"message"`
		PhoneNumber string `json:"phone_number"`
	}

	if err := c.ShouldBindJSON(&requestBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"signed_in": true,
			"sent":      false,
		})
		return
	}

	sent := whatsappBot.SendMessage(requestBody.PhoneNumber, requestBody.Message)
	c.JSON(http.StatusOK, gin.H{
		"signed_in": true,
		"sent":      sent,
	})
}

func startConnection(c *gin.Context) {
	code, status := whatsappBot.StartConnect()
	switch status {
	case whatsapp.CodeGenerated:
		c.JSON(http.StatusOK, gin.H{
			"code":      code,
			"signed_in": false,
			"error":     false,
		})
	case whatsapp.SignedIn:
		c.JSON(http.StatusOK, gin.H{
			"code":      nil,
			"signed_in": true,
			"error":     false,
		})
	case whatsapp.Error:
		c.JSON(http.StatusBadRequest, gin.H{
			"code":      nil,
			"signed_in": false,
			"error":     true,
		})
	}
}

func main() {
	bot, err := whatsapp.New()
	if err != nil {
		panic(err)
	}

	// Set global variable
	whatsappBot = bot
	whatsappBot.InitializeClient()

	// Initialize routes
	router := gin.Default()
	router.GET("/info", info)
	router.POST("/send_message", sendMessage)
	router.POST("/start_connection", startConnection)
	router.Run("localhost:5003")
}
