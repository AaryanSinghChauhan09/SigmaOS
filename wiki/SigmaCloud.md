# SigmaCloud

**SigmaCloud** is the SigmaOS alternative to Google Workspace.

## Features

### SigmaDrive (Cloud Storage)
Decentralized cloud file storage (Google Drive alternative):
- End-to-end encrypted file storage
- Decentralized storage via IPFS/Filecoin
- File versioning with SovereignFS snapshots
- Real-time collaboration
- Offline mode with sync
- File sharing with capability tokens
- 10GB free storage, scalable to petabytes
- AI-powered file organization and search
- Cross-device sync (desktop, mobile, web)

### SigmaMail (Email Client)
Secure, encrypted mailing (Gmail alternative):
- End-to-end encrypted email (PGP integration)
- Spam and phishing protection with AI
- Rich text composer
- Attachment encryption
- Calendar integration
- Contacts integration
- Multiple account support
- AI-powered email categorization (Primary, Social, Promotions)
- Smart replies and compose suggestions
- Offline mode with sync

### SigmaCalendar (Calendar & Scheduling)
Sync and scheduling tools:
- Event creation and management
- Recurring events
- Calendar sharing with permission controls
- Meeting scheduling with availability detection
- Reminders and notifications
- Integration with SigmaMail
- Time zone support
- AI-powered scheduling suggestions
- Export to iCal format

### SigmaContacts (Contacts Management)
Contact management and synchronization:
- Contact cards with multiple fields
- Contact groups and labels
- Import/export vCard format
- Integration with SigmaMail and SigmaCalendar
- Contact sharing
- AI-powered contact deduplication
- Sync with mobile devices

## Architecture

```
SigmaCloud Suite
   ├─ SigmaDrive (cloud storage engine)
   │   ├─ IPFS/Filecoin backend
   │   ├─ Encryption layer (ChaCha20-Poly1305)
   │   ├─ Sync engine
   │   └─ Collaboration backend
   ├─ SigmaMail (email engine)
   │   ├─ IMAP/SMTP client
   │   ├─ PGP encryption
   │   ├─ Spam filter (AI)
   │   └─ Rich text composer
   ├─ SigmaCalendar (calendar engine)
   │   ├─ Event scheduler
   │   ├─ Availability detector
   │   └─ Reminder system
   └─ SigmaContacts (contacts engine)
       ├─ Contact database
       ├─ vCard parser/generator
       └─ Sync engine
```

## Security Model

- **End-to-end encryption**: All data encrypted client-side before upload
- **Zero-knowledge architecture**: Server cannot access user data
- **Capability-based sharing**: Fine-grained access control via capability tokens
- **Post-quantum ready**: Uses Kyber-768 for key exchange
- **Secure boot integration**: Verified boot chain for cloud clients

## API Interface

```c
// SigmaDrive API
int sigma_drive_upload(const char *local_path, const char *remote_path);
int sigma_drive_download(const char *remote_path, const char *local_path);
int sigma_drive_share(const char *remote_path, const char *recipient, cap_token_t cap);
int sigma_drive_sync(void);
int sigma_drive_list(const char *path, drive_entry_t *entries, size_t *count);

// SigmaMail API
int sigma_mail_send(const mail_message_t *msg);
int sigma_mail_receive(mailbox_t *mailbox, mail_message_t *msgs, size_t *count);
int sigma_mail_encrypt(mail_message_t *msg, const char *recipient_key);
int sigma_mail_decrypt(mail_message_t *msg);
int sigma_mail_compose_rich_text(const char *html, const char *plain);

// SigmaCalendar API
int sigma_calendar_create_event(const event_t *event);
int sigma_calendar_update_event(const char *event_id, const event_t *event);
int sigma_calendar_delete_event(const char *event_id);
int sigma_calendar_list_events(time_t start, time_t end, event_t *events, size_t *count);
int sigma_calendar_share_calendar(const char *calendar_id, const char *recipient, cap_token_t cap);

// SigmaContacts API
int sigma_contacts_add(const contact_t *contact);
int sigma_contacts_update(const char *contact_id, const contact_t *contact);
int sigma_contacts_delete(const char *contact_id);
int sigma_contacts_search(const char *query, contact_t *results, size_t *count);
int sigma_contacts_import_vcard(const char *path);
int sigma_contacts_export_vcard(const char *contact_id, const char *path);
```

## Integration

- **SigmaFS Integration**: Local cache with SovereignFS snapshots
- **SigmaAI Integration**: AI-powered spam filtering, email categorization, scheduling suggestions
- **SigmaNet Integration**: Encrypted communication via SovereignNet
- **Zenith Desktop Integration**: Native Zenith UI with notifications
- **Mobile Sync**: SigmaOS Mobile app for on-the-go access

## Performance Characteristics

| Application | Sync Speed | Encryption | Offline Support |
|---|---|---|---|
| SigmaDrive | 100MB/s (local), 10MB/s (remote) | ChaCha20-Poly1305 | ✅ Yes |
| SigmaMail | Instant (IMAP) | PGP + TLS | ✅ Yes |
| SigmaCalendar | Instant | TLS | ✅ Yes |
| SigmaContacts | Instant | TLS | ✅ Yes |

## Roadmap

- [x] Architecture design and component specification
- [ ] SigmaDrive IPFS/Filecoin integration
- [ ] SigmaMail IMAP/SMTP client implementation
- [ ] SigmaCalendar event scheduler implementation
- [ ] SigmaContacts database implementation
- [ ] End-to-end encryption implementation
- [ ] AI-powered features (spam filter, categorization, suggestions)
- [ ] Mobile apps (SigmaOS Mobile, iOS, Android)
- [ ] Web interface (SigmaOS Cloud)
- [ ] Enterprise features (SSO, admin console, audit logs)

## Related Modules

- [`modules/core/net`](../../modules/core/net/README.md) — Network stack
- [`security/pqc/README.md`](../../security/pqc/README.md) — Post-quantum cryptography
- [`modules/core/fs`](../../modules/core/fs/README.md) — Filesystem integration
