use std::result;
use std::io;
use std::fmt;
use std::error;
use std::num;

use sincere;
use mongors;
use mongors::bson::doc;
use mongors::bson::encode::EncodeError;
use mongors::bson::decode::DecodeError;
use mongors::object_id;
use sincere_token;
use reqwest;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Sincere(sincere::Error),
    CodedError(ErrorCode),
    MongoError(mongors::error::Error),
    TokenError(sincere_token::Error),
    DocError(doc::Error),
    BsonEncodeError(EncodeError),
    BsonDecodeError(DecodeError),
    ParseIntError(num::ParseIntError),
    ObjectIdError(object_id::Error),
    RewqestError(reqwest::Error),
    Message(String)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<sincere::Error> for Error {
    fn from(err: sincere::Error) -> Error {
        Error::Sincere(err)
    }
}

impl From<mongors::error::Error> for Error {
    fn from(err: mongors::error::Error) -> Error {
        Error::MongoError(err)
    }
}

impl From<ErrorCode> for Error {
    fn from(err: ErrorCode) -> Error {
        Error::CodedError(err)
    }
}

impl From<sincere_token::Error> for Error {
    fn from(err: sincere_token::Error) -> Error {
        Error::TokenError(err)
    }
}

impl From<doc::Error> for Error {
    fn from(err: doc::Error) -> Error {
        Error::DocError(err)
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Error {
        Error::BsonEncodeError(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Error {
        Error::BsonDecodeError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

impl From<object_id::Error> for Error {
    fn from(err: object_id::Error) -> Error {
        Error::ObjectIdError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RewqestError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::Sincere(ref inner) => inner.fmt(fmt),
            Error::CodedError(ref inner) => inner.fmt(fmt),
            Error::MongoError(ref inner) => inner.fmt(fmt),
            Error::TokenError(ref inner) => inner.fmt(fmt),
            Error::DocError(ref inner) => inner.fmt(fmt),
            Error::BsonEncodeError(ref inner) => inner.fmt(fmt),
            Error::BsonDecodeError(ref inner) => inner.fmt(fmt),
            Error::ParseIntError(ref inner) => inner.fmt(fmt),
            Error::ObjectIdError(ref inner) => inner.fmt(fmt),
            Error::RewqestError(ref inner) => inner.fmt(fmt),
            Error::Message(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::Sincere(ref err) => err.description(),
            Error::CodedError(ref err) => err.to_str(),
            Error::MongoError(ref err) => err.description(),
            Error::TokenError(ref err) => err.description(),
            Error::DocError(ref err) => err.description(),
            Error::BsonEncodeError(ref err) => err.description(),
            Error::BsonDecodeError(ref err) => err.description(),
            Error::ParseIntError(ref err) => err.description(),
            Error::ObjectIdError(ref err) => err.description(),
            Error::RewqestError(ref err) => err.description(),
            Error::Message(ref err) => err
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::Sincere(ref err) => Some(err),
            Error::CodedError(_) => None,
            Error::MongoError(ref err) => Some(err),
            Error::TokenError(ref err) => Some(err),
            Error::DocError(ref err) => Some(err),
            Error::BsonEncodeError(ref err) => Some(err),
            Error::BsonDecodeError(ref err) => Some(err),
            Error::ParseIntError(ref err) => Some(err),
            Error::ObjectIdError(ref err) => Some(err),
            Error::RewqestError(ref err) => Some(err),
            Error::Message(_) => None
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub struct ErrorCode(pub u16);

impl ErrorCode {
    pub fn to_str(&self) -> &str {
        match self.0 {
            10004 => "资源不存在",
            10005 => "权限不足",
            10006 => "参数错误",
            10007 => "文件格式错误",
            20001 => "登录信息已过期",
            20002 => "用户名或密码错误",
            20003 => "用户已存在",
            _ => "未知错误"
        }
    }

    pub fn to_code(&self) -> u16 {
        self.0
    }
}

impl From<i16> for ErrorCode {
    fn from(in_code: i16) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u16> for ErrorCode {
    fn from(in_code: u16) -> ErrorCode {
        ErrorCode(in_code)
    }
}

impl From<i32> for ErrorCode {
    fn from(in_code: i32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u32> for ErrorCode {
    fn from(in_code: u32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }

}

impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.to_str())
    }
}
