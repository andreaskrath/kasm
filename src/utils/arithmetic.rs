use super::{overflow::Overflow, FromBytes, Setable, ToWord};
use crate::constant::{Byte, Half, Quarter, Word};

pub trait Arithmetic: Overflow + Setable + FromBytes + Copy + ToWord {}

impl Arithmetic for Byte {}
impl Arithmetic for Quarter {}
impl Arithmetic for Half {}
impl Arithmetic for Word {}
