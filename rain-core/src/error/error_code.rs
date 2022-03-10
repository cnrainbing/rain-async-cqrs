use std::num::NonZeroU32;
use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;
use std::error::Error;
use actix_web::{HttpResponse, Responder, ResponseError};
use crate::response::ServiceResponse;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(NonZeroU32);

pub struct InvalidErrorCode {
    _priv: (),
}

impl ErrorCode {
    pub fn from_u32(src: u32) -> anyhow::Result<ErrorCode, InvalidErrorCode> {
        if src < 1000000 || src >= 9999999 {
            return Err(InvalidErrorCode::new());
        }

        NonZeroU32::new(src)
            .map(ErrorCode)
            .ok_or_else(InvalidErrorCode::new)
    }

    pub fn from_bytes(src: &[u8]) -> anyhow::Result<ErrorCode, InvalidErrorCode> {
        if src.len() != 7 {
            return Err(InvalidErrorCode::new());
        }

        let a = src[0].wrapping_sub(b'0') as u32;
        let b = src[1].wrapping_sub(b'0') as u32;
        let c = src[2].wrapping_sub(b'0') as u32;
        let d = src[3].wrapping_sub(b'0') as u32;
        let e = src[4].wrapping_sub(b'0') as u32;
        let f = src[5].wrapping_sub(b'0') as u32;
        let g = src[6].wrapping_sub(b'0') as u32;

        if a == 0 || a > 9 || b > 9 || c > 9 || d > 9 || e > 9 || f > 9 || g > 9 {
            return Err(InvalidErrorCode::new());
        }

        let status = (a * 1000000) + (b * 100000) + (c * 10000) + (d * 1000) + (e * 100) + (f * 10) + g;
        NonZeroU32::new(status)
            .map(ErrorCode)
            .ok_or_else(InvalidErrorCode::new)
    }

    pub fn as_u32(&self) -> u32 {
        (*self).into()
    }

    pub fn as_str(&self) -> &'static str {
        self::canonical_reason(self.0.get()).unwrap()
    }

    pub fn as_string(&self) -> String {
        self::canonical_reason(self.0.get()).unwrap().to_string()
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
    }
}

impl ResponseError for ErrorCode {
    fn error_response(&self) -> HttpResponse {
        let err_json = ServiceResponse::<&str>::failure_msg_str(self.as_u32(), self.as_str());
        HttpResponse::Ok().json(err_json)
    }
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            u32::from(*self),
            self.canonical_reason().unwrap_or("<unknown error code>")
        )
    }
}

impl Default for ErrorCode {
    #[inline]
    fn default() -> ErrorCode {
        ErrorCode::OK
    }
}

impl PartialEq<u32> for ErrorCode {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.as_u32() == *other
    }
}

impl PartialEq<ErrorCode> for u32 {
    #[inline]
    fn eq(&self, other: &ErrorCode) -> bool {
        *self == other.as_u32()
    }
}

impl From<ErrorCode> for u32 {
    #[inline]
    fn from(status: ErrorCode) -> u32 {
        status.0.get()
    }
}

impl FromStr for ErrorCode {
    type Err = InvalidErrorCode;

    fn from_str(s: &str) -> anyhow::Result<ErrorCode, InvalidErrorCode> {
        ErrorCode::from_bytes(s.as_ref())
    }
}

impl<'a> From<&'a ErrorCode> for ErrorCode {
    #[inline]
    fn from(t: &'a ErrorCode) -> Self {
        t.clone()
    }
}

impl<'a> TryFrom<&'a [u8]> for ErrorCode {
    type Error = InvalidErrorCode;

    #[inline]
    fn try_from(t: &'a [u8]) -> anyhow::Result<Self, Self::Error> {
        ErrorCode::from_bytes(t)
    }
}

impl<'a> TryFrom<&'a str> for ErrorCode {
    type Error = InvalidErrorCode;

    #[inline]
    fn try_from(t: &'a str) -> anyhow::Result<Self, Self::Error> {
        t.parse()
    }
}

impl TryFrom<u32> for ErrorCode {
    type Error = InvalidErrorCode;

    #[inline]
    fn try_from(t: u32) -> anyhow::Result<Self, Self::Error> {
        ErrorCode::from_u32(t)
    }
}

macro_rules! error_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl ErrorCode {
        $(
            $(#[$docs])*
            pub const $konst: ErrorCode = ErrorCode(unsafe { NonZeroU32::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u32) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

error_codes! {
    (0, ERROR, "ERROR");
    (200, OK, "OK");
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
    //用户错误1001000 - 1001999
    (1001000, USER_UID_NOT_FOUND, "用户UID不存在");
    (1001001, USER_ID_NOT_FOUND, "用户ID不存在");
    (1001002, USER_ACCESS_KEY_AUTH_EXC, "用户accessKey授权异常");

    //订单错误1002000 - 1002999
    (1002000, ORDER_ID_NOT_FOUND, "订单不存在");

    //订单错误1003000 - 1003999
    (1003000, DB_ERR, "数据库错误");
}

impl InvalidErrorCode {
    fn new() -> InvalidErrorCode {
        InvalidErrorCode {
            _priv: (),
        }
    }
}


impl fmt::Debug for InvalidErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidErrorCode")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid error code")
    }
}

impl Error for InvalidErrorCode {}
