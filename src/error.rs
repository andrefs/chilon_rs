use std::fmt;

#[derive(Debug)]
pub enum ChilonError {
    Io(std::io::Error),
    Parse(String),
    InvalidInput(String),
    ChannelSend(String),
}

impl fmt::Display for ChilonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChilonError::Io(e) => write!(f, "I/O error: {}", e),
            ChilonError::Parse(msg) => write!(f, "parse error: {}", msg),
            ChilonError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            ChilonError::ChannelSend(msg) => write!(f, "channel send failed: {}", msg),
        }
    }
}

impl std::error::Error for ChilonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChilonError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ChilonError {
    fn from(e: std::io::Error) -> Self {
        ChilonError::Io(e)
    }
}

impl From<rayon::ThreadPoolBuildError> for ChilonError {
    fn from(e: rayon::ThreadPoolBuildError) -> Self {
        ChilonError::Io(std::io::Error::other(e))
    }
}

impl From<oxttl::TurtleParseError> for ChilonError {
    fn from(e: oxttl::TurtleParseError) -> Self {
        ChilonError::Parse(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn invalid_input_display_is_non_empty() {
        let err = ChilonError::InvalidInput("n_workers < 2".into());
        assert!(!err.to_string().is_empty());
        assert_eq!(err.to_string(), "invalid input: n_workers < 2");
    }

    #[test]
    fn io_error_converts_via_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
        let converted: ChilonError = io_err.into();
        assert!(matches!(converted, ChilonError::Io(_)));
    }

    #[test]
    fn io_error_exposes_source() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = ChilonError::Io(io_err);
        assert!(err.source().is_some());
        assert_eq!(err.source().unwrap().to_string(), "denied");
    }
}
