# Architecture

## Layer Structure

```
┌─────────────────────────────────────────┐
│            Frontend (React)             │
├─────────────────────────────────────────┤
│              API (Axum)                 │
│  ┌─────────────────────────────────┐   │
│  │  Routes → Handlers → Services    │   │
│  └─────────────────────────────────┘   │
├─────────────────────────────────────────┤
│              Core (Domain)              │
│  ┌─────────────────────────────────┐   │
│  │  Models, Permissions, Validation │   │
│  └─────────────────────────────────┘   │
├─────────────────────────────────────────┤
│           Persistence (SQLx)            │
│  ┌─────────────────────────────────┐   │
│  │   Repositories, Migrations       │   │
│  └─────────────────────────────────┘   │
├─────────────────────────────────────────┤
│       Infrastructure (S3, etc.)         │
└─────────────────────────────────────────┘
```

## Permission Flow

```
Request
   │
   ▼
┌──────────────┐
│ Auth Guard   │ ─── Token invalid ───► 401 Unauthorized
└──────────────┘
   │
   ▼ (Extract User + Role)
┌──────────────┐
│ Permission   │ ─── No permission ───► 403 Forbidden
│ Middleware   │
└──────────────┘
   │
   ▼
┌──────────────┐
│ Handler      │ ─► Business logic
└──────────────┘
```

## File Upload Flow

```
1. Client: POST /api/files/presign
   │
   ▼
2. Server: Generate presigned URL & return
   │
   ▼
3. Client: PUT presigned_url (direct upload to S3)
   │
   ▼
4. Client: POST /api/posts (with file_keys)
   │
   ▼
5. Server: Save file metadata
```

## Data Model

```
User ─────────┬────────── Post
  │           │             │
  │ author_id │             │ post_id
  │           │             │
  └───────────┼─────────────┤
              │             │
              ▼             ▼
           Comment ◄─────── FileAttachment
```

## Permission Matrix

| Resource | Action | Admin | Writer | Reader |
|----------|--------|-------|--------|--------|
| Post | View | ✓ | ✓ | ✓ |
| Post | Create | ✓ | ✓ | ✗ |
| Post | Edit | ✓ | Own only | ✗ |
| Post | Delete | ✓ | Own only | ✗ |
| Comment | View | ✓ | ✓ | ✓ |
| Comment | Create | ✓ | ✓ | ✓ |
| Comment | Edit | ✓ | ✗ | Own only |
| Comment | Delete | ✓ | ✗ | Own only |
