use crate::{constant::Word, operand::Operand};
use variant_count::VariantCount;

pub use addition::Addition;
pub use and::And;
pub use compare::Compare;
pub use division::Division;
pub use jump::{Jump, Relative};
pub use multiplication::Multiplication;
pub use not::Not;
pub use or::Or;
pub use pop::Pop;
pub use print_register::PrintRegister;
pub use print_stack::PrintStack;
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
mod print_stack;
mod push;
mod remainder;
mod set;
mod subtraction;
mod test;
mod xor;

#[derive(Debug, PartialEq, VariantCount)]
pub enum Instruction {
    Addition(Addition),
    And(And),
    Call(Operand<Word>),
    Compare(Compare),
    Division(Division),
    Jump(Jump, Operand<Word>, Option<Relative>),
    Multiplication(Multiplication),
    Not(Not),
    Or(Or),
    Pop(Pop),
    PrintRegister(PrintRegister),
    PrintStack(PrintStack),
    Push(Push),
    Remainder(Remainder),
    Return,
    Set(Set),
    Stop,
    Subtraction(Subtraction),
    Test(Test),
    Xor(Xor),
}

impl Instruction {
    /// Returns true if this instruction should increment the program counter after execution.
    pub fn increment(&self) -> bool {
        use Instruction::*;

        match self {
            Addition(_) | And(_) | Compare(_) | Division(_) | Multiplication(_) | Not(_)
            | Or(_) | Pop(_) | PrintRegister(_) | PrintStack(_) | Push(_) | Remainder(_)
            | Set(_) | Subtraction(_) | Test(_) | Xor(_) => true,

            Call(_) | Jump(_, _, _) | Return | Stop => false,
        }
    }
}
