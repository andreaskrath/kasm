# Architecture

## Registers
There are 8 registers, each referred to by a single letter: A through H.

Registers have a 64-bit capacity, but can also be used for instructions that target less than 64-bits.
However, it is worth noting that any bytes outside the ones targeted by an instruction are zeroed.
In other words, setting the least significant byte of a register will clear the remaining seven bytes of all their data.

There are also registers that contain the stack pointer and program counter. However, unlike most other assembly languages, these are not accessible through the instruction set.
In essence, they can be thought of as private registers.

## Stack

# Instructions
Some instructions have size variants, where the variant is specified with a suffix character on the instruction.

The following table illustrates the different size variants.

|Size (in bits)|Name   |Suffix|
|:------------:|:-----:|:----:|
|8             |byte   |b     |
|16            |quarter|q     |
|32            |half   |h     |
|64            |word   |w     |

As an example, the `add` instruction performs arithmetic addition between the two provided paramters, if the instruction is suffixed with `w`, like so `addw`, then the parameters will be interpretted as 64-bit.

## Parameters
There are two types of parameters that an instruction can take: register and operand.

A register is specified with a prefix `r` and then the letter indicating the register, for example `rd` to indicate register D.
There is no prefix required to indicate that a register has been specified.
In the same vein, you do not specify the register differently based on the size variant of the instruction; if you specify register A in a byte instruction, only the least significant byte of the register will be used.

An perand simply refers to a parameter that can either be a register, or an immediate value. 
Immediate values are not prefixed with a special character, you simply write the value as the parameter.

**NB:** for operations that store the result, like arithmetic operations, the first parameter is **always** the destination.

## List
- [Set](#Set)
- [Push](#Push)
- [Pop](#Pop)
- [Addition](#Addition)
- [Subtraction](#Subtraction)
- [Multiplication](#Multiplication)
- [Division](#Division)
- [Remainder](#Remainder)
- [Print Register](#Print-Register)
- [Print Stack](#Print-Stack)

## Set
Sets a register to a given value.

### Format
This is a generalized format for the set instruction.

```
set* register operand
```

Where `*` can be replaced by any of the size suffixes.

### Example
The following example will set register *a* to the value 200, and is a byte instruction.

```
setb ra 200
```

## Push

## Pop

## Addition

## Subtraction

## Multiplication

## Division

## Remainder

## Print Register

## Print Stack


