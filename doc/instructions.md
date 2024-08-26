# Architecture

## Registers
There are 8 registers, each referred to by a single letter: A through H.

Registers have a 64-bit capacity, but can also be used for instructions that target less than 64-bits.
However, it is worth noting that any bytes outside the ones targeted by an instruction are zeroed.
In other words, setting the least significant byte of a register will clear the remaining seven bytes of all their data.

There are also registers that contain the stack pointer and program counter. However, unlike most other assembly languages, these are not accessible through the instruction set.
In essence, they can be thought of as private registers.

## Stack

## Flags
There are three flags: **overflow**, **zero**, and **sign**.

The overflow flag is set when an instruction results in an arithmetic overflow.
Also note that in the case of an overflow, the wrapped value is stored as the result, and the overflow flag is set.

The zero flag is set when an instruction results in the value zero.

The sign flag is set when an instruction results in the most significant bit of a value being 1.

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

## Errors
Because this interpreter does not mimic a processor, some liberties are taken in certain areas - one of these are in relation to errors.

Generally speaking, the intention is that if you make a mistake in the program, an error is returned and program execution stops; as much as possible, undefined behaviour has been limited/removed.

There are a couple of different errors that can happen, but the most likely are decode errors and execute errors.

A decode error indicates that something was wrong with the specified instruction, this can come in multiple different forms including:
- unknown instruction
- incomplete instruction (missing paraters)
- invalid register
- invalid immediate value (for example specifying 3000 as the immediate value of a byte operation)

In other wrods, decode errors indicate that something is wrong in the source code of the program being executed.

Execute errors on the other hard indicate that something went wrong during execution, this can also come in multiple different forms like:
- stack overflow
- stack underflow
- io error (in relation to print statements)
- attempting divide by 0

In other words, execute errors indicate an issue that occured during the execution of the program, most often this would be logic errors in the program.

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


