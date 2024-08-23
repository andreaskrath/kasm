use crate::register::Register;

#[derive(Debug, PartialEq)]
pub enum Pop {
    Byte(Register),
    Quarter(Register),
    Half(Register),
    Word(Register),
}
