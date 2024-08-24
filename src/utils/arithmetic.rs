use super::{FromBytes, Overflow, Setable};
use crate::constant::{Byte, Half, Quarter, Word};

pub trait Arithmetic: Overflow + Setable + FromBytes + Copy {}

impl Arithmetic for Byte {}
impl Arithmetic for Quarter {}
impl Arithmetic for Half {}
impl Arithmetic for Word {}
