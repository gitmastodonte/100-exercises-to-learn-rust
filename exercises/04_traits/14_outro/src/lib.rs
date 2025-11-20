// TODO: Define a new `SaturatingU16` type.
//x   It should hold a `u16` value.
//x   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//x   It should support addition with a right-hand side of type
//x   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//x   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::cmp::Ord;
use std::ops::{Add, Deref};

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl Deref for SaturatingU16 {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &Self) -> Self::Output {
     SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        SaturatingU16 {
            value: value as u16,
        }
    }
}
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> SaturatingU16 {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        SaturatingU16 { value: *value }
    }
}

//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
impl Ord for SaturatingU16 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let value = self.value;
        let other_value = other.value;

        if value > other_value {
            return std::cmp::Ordering::Less;
        } else if value < other_value {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}
