use variant_count::VariantCount;

mod add;
pub use add::AddInstruction;

mod div;
pub use div::DivInstruction;

mod mul;
pub use mul::MulInstruction;

mod set;
pub use set::SetInstruction;

mod sub;
pub use sub::SubInstruction;

#[derive(Debug, PartialEq, VariantCount)]
pub enum Instruction {
    Stop,
    Set(SetInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Mul(MulInstruction),
    Div(DivInstruction),
}
