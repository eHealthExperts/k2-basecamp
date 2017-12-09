use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub enum Status {
    OK,
    ErrInvalid,
    ErrCt,
    ErrTrans,
    ErrMemory,
    ErrHost,
    ErrHtsi,
    Unknown(i8),
}

impl From<i8> for Status {
    fn from(value: i8) -> Status {
        match value {
            0 => Status::OK,
            -1 => Status::ErrInvalid,
            -8 => Status::ErrCt,
            -10 => Status::ErrTrans,
            -11 => Status::ErrMemory,
            -127 => Status::ErrHost,
            -128 => Status::ErrHtsi,
            code => Status::Unknown(code),
        }
    }
}

impl From<Status> for i8 {
    fn from(status: Status) -> i8 {
        match status {
            Status::OK => 0,
            Status::ErrInvalid => -1,
            Status::ErrCt => -8,
            Status::ErrTrans => -10,
            Status::ErrMemory => -11,
            Status::ErrHost => -127,
            Status::ErrHtsi => -128,
            Status::Unknown(code) => code,
        }
    }
}

impl FromStr for Status {
    type Err = ParseIntError;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        let code: i8 = try!(status.parse::<i8>());
        Ok(From::from(code))
    }
}

impl PartialEq<Status> for i8 {
    fn eq(&self, status: &Status) -> bool {
        let code: i8 = From::from(status.clone());
        *self == code
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code: i8 = From::from(self.clone());
        let _ = f.write_str(&code.to_string());
        Ok(())
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
