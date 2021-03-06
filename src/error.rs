use failure::Fail;
use std::io;

#[derive(Debug, Clone, Fail)]
pub enum ProtocolError {
    #[fail(display = "disconnect")]
    Disconnect,

    #[fail(display = "forced disconnect")]
    DisconnectByMe,

    #[fail(display = "invalid handshake")]
    InvalidHandshake,

    #[fail(display = "missing handshake")]
    MissingHandshake,

    #[fail(display = "handshake timeout")]
    HandshakeTimeout,
}

impl ProtocolError {
    pub fn into_err(&self) -> Error {
        Error::ProtocolError(self.clone())
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "invalid format: {}", _0)]
    InvalidJsonFormat(#[cause] serde_json::Error),
    #[fail(display = "invalid format: {}", _0)]
    InvalidBinFormat(#[cause] bincode::Error),
    #[fail(display = "invalid matadata version: {}", detected_version)]
    InvalidMetaVersion { detected_version: u32 },
    #[fail(display = "matadata not found")]
    MetadataNotFound,
    #[fail(display = "{} not working", _0)]
    ServiceFail(&'static str),
    #[fail(display = "{}", _0)]
    Mailbox(actix::MailboxError),
    #[fail(display = "request canceled {}", _0)]
    RequestCanceled(#[cause] futures::Canceled),
    #[fail(display = "resource {:032x} not found", _0)]
    ResourceNotFound(u128),
    #[fail(display = "invalid block hash {:032x}", _0)]
    InvalidBlockHash(u128),
    #[fail(display = "{}", _0)]
    ProtocolError(#[cause] ProtocolError),
}

macro_rules! convert {
    {
        $($t:path => $opt:ident),*
    } => {
        $(impl From<$t> for Error {
            fn from(e : $t) -> Self {
                Error::$opt(e)
            }
        })*
    };
}

convert! {
    io::Error => IO,
    bincode::Error => InvalidBinFormat,
    serde_json::Error => InvalidJsonFormat,
    actix::MailboxError => Mailbox,
    futures::Canceled => RequestCanceled,
    ProtocolError => ProtocolError
}
