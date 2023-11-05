// an Answer trait which requires Debug and Display
use std::fmt::{Debug, Display};

pub trait Answer: Debug + Display + PartialEq {}

impl Answer for String {}
impl Answer for &str {}
impl Answer for u8 {}
impl Answer for i8 {}
impl Answer for u16 {}
impl Answer for i16 {}
impl Answer for u32 {}
impl Answer for i32 {}
impl Answer for u64 {}
impl Answer for i64 {}
impl Answer for u128 {}
impl Answer for i128 {}
impl Answer for f32 {}
impl Answer for f64 {}
impl Answer for usize {}
impl Answer for isize {}
