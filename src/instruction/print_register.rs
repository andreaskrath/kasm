use crate::register::Register;

#[derive(Debug, PartialEq)]
pub enum PrintRegister {
    Byte(Register),
    Quarter(Register),
    Half(Register),
    Word(Register),
}
