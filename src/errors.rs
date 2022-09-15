use std::io::ErrorKind;

pub type OzesResult<T> = Result<T, OzesConnectorError>;

#[derive(Debug, PartialEq, Eq)]
pub enum OzesConnectorError {
    TimeOut,
    WithouConnection,
    UnknownError(String),
    PermissionDenied,
    Refused,
    Reseted,
    InvalidLen(usize),
    IncompatibleCommand,
    FailToParseServerMessage(Vec<u8>),
    InvalidMessageToServer(Vec<u8>),
}

impl From<std::io::Error> for OzesConnectorError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::BrokenPipe => Self::WithouConnection,
            ErrorKind::TimedOut => Self::TimeOut,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            ErrorKind::ConnectionReset => Self::Reseted,
            ErrorKind::ConnectionRefused => Self::Refused,
            _ => Self::UnknownError(e.to_string()),
        }
    }
}

impl std::fmt::Display for OzesConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::InvalidMessageToServer(msg) => format!(
                "invalide message {}",
                String::from_utf8(msg.clone()).unwrap()
            ),
            Self::FailToParseServerMessage(msg) => format!(
                "fail to parse server message {}",
                String::from_utf8(msg.clone()).unwrap()
            ),
            Self::PermissionDenied => "permission denied".to_owned(),
            Self::Refused => "connection refused".to_owned(),
            Self::Reseted => "connection reset".to_owned(),
            Self::TimeOut => "connection time out".to_owned(),
            Self::WithouConnection => "lose connection".to_owned(),
            Self::InvalidLen(len) => format!("invalid len of message {}", len),
            Self::UnknownError(error) => format!("unknown error {}", error),
            Self::IncompatibleCommand => "incompatible command are received".to_owned(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for OzesConnectorError {}
