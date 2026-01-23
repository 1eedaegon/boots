# API Documentation

## Base URL

- Development: `http://localhost:8080/api`
- Production: `https://your-domain.com/api`

## Authentication

All API requests require `Authorization` header with Bearer token:

```
Authorization: Bearer <token>
```

### Login

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "writer@example.com",
  "password": "writer123"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "uuid",
    "email": "writer@example.com",
    "name": "Writer",
    "role": "writer"
  }
}
```

---

## Posts API

### List Posts

```http
GET /api/posts?page=1&limit=10
```

### Get Post

```http
GET /api/posts/:id
```

### Create Post (Writer, Admin)

```http
POST /api/posts
Content-Type: application/json

{
  "title": "Title",
  "content": "Content",
  "file_keys": ["uploads/xxx.png"]
}
```

### Update Post (Author, Admin)

```http
PUT /api/posts/:id
Content-Type: application/json

{
  "title": "Updated Title",
  "content": "Updated Content"
}
```

### Delete Post (Author, Admin)

```http
DELETE /api/posts/:id
```

---

## Comments API

### List Comments

```http
GET /api/posts/:postId/comments
```

### Create Comment (All roles)

```http
POST /api/posts/:postId/comments
Content-Type: application/json

{
  "content": "Comment content"
}
```

### Update Comment (Reader Author, Admin)

```http
PUT /api/comments/:id
Content-Type: application/json

{
  "content": "Updated comment"
}
```

### Delete Comment (Reader Author, Admin)

```http
DELETE /api/comments/:id
```

---

## Files API

### Get Presigned Upload URL

```http
POST /api/files/presign
Content-Type: application/json

{
  "filename": "image.png",
  "content_type": "image/png"
}
```

**Response:**
```json
{
  "upload_url": "https://s3.../presigned-url",
  "file_key": "uploads/uuid-image.png",
  "required_headers": {
    "Content-Type": "image/png"
  }
}
```

### Get Download URL

```http
GET /api/files/:key/download
```

---

## Error Responses

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Permission denied"
  }
}
```

| Code | HTTP Status | Description |
|------|-------------|-------------|
| UNAUTHORIZED | 401 | Authentication required |
| FORBIDDEN | 403 | Permission denied |
| NOT_FOUND | 404 | Resource not found |
| VALIDATION_ERROR | 400 | Invalid input |
