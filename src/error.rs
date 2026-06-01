use crate::communication::utils::APIRequest;
use std::error::Error;
use thiserror::Error as TError;

pub type APIResult<T> = Result<T, APIRequestError>;
pub type ServiceStartResult<T> = Result<T, ServiceStartError>;
pub type ServiceRuntimeResult<T> = Result<T, ServiceRuntimeError>;

#[derive(Debug, TError)]
pub enum APIRequestError {
	#[error("there is no result returned in time")]
	Timeout,
	#[error("the request failed with code: {:?}", code)]
	HttpError { code: i32 },
	#[error("deserialize failed")]
	DeserializeError(#[from] serde_json::Error),
	#[error("send request failed")]
	SendError(#[from] flume::SendError<APIRequest>),
	#[error("missing parameters")]
	MissingParameters,
}

#[derive(Debug, TError)]
pub enum ServiceStartError {
	#[error("unknown error")]
	Unknown(Box<dyn Error + Send + Sync>),
	#[error("can not find the event sender")]
	NotInjectedEventSender,
	#[error("can not find the api receiver")]
	NotInjectedAPIReceiver,
	#[error("can not find event sender and api receiver")]
	NotInjected,
	#[error("can not create tcp listener")]
	TcpListenerError(#[from] tokio::io::Error),
	#[cfg(feature = "websocket")]
	#[error("can not create websocket connection")]
	WebSocketConnectError(#[from] tokio_tungstenite::tungstenite::Error),
	#[cfg(feature = "http-post")]
	#[error("invalid secret length for hmac")]
	InvalidSecretLength(#[from] hmac::digest::InvalidLength),
	#[error("task is running")]
	TaskIsRunning,
	#[error("task is not running")]
	TaskIsNotRunning,
}

#[derive(Debug, TError)]
pub enum ServiceRuntimeError {
	#[error("unknown error")]
	Unknown(Box<dyn Error + Send + Sync>),
	#[error("io error")]
	Io(#[from] tokio::io::Error),
	#[error("internal channel closed")]
	ChannelClosed,
	#[error("serialization/deserialization failed")]
	SerializeError(#[from] serde_json::Error),
	#[cfg(feature = "websocket-reverse")]
	#[error("websocket closed by peer")]
	WebSocketClosedByPeer,
	#[cfg(feature = "websocket-reverse")]
	#[error("websocket error")]
	WebSocketError,
	#[cfg(feature = "websocket-reverse")]
	#[error("websocket stream ended")]
	WebSocketStreamEnded,
	#[cfg(feature = "sse")]
	#[error("eventsource ended")]
	EventSourceEnded,
	#[cfg(feature = "http")]
	#[error("url is cannot-be-a-base")]
	UrlCannotBeBase,
	#[cfg(any(feature = "http", feature = "sse"))]
	#[error("http request error")]
	ReqwestError(#[from] reqwest::Error),
}
