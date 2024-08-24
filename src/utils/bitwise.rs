use super::{bit_ops::BitOps, FromBytes, Setable, ToWord};
use crate::constant::{Byte, Half, Quarter, Word};

pub trait BitWise: BitOps + FromBytes + ToWord + Setable + Copy {}

impl BitWise for Byte {}
impl BitWise for Quarter {}
impl BitWise for Half {}
impl BitWise for Word {}
