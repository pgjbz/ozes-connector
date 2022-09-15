use std::io::Read;

use errors::{OzesConnectorError, OzesResult};

#[cfg(feature = "consumer")]
pub mod consumer;
pub mod errors;
#[cfg(feature = "publisher")]
pub mod publisher;

const BASE_MESSAGE_LEN: usize = "message +l #".len();

pub(crate) fn number_len(number: usize) -> usize {
    number.to_string().len()
}

#[allow(dead_code)]
pub(crate) fn unwrap_return<T>(stream: &mut T) -> OzesResult<()>
where
    T: Read,
{
    let mut buffer = vec![0; 4096];
    match stream.read(&mut buffer) {
        Ok(n) => {
            buffer.truncate(n);
            if !buffer.starts_with(b"ok") {
                return Err(OzesConnectorError::InvalidMessageToServer(buffer));
            }
            Ok(())
        }
        Err(e) => {
            buffer.extend_from_slice(e.to_string().as_bytes());
            Err(OzesConnectorError::InvalidMessageToServer(buffer))
        }
    }
}
