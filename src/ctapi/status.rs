use std::cmp::Ordering;
use std::fmt;

pub enum StatusCode {
    // 0
    Ok,
    // -1
    ErrInvalid,
    // -8
    ErrCt,
    // -10
    ErrTrans,
    // -11
    ErrMemory,
    // -127
    ErrHost,
    // -128
    ErrHtsi,
}

impl StatusCode {
    pub fn to_i8(&self) -> i8 {
        match *self {
            StatusCode::Ok => 0,
            StatusCode::ErrInvalid => -1,
            StatusCode::ErrCt => -8,
            StatusCode::ErrTrans => -10,
            StatusCode::ErrMemory => -11,
            StatusCode::ErrHost => -127,
            StatusCode::ErrHtsi => -128,
        }
    }

    pub fn from_i8(n: i8) -> Result<StatusCode, &'static str> {
        match n {
            0 => Ok(StatusCode::Ok),
            -1 => Ok(StatusCode::ErrInvalid),
            -8 => Ok(StatusCode::ErrCt),
            -10 => Ok(StatusCode::ErrTrans),
            -11 => Ok(StatusCode::ErrMemory),
            -127 => Ok(StatusCode::ErrHost),
            -128 => Ok(StatusCode::ErrHtsi),
            _ => Err("Invalid CTAPI status code!"),
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_i8())
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_i8())
    }
}

impl PartialEq for StatusCode {
    fn eq(&self, other: &StatusCode) -> bool {
        self.to_i8() == other.to_i8()
    }
}

impl PartialEq<i8> for StatusCode {
    fn eq(&self, other: &i8) -> bool {
        self.to_i8() == *other
    }
}

impl PartialEq<StatusCode> for i8 {
    fn eq(&self, other: &StatusCode) -> bool {
        self == &other.to_i8()
    }
}

impl PartialOrd for StatusCode {
    fn partial_cmp(&self, other: &StatusCode) -> Option<Ordering> {
        self.to_i8().partial_cmp(&(other.to_i8()))
    }
}

impl From<StatusCode> for i8 {
    fn from(code: StatusCode) -> i8 {
        code.to_i8()
    }
}
