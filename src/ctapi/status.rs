#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[repr(i8)]
pub enum Status {
    OK = 0,
    ERR_INVALID = -1,
    ERR_CT = -8,
    ERR_TRANS = -10,
    ERR_MEMORY = -11,
    ERR_HOST = -127,
    ERR_HTSI = -128,
}

impl Status {
    fn from_i8(value: i8) -> Self {
        match value {
            0 => Status::OK,
            -1 => Status::ERR_INVALID,
            -8 => Status::ERR_CT,
            -10 => Status::ERR_TRANS,
            -11 => Status::ERR_MEMORY,
            -127 => Status::ERR_HOST,
            -128 => Status::ERR_HTSI,
            _ => {
                error!("Unknown status type given: {}", value);
                Status::ERR_HTSI
            }
        }
    }
}

impl From<i8> for Status {
    fn from(value: i8) -> Self {
        Status::from_i8(value)
    }
}

impl From<Status> for i8 {
    fn from(status: Status) -> i8 {
        status as i8
    }
}

#[test]
fn create_status_from_i8() {
    assert_eq!(Status::OK, Status::from(0));
    assert_eq!(Status::ERR_INVALID, Status::from(-1));
    assert_eq!(Status::ERR_CT, Status::from(-8));
    assert_eq!(Status::ERR_TRANS, Status::from(-10));
    assert_eq!(Status::ERR_MEMORY, Status::from(-11));
    assert_eq!(Status::ERR_HOST, Status::from(-127));
    assert_eq!(Status::ERR_HTSI, Status::from(-128));
}

#[test]
fn status_from_unknown_i8_will_be_err_htsi() {
    assert_eq!(Status::ERR_HTSI, Status::from(1));
    assert_eq!(Status::ERR_HTSI, Status::from(-12));
    assert_eq!(Status::ERR_HTSI, Status::from(-120));
}

#[test]
fn status_has_unique_i8() {
    assert_eq!(Status::OK as i8, 0);
    assert_eq!(Status::ERR_INVALID as i8, -1);
    assert_eq!(Status::ERR_CT as i8, -8);
    assert_eq!(Status::ERR_TRANS as i8, -10);
    assert_eq!(Status::ERR_MEMORY as i8, -11);
    assert_eq!(Status::ERR_HOST as i8, -127);
    assert_eq!(Status::ERR_HTSI as i8, -128);
}
