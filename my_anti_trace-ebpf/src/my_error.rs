#![no_std]

#[derive(Debug)]
pub enum BpfError {
    Busy,       // -EBUSY
    Invalid,    // -EINVAL
    Permission, // -EPERM
    Again,      // -EAGAIN
    Unknown(i32),
}
const EBUSY: i32 = 16;
const EINVAL: i32 = 22;
const EPERM: i32 = 1;
const EAGAIN: i32 = 11;
impl From<i32> for BpfError {
    fn from(code: i32) -> Self {
        match code {
            x if x == -EBUSY => BpfError::Busy,       // EBUSY
            x if x == -EINVAL => BpfError::Invalid,   // EINVAL
            x if x == -EPERM => BpfError::Permission, // EPERM
            x if x == -EAGAIN => BpfError::Again,     // EAGAIN
            _ => BpfError::Unknown(code),
        }
    }
}
