use std::fmt;

/// Error types that can occur during browser operations.
#[derive(Debug)]
pub enum BrowserError {
    /// WebView creation or initialization failed
    WebViewCreationFailed(String),
    /// Inter-process communication error
    IpcError(String),
    /// Configuration file or setting error
    ConfigError(String),
    /// URL parsing or validation error
    UrlParseError(String),
    /// File system or I/O error
    IoError(std::io::Error),
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserError::WebViewCreationFailed(msg) => {
                write!(f, "Failed to create WebView: {}", msg)
            }
            BrowserError::IpcError(msg) => write!(f, "IPC communication error: {}", msg),
            BrowserError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            BrowserError::UrlParseError(msg) => write!(f, "URL parse error: {}", msg),
            BrowserError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for BrowserError {}

impl From<std::io::Error> for BrowserError {
    fn from(err: std::io::Error) -> Self {
        BrowserError::IoError(err)
    }
}

impl From<serde_json::Error> for BrowserError {
    fn from(err: serde_json::Error) -> Self {
        BrowserError::IpcError(err.to_string())
    }
}

impl From<serde_yaml::Error> for BrowserError {
    fn from(err: serde_yaml::Error) -> Self {
        BrowserError::ConfigError(err.to_string())
    }
}

impl From<url::ParseError> for BrowserError {
    fn from(err: url::ParseError) -> Self {
        BrowserError::UrlParseError(err.to_string())
    }
}

impl From<wry::Error> for BrowserError {
    fn from(err: wry::Error) -> Self {
        BrowserError::WebViewCreationFailed(err.to_string())
    }
}
