# TechScript WebForge: REST API Specification
**Declarative Endpoints, Request Validation, and JSON Protocols**

---

## 1. Overview

WebForge automatically creates structured, typed REST API endpoints from your TechScript database declarations. This enables seamless, zero-boilerplate frontend-to-backend communication without manual endpoint controllers or SQL query wiring.

---

## 2. Automatic Endpoint Derivation

When a table is defined in TechScript:

```techscript
database Blog:
    table posts:
        title is text
        content is text
```

WebForge automatically binds two standard REST routes:

### 1. Retrieve Records (`GET /api/posts`)
- **Method:** `GET`
- **Request Headers:** `Accept: application/json`
- **Response Status:** `200 OK`
- **Response Body:**
  ```json
  [
    {
      "id": 1,
      "title": "Introducing WebForge",
      "content": "Full-stack development made intuitive."
    }
  ]
  ```

### 2. Create Record (`POST /api/posts`)
- **Method:** `POST`
- **Request Headers:** `Content-Type: application/json`
- **Request Body:**
  ```json
  {
    "title": "New Post",
    "content": "This is content."
  }
  ```
- **Response Status:** `200 OK`
- **Response Body:**
  ```json
  {
    "status": "ok",
    "id": 2
  }
  ```

---

## 3. Client Interaction via TechScript

In your frontend code, you can trigger REST API calls directly without writing `fetch()` or `axios`:

```techscript
when SavePostBtn is clicked:
    send PostContentInput to /api/posts
    show message "Post created successfully!"
```

Under the hood, WebForge generates an asynchronous `fetch()` payload, captures the input value, executes the `POST` request, handles network exceptions, and displays the reactive toast notification.

---

## 4. Testing Endpoints via CLI

You can test WebForge endpoints with standard tools:

### Using cURL
```bash
# Query notes
curl -X GET http://127.0.0.1:3000/api/notes

# Insert a note
curl -X POST http://127.0.0.1:3000/api/notes \
     -H "Content-Type: application/json" \
     -d '{"content": "Automated test note"}'
```

### Using PowerShell
```powershell
Invoke-RestMethod -Uri "http://127.0.0.1:3000/api/notes" -Method Get
```
