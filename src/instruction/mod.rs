use variant_count::VariantCount;

pub use addition::Addition;
pub use division::Division;
pub use multiplication::Multiplication;
pub use push::Push;
pub use remainder::Remainder;
pub use set::Set;
pub use subtraction::Subtraction;

mod addition;
mod division;
mod multiplication;
mod push;
mod remainder;
mod set;
mod subtraction;

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
}
