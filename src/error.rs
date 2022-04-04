#[derive(Debug)]
pub enum PoolWatcherError {
    WebSocket(websocket::WebSocketError),
    Parse(websocket::client::ParseError),
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

impl From<websocket::WebSocketError> for PoolWatcherError {
    fn from(err: websocket::WebSocketError) -> Self {
        PoolWatcherError::WebSocket(err)
    }
}
impl From<websocket::client::ParseError> for PoolWatcherError {
    fn from(err: websocket::client::ParseError) -> Self {
        PoolWatcherError::Parse(err)
    }
}
impl From<std::io::Error> for PoolWatcherError {
    fn from(err: std::io::Error) -> Self {
        PoolWatcherError::IO(err)
    }
}

impl From<reqwest::Error> for PoolWatcherError {
    fn from(err: reqwest::Error) -> Self {
        PoolWatcherError::Reqwest(err)
    }
}
impl From<serde_json::Error> for PoolWatcherError {
    fn from(err: serde_json::Error) -> Self {
        PoolWatcherError::Json(err)
    }
}
