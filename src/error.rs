use crate::communication::utils::APIRequest;
use std::error::Error;
use thiserror::Error as TError;

pub type APIResult<T> = Result<T, APIRequestError>;
pub type ServiceStartResult<T> = Result<T, ServiceStartError>;

#[derive(Debug, TError)]
pub enum APIRequestError {
	#[error("There is no result returned in time")]
	Timeout,
	#[error("The request failed with code: {:?}", code)]
	HttpError { code: i32 },
	#[error("Deserialize failed")]
	DeserializeError(#[from] serde_json::Error),
	#[error("Send request failed")]
	SendError(#[from] flume::SendError<APIRequest>),
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
}

#[derive(Debug, TError)]
pub enum ServiceRuntimeError {
	#[error("unknown error")]
	Unknown(Box<dyn Error + Send + Sync>),
}
