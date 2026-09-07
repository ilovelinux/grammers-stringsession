Restores a session from a string previously produced by [`export`].

The session is decoded from `value` and imported into `session`.

# Examples

```
# #[cfg(feature = "grammers-session-0.9.0")]
# extern crate grammers_session_0_9_0 as grammers_session;
# #[cfg(feature = "grammers-session-git")]
# extern crate grammers_session_git as grammers_session;
use grammers_session::storages::MemorySession;
# mod env {
#     pub fn var(_: &str) -> Result<String, ()> {
#         Ok("AAAApBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYAEMDAwMTAyMDMwNDA1MDYwNzA4MDkwYTBiMGMwZDBlMGYxMDExMTIxMzE0MTUxNjE3MTgxOTFhMWIxYzFkMWUxZjIwMjEyMjIzMjQyNTI2MjcyODI5MmEyYjJjMmQyZTJmMzAzMTMyMzMzNDM1MzYzNzM4MzkzYTNiM2MzZDNlM2Y0MDQxNDI0MzQ0NDU0NjQ3NDg0OTRhNGI0YzRkNGU0ZjUwNTE1MjUzNTQ1NTU2NTc1ODU5NWE1YjVjNWQ1ZTVmNjA2MTYyNjM2NDY1NjY2NzY4Njk2YTZiNmM2ZDZlNmY3MDcxNzI3Mzc0NzU3Njc3Nzg3OTdhN2I3YzdkN2U3ZjgwODE4MjgzODQ4NTg2ODc4ODg5OGE4YjhjOGQ4ZThmOTA5MTkyOTM5NDk1OTY5Nzk4OTk5YTliOWM5ZDllOWZhMGExYTJhM2E0YTVhNmE3YThhOWFhYWJhY2FkYWVhZmIwYjFiMmIzYjRiNWI2YjdiOGI5YmFiYmJjYmRiZWJmYzBjMWMyYzNjNGM1YzZjN2M4YzljYWNiY2NjZGNlY2ZkMGQxZDJkM2Q0ZDVkNmQ3ZDhkOWRhZGJkY2RkZGVkZmUwZTFlMmUzZTRlNWU2ZTdlOGU5ZWFlYmVjZWRlZWVmZjBmMWYyZjNmNGY1ZjZmN2Y4ZjlmYWZiZmNmZGZlZmY=".to_string())
#     }
# }

let session = MemorySession::default();

let string_session = env::var("TG_SESSION").unwrap();
# tokio_test::block_on(async {
grammers_stringsession::restore(&session, string_session).await.unwrap();
# });
```

# Errors

Returns `Err` if `value` is not valid base64 (see [`DecodeError`]),
or the payload fails to deserialize.

```
# #[cfg(feature = "grammers-session-0.9.0")]
# extern crate grammers_session_0_9_0 as grammers_session;
# #[cfg(feature = "grammers-session-git")]
# extern crate grammers_session_git as grammers_session;
use grammers_session::storages::MemorySession;

let session = MemorySession::default();

# tokio_test::block_on(async {
grammers_stringsession::restore(&session, "Invalid string session")
    .await
    .unwrap_err();
# });
```