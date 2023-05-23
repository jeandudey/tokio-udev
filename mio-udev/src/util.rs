// SPDX-FileCopyrightText: © 2020 Jean-Pierre De Jesus DIAZ <me@jeandudey.tech>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{io, ops::Neg};

#[doc(hidden)]
pub trait One {
    fn one() -> Self;
}

macro_rules! one {
    ($($t:ident)*) => ($(
        impl One for $t { fn one() -> $t { 1 } }
    )*)
}

one! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

pub fn cvt<T: One + PartialEq + Neg<Output = T>>(t: T) -> io::Result<T> {
    let one: T = T::one();
    if t == -one {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
