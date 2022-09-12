use std::io::Read;

use errors::{OzesConnectorError, OzesResult};

pub mod errors;
//#[cfg(feature = "publisher")]
pub mod publisher;
//#[cfg(feature = "consumer")]
pub mod consumer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

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
