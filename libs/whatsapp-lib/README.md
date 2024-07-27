# Magmooty WhatsApp Bot

This service exposes an API that allows a user to connect private WhatsApp account and send messages programmatically through API calls.

## Building

Install mingw-w64

```shell
brew install mingw-w64
```

Run build script

```
sh build.sh
```

It should output a Windows 32-bit executable in the `dist` folder.

## API documentation

### Check if a user is signed in

**Request**

```
GET /info
```

**Output**

```json
{
  "signed_in": false
}
```

### Connect a user with QR code

**Request**

```
POST /start_connection
```

**Output**

```json
{
  "code": "2@fs64...",
  "signed_in": false,
  "error": false
}
```

### Send a message

**Request**

```json
POST /send_message
{
  "message": "...",
  "phone_number": "+20109..."
}
```

**Response**

```json
{
  "signed_in": true,
  "sent": true
}
```
