Serializes the `session` into a portable, base64-encoded string.

The returned value can later be passed to [`restore`] to repopulate a fresh
session.

Each string carries a 3-byte header (two reserved bytes plus a one-byte
format version), so older strings remain decodable as the format evolves.

# Examples

```
# #[cfg(feature = "grammers-session-0.9.0")]
# extern crate grammers_session_0_9_0 as grammers_session;
# #[cfg(feature = "grammers-session-git")]
# extern crate grammers_session_git as grammers_session;
use grammers_session::storages::MemorySession;

let session = MemorySession::default();

// [...] Authenticate or restore the session...

let string_session = grammers_stringsession::export(&session).unwrap();
```

# Errors

Returns `Err` if session serialization fails.

# Panics

Panics if `session.home_dc_id()` does not correspond to a known DC option
in `session`. A freshly built [`MemorySession`] has its home DC
preconfigured to one of Telegram's well-known data centers, so calling
`export` on it is safe. Custom [`Session`] implementations must ensure
that `home_dc_id` and `dc_option` agree before exporting.

[`MemorySession`]: grammers_session::storages::MemorySession
[`Session`]: grammers_session::Session