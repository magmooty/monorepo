# Magmooty WhatsApp Bot

This service exposes an API that allows a user to connect private WhatsApp account and send messages programmatically through API calls.

## Building

### Building for Linux

Install MUSL cross-compiler

```shell
brew install FiloSottile/musl-cross/musl-cross
```

### Building for Windows

Install mingw-w64

```shell
brew install mingw-w64
```

Run build script

```
sh build.sh <arch>
```

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
