# grammers-stringsession

Export and restore a [grammers session] as a portable, versioned base64 string.

Inspired by Telethon's `StringSession`.

## Install

```sh
cargo add grammers-stringsession
```

## Compatibility matrix

|`feature`|gramme.rs version|
|:---|:---|
|`grammers-session-0.9.0`|`~0.9.0`|
|`grammers-session` (default)|`~0.10.0`|

## Example

```rust
# #[cfg(feature = "grammers-session-0.9.0")]
# extern crate grammers_session_0_9_0 as grammers_session;
# #[cfg(feature = "grammers-session-git")]
# extern crate grammers_session_git as grammers_session;
use grammers_session::storages::MemorySession;
use grammers_stringsession::{export, restore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let session = MemorySession::default();
    // ...authenticate or restore the session as usual...

    let string_session = export(&session)?;

    let restored = MemorySession::default();
    restore(&restored, string_session).await?;

    Ok(())
}
```

See the [`crate::export`] and [`crate::restore`] documentation for more details.

## Format

Each session string is base64-encoded with a 3-byte header (two reserved bytes
plus a one-byte format version), so existing strings stay decodable as the
format evolves.

## License

Licensed under either MIT or Apache-2.0 at your option.

[grammers session]: https://docs.rs/grammers-session
