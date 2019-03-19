#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
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
    pub fn value(&self) -> i8 {
        match *self {
            Status::OK => 0,
            Status::ERR_INVALID => -1,
            Status::ERR_CT => -8,
            Status::ERR_TRANS => -10,
            Status::ERR_MEMORY => -11,
            Status::ERR_HOST => -127,
            Status::ERR_HTSI => -128,
        }
    }

    pub fn from_i8(value: i8) -> Self {
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

#[test]
fn create_status_from_i8() {
    assert_eq!(Status::OK, Status::from_i8(0));
    assert_eq!(Status::ERR_INVALID, Status::from_i8(-1));
    assert_eq!(Status::ERR_CT, Status::from_i8(-8));
    assert_eq!(Status::ERR_TRANS, Status::from_i8(-10));
    assert_eq!(Status::ERR_MEMORY, Status::from_i8(-11));
    assert_eq!(Status::ERR_HOST, Status::from_i8(-127));
    assert_eq!(Status::ERR_HTSI, Status::from_i8(-128));
}

#[test]
fn status_from_unknown_i8_will_be_err_htsi() {
    assert_eq!(Status::ERR_HTSI, Status::from_i8(1));
    assert_eq!(Status::ERR_HTSI, Status::from_i8(-12));
    assert_eq!(Status::ERR_HTSI, Status::from_i8(-120));
}

#[test]
fn status_has_unique_i8() {
    assert_eq!(Status::OK.value(), 0);
    assert_eq!(Status::ERR_INVALID.value(), -1);
    assert_eq!(Status::ERR_CT.value(), -8);
    assert_eq!(Status::ERR_TRANS.value(), -10);
    assert_eq!(Status::ERR_MEMORY.value(), -11);
    assert_eq!(Status::ERR_HOST.value(), -127);
    assert_eq!(Status::ERR_HTSI.value(), -128);
}
