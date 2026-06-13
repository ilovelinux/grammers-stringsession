use crate::version::Version;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use grammers_session::Session;
use grammers_session::types::DcOption;
use std::{error, fmt};

/// Returned by [`restore`] when the encoded session is invalid.
///
/// For example, a header shorter than three bytes or a version byte that is
/// not recognised by this crate.
#[derive(Debug, Clone)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid session string layout")
    }
}

impl error::Error for DecodeError {}

/// Serializes the `session` into a portable, base64-encoded string.
///
/// The returned value can later be passed to [`restore`] to repopulate a fresh
/// session.
///
/// Each string carries a 3-byte header (two reserved bytes plus a one-byte
/// format version), so older strings remain decodable as the format evolves.
///
/// # Examples
///
/// ```
/// use grammers_session::storages::MemorySession;
///
/// let session = MemorySession::default();
///
/// // [...] Authenticate or restore the session...
///
/// let string_session = grammers_stringsession::export(&session).unwrap();
/// ```
///
/// # Errors
///
/// Returns `Err` if session serialization fails.
///
/// # Panics
///
/// Panics if `session.home_dc_id()` does not correspond to a known DC option
/// in `session`. A freshly built [`MemorySession`] has its home DC
/// preconfigured to one of Telegram's well-known data centers, so calling
/// `export` on it is safe. Custom [`Session`] implementations must ensure
/// that `home_dc_id` and `dc_option` agree before exporting.
///
/// [`MemorySession`]: grammers_session::storages::MemorySession
/// [`Session`]: grammers_session::Session
pub fn export<T>(session: &T) -> Result<String, postcard::Error>
where
    T: Session,
{
    let Some(dc_option) = session.dc_option(session.home_dc_id()) else {
        // According to grammer's documentation, dc_option() returns None only if the given DC ID is unknown.
        // Since we're using home_dc_id() as value, we expect it to always return a value.
        unreachable!("Home DC option not found, this should never happen");
    };

    // Serialize the DcOption struct into a byte vector, prepending a 3 bytes header for versioning and future usages.
    // As now, the header consists of 2 bytes of padding and 1 byte for the version number.
    let data = {
        let mut raw_data = vec![0u8, 0u8, Version::default().into()];
        raw_data = postcard::to_extend(&dc_option, raw_data)?;
        raw_data
    };

    let encoded = BASE64_STANDARD.encode(data);

    Ok(encoded)
}

/// Restores a session from a string previously produced by [`export`].
///
/// The session is decoded from `value` and imported into `session`.
///
/// # Examples
///
/// ```
/// use grammers_session::storages::MemorySession;
/// # mod env {
/// #     pub fn var(_: &str) -> Result<String, ()> {
/// #         Ok("AAAApBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYAEMDAwMTAyMDMwNDA1MDYwNzA4MDkwYTBiMGMwZDBlMGYxMDExMTIxMzE0MTUxNjE3MTgxOTFhMWIxYzFkMWUxZjIwMjEyMjIzMjQyNTI2MjcyODI5MmEyYjJjMmQyZTJmMzAzMTMyMzMzNDM1MzYzNzM4MzkzYTNiM2MzZDNlM2Y0MDQxNDI0MzQ0NDU0NjQ3NDg0OTRhNGI0YzRkNGU0ZjUwNTE1MjUzNTQ1NTU2NTc1ODU5NWE1YjVjNWQ1ZTVmNjA2MTYyNjM2NDY1NjY2NzY4Njk2YTZiNmM2ZDZlNmY3MDcxNzI3Mzc0NzU3Njc3Nzg3OTdhN2I3YzdkN2U3ZjgwODE4MjgzODQ4NTg2ODc4ODg5OGE4YjhjOGQ4ZThmOTA5MTkyOTM5NDk1OTY5Nzk4OTk5YTliOWM5ZDllOWZhMGExYTJhM2E0YTVhNmE3YThhOWFhYWJhY2FkYWVhZmIwYjFiMmIzYjRiNWI2YjdiOGI5YmFiYmJjYmRiZWJmYzBjMWMyYzNjNGM1YzZjN2M4YzljYWNiY2NjZGNlY2ZkMGQxZDJkM2Q0ZDVkNmQ3ZDhkOWRhZGJkY2RkZGVkZmUwZTFlMmUzZTRlNWU2ZTdlOGU5ZWFlYmVjZWRlZWVmZjBmMWYyZjNmNGY1ZjZmN2Y4ZjlmYWZiZmNmZGZlZmY=".to_string())
/// #     }
/// # }
///
/// let session = MemorySession::default();
///
/// let string_session = env::var("TG_SESSION").unwrap();
/// # tokio_test::block_on(async {
/// grammers_stringsession::restore(&session, string_session).await.unwrap();
/// # });
/// ```
///
/// # Errors
///
/// Returns `Err` if `value` is not valid base64 (see [`DecodeError`]),
/// or the payload fails to deserialize.
///
/// ```
/// use grammers_session::storages::MemorySession;
///
/// let session = MemorySession::default();
///
/// # tokio_test::block_on(async {
/// grammers_stringsession::restore(&session, "Invalid string session")
///     .await
///     .unwrap_err();
/// # });
/// ```
pub async fn restore<T, U>(session: &U, value: T) -> Result<(), Box<dyn error::Error + Send + Sync>>
where
    T: AsRef<[u8]>,
    U: Session,
{
    let dc_option: DcOption = {
        let data = BASE64_STANDARD.decode(value)?;
        let header = data.get(..3).ok_or(DecodeError)?;
        match Version::try_from(header[2]).map_err(|_| DecodeError)? {
            Version::V1 => postcard::from_bytes(data.get(3..).ok_or(DecodeError)?)?,
        }
    };

    session.set_home_dc_id(dc_option.id).await;
    session.set_dc_option(&dc_option).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
    use std::sync::LazyLock;

    use grammers_session::storages::MemorySession;

    use super::*;

    const VALID_STRINGSESSION_ENCODED: &str = "AAAApBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYAEMDAwMTAyMDMwNDA1MDYwNzA4MDkwYTBiMGMwZDBlMGYxMDExMTIxMzE0MTUxNjE3MTgxOTFhMWIxYzFkMWUxZjIwMjEyMjIzMjQyNTI2MjcyODI5MmEyYjJjMmQyZTJmMzAzMTMyMzMzNDM1MzYzNzM4MzkzYTNiM2MzZDNlM2Y0MDQxNDI0MzQ0NDU0NjQ3NDg0OTRhNGI0YzRkNGU0ZjUwNTE1MjUzNTQ1NTU2NTc1ODU5NWE1YjVjNWQ1ZTVmNjA2MTYyNjM2NDY1NjY2NzY4Njk2YTZiNmM2ZDZlNmY3MDcxNzI3Mzc0NzU3Njc3Nzg3OTdhN2I3YzdkN2U3ZjgwODE4MjgzODQ4NTg2ODc4ODg5OGE4YjhjOGQ4ZThmOTA5MTkyOTM5NDk1OTY5Nzk4OTk5YTliOWM5ZDllOWZhMGExYTJhM2E0YTVhNmE3YThhOWFhYWJhY2FkYWVhZmIwYjFiMmIzYjRiNWI2YjdiOGI5YmFiYmJjYmRiZWJmYzBjMWMyYzNjNGM1YzZjN2M4YzljYWNiY2NjZGNlY2ZkMGQxZDJkM2Q0ZDVkNmQ3ZDhkOWRhZGJkY2RkZGVkZmUwZTFlMmUzZTRlNWU2ZTdlOGU5ZWFlYmVjZWRlZWVmZjBmMWYyZjNmNGY1ZjZmN2Y4ZjlmYWZiZmNmZGZlZmY=";
    static VALID_STRINGSESSION: LazyLock<Vec<u8>> =
        LazyLock::new(|| BASE64_STANDARD.decode(VALID_STRINGSESSION_ENCODED).unwrap());

    #[tokio::test]
    async fn save_and_restore() -> Result<(), Box<dyn error::Error>> {
        let dc_option = DcOption {
            id: 1234,
            ipv4: SocketAddrV4::new(Ipv4Addr::from_bits(0), 0),
            ipv6: SocketAddrV6::new(Ipv6Addr::from_bits(0), 0, 0, 0),
            auth_key: Some(
                (0u8..=255u8)
                    .collect::<Vec<_>>()
                    .as_array::<256>()
                    .unwrap()
                    .to_owned(),
            ),
        };

        let string_session = {
            let session = MemorySession::default();
            session.set_home_dc_id(dc_option.id).await;
            session.set_dc_option(&dc_option).await;
            export(&session)?
        };

        let restored_dc_option = {
            let session = MemorySession::default();
            restore(&session, string_session).await.unwrap();
            session.dc_option(dc_option.id).unwrap()
        };

        assert_eq!(dc_option, restored_dc_option);
        Ok(())
    }

    #[tokio::test]
    async fn restore_invalid_base64_string() {
        let err = restore(&MemorySession::default(), "Invalid BASE64 string")
            .await
            .expect_err("Expected an error when restoring from an invalid BASE64 string");
        err.downcast::<base64::DecodeError>()
            .expect("Expected a DecodeError, got a different error type");
    }

    #[tokio::test]
    async fn restore_invalid_data() {
        let invalid_data = BASE64_STANDARD.encode("Invalid data");
        restore(&MemorySession::default(), invalid_data)
            .await
            .expect_err("Expected an error when restoring from invalid data");
    }

    #[tokio::test]
    async fn restore_invalid_version_data() {
        let invalid_version_data = {
            let mut data = VALID_STRINGSESSION.to_owned();
            data[2] = 0xff;
            BASE64_STANDARD.encode(data)
        };
        let err = restore(&MemorySession::default(), invalid_version_data)
            .await
            .expect_err("Expected an error when restoring from invalid data");
        err.downcast::<DecodeError>()
            .expect("Expected a DecodeError, got a different error type");
    }

    #[tokio::test]
    async fn restore_invalid_struct_data() {
        let invalid_struct_data = {
            let mut data = VALID_STRINGSESSION.to_owned();
            data[3..].reverse();
            BASE64_STANDARD.encode(data)
        };
        let err = restore(&MemorySession::default(), invalid_struct_data)
            .await
            .expect_err("Expected an error when restoring from invalid data");
        err.downcast::<postcard::Error>()
            .expect("Expected a postcard::Error, got a different error type");
    }

    #[tokio::test]
    async fn restore_empty_data() {
        let invalid_struct_data = BASE64_STANDARD.encode([]);
        let err = restore(&MemorySession::default(), invalid_struct_data)
            .await
            .expect_err("Expected an error when restoring from invalid data");
        err.downcast::<DecodeError>()
            .expect("Expected a DecodeError, got a different error type");
    }
}
