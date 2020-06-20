// Copyright 2020 Jean Pierre Dudey. See the LICENSE-MIT and
// LICENSE-APACHE files at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
