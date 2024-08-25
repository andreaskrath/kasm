use crate::{constant::Word, operand::Operand};
use variant_count::VariantCount;

pub use addition::Addition;
pub use and::And;
pub use compare::Compare;
pub use division::Division;
pub use jump::Jump;
pub use multiplication::Multiplication;
pub use not::Not;
pub use or::Or;
pub use pop::Pop;
pub use print_register::PrintRegister;
pub use push::Push;
pub use remainder::Remainder;
pub use set::Set;
pub use subtraction::Subtraction;
pub use test::Test;
pub use xor::Xor;

mod addition;
mod and;
mod compare;
mod division;
mod jump;
mod multiplication;
mod not;
mod or;
mod pop;
mod print_register;
mod push;
mod remainder;
mod set;
mod subtraction;
mod test;
mod xor;

#[derive(Debug, PartialEq, VariantCount)]
pub enum Instruction {
    Stop,
    Set(Set),
    Addition(Addition),
    Subtraction(Subtraction),
    Multiplication(Multiplication),
    Division(Division),
    Remainder(Remainder),
    Push(Push),
    Pop(Pop),
    Call(Operand<Word>),
    Return,
    And(And),
    Or(Or),
    Xor(Xor),
    Not(Not),
    Test(Test),
    Compare(Compare),
    Jump(Jump),
    PrintRegister(PrintRegister),
}
