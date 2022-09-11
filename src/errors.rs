use std::io::ErrorKind;

pub type OzesResult<T> = Result<T, OzesConnectorError>;

#[derive(Debug)]
pub enum OzesConnectorError {
    TimeOut,
    WithouConnection,
    ErrorResponse,
    UnknownError(String),
    ToLongMessage,
    AddrInUse,
    PermissionDenied,
    Refused,
    Reseted,
    InvalidMessageToServer(Vec<u8>),
}

impl From<std::io::Error> for OzesConnectorError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::BrokenPipe => Self::WithouConnection,
            ErrorKind::TimedOut => Self::TimeOut,
            ErrorKind::AddrInUse => Self::AddrInUse,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            ErrorKind::ConnectionReset => Self::Reseted,
            ErrorKind::ConnectionRefused => Self::Refused,
            _ => Self::UnknownError(e.to_string()),
        }
    }
}

impl std::fmt::Display for OzesConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "foo")
    }
}

impl std::error::Error for OzesConnectorError {}
