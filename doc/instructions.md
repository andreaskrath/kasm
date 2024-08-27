# Architecture

## Registers
There are 8 registers:
- Register A referred to by `ra`
- Register B referred to by `rb`
- Register C referred to by `rc`
- Register D referred to by `rd`
- Register E referred to by `re`
- Register F referred to by `rf`
- Register G referred to by `rg`
- Register H referred to by `rh`

Registers have a 64-bit capacity, but can also be used for instructions that target less than 64-bits.
However, it is worth noting that any bytes outside the ones targeted by an instruction are zeroed.
In other words, setting the least significant byte of a register will clear the remaining seven bytes of all their data.

Each register does not have a designated purpose like in other instruction sets, and you are free to use them as you see fit.

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

Note that the size of an instruction matters, in the sense that `setb ra 3000` will result in a decode error, as the value 3000 cannot be represented in a single byte.

## Parameters
There are two types of parameters that an instruction can take: register and operand.

A register is specified with a prefix `r` and then the letter indicating the register, for example `rd` to indicate register *d*.
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

No flags are affected by this instruction.

### Format
This is a generalized format for the set instruction.

```
set* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a byte instruction and will set register *a* to the value 200.

```
setb ra 200
```

## Push
Pushes a value onto the stack.

No flags are affected by this instruction.

### Format
This is a generalized format for the push instruction.

```
psh* operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a word instruction and will push the value in register *a* onto the stack.

```
pshw ra
```

### Error
This instruction can return a stack overflow error in case the stack cannot contain the value specified to be pushed onto it.

## Pop
Pops a value from the stack into a register.

No flags are affected by this instruction.

### Format
This is a generalized format for the pop instruction.

```
pop* register
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a half instruction and will pop the four topmost bytes on the stack into register *b*.

```
poph rb
```

### Error
This instruction can return a stack underflow error in case the stack contained less bytes than specified to be popped by the instruction.

## Addition
Adds two values and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the add instruction.

```
add* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a quarter instruction and will add the values stored in register *a* and *f* and store the result in register *a*.

```
addq ra rf
```

## Subtraction
Subtracts the second parameter from the first and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the sub instruction.

```
sub* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a word instruction and will decrement the value in register *g*.

```
subw rg 1
```

## Multiplication
Multiplies two values and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the mul instruction.

```
mul* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a byte instruction and will double the value stored in register *b*.

```
mulb rb 2
```

## Division
Divides the first parameter with the second, and stores the result in the first parameter.

For the unsigned variants of this instruction, the zero and sign flags are affected, and the overflow flag is cleared.
For the signed variants of this instruction, the overflow flag is also affected.

### Format
This is a generalized format for the div instruction.

```
div* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a quarter instruction divide the value in register *d* by 4.

```
divq rd 4
```

### Error
This instruction can return a divide by zero error in case the divisor is zero.

## Remainder
Divides the first parameter with the second to determine the remainder, and stores the result in the first parameter.

For the unsigned variants of this instruction, the zero and sign flags are affected, and the overflow flag is cleared.
For the signed variants of this instruction, the overflow flag is also affected.

### Format
This is a generalized format for the rem instruction.

```
rem* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a half instruction and computes the remainder after dividing the value in register *a* with 3, and stores the result in register *a*.

```
remh ra 3
```

### Error
This instruction can return a divide by zero error in case the divisor is zero.

## Print Register

## Print Stack

