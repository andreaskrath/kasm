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
These flags are sometimes abbreviated as **of**, **zf**, and **sf**, respectively.

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

Generally speaking, the intention is that if you make a mistake in the program, an error is returned and program execution stops; to the greatest of my ability, undefined behaviour has been limited/removed.

There are a couple of different errors that can be returned, but the most likely are decode errors and execute errors.

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

# Preprocessing
The following section are part of the preprocessing step of the interpreter. In other words, these things happen before the program is interpretted.

## Data Section
The data section functions similar to a key-value store that can be used to defined meaningfully named constants for a program, and is entirely optional to use.

It is located at the tail end of the source code and is initialized with `data:` where the lines below defines the key-value pairs.

The key or name **must** be ascii uppercase letters, digits or underscores, for example `FIB_NUMBER_1` is a valid key, while `fib-number-1` is not.

The value of a key can be any value you would otherwise utilize in-place of the constant.
Therefore, depending on where you use the given constant it could be either a numerical value or a register.

The size of the numerical values and registers are defined by the operation they are used in relation to.
Meaning if the constant, `FIVE 5`, is used in relation to a byte-operation it will be interpretted as a byte.
And the constant, `FIVE_HUNDRED 500`, is used in relation to a byte-operation it will result in a decode error, when the interpreter attempts to decode that instruction.

The following is an example of a data section.
```
data:
  FIB_NUMBER_1 0
  FIVE_HUNDRED 500

  // empty lines are allowed with or without comments
  FIB_NUMBER_2 1
```

Anything below the `data:` marker is considered part of the data section, and will be parsed as such.
Therefore, any source code placed below the marker will result in a data processing error.

The constants in the data section are expanded at runtime, before the program is interpreted.
As such, using constants does not result in a performance loss when the program is interpretted, but it does carry a small overhead to perform the substitution process before interpretation starts.

# Overview
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
- [Stop](#Stop)
- [Call](#Call)
- [Return](#Return)
- [And](#And)
- [Or](#Or)
- [Xor](#Xor)
- [Not](#Not)
- [Test](#Test)
- [Compare](#Compare)

## Set
Sets a register to a given value.

No flags are affected by this instruction.

### Format
This is a generalized format for the set instruction.

```
est* register operand
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
Prints a register value to the defined output.

No flags are affected by this instruction.

### Format
This is a generalized format for the print register instruction.

```
prr* register
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a byte instruction and prints the least significant byte of register *e* to the defined output.

```
prrb rd
```

Say that the least significant byte of register *e* contains the value 150, then the output would look like the following.

```
re: 150
```

Note that this instruction always ends on a newline.

## Print Stack
Prints a section of values from the top of the stack, to the defined output.

No flags are affected by this instruction.

### Format
This is a generalized format for the print stack instruction.

```
prs* operand
```

Where `*` is replaced by any of the size suffixes.

Note that here is also an additional prefix that can be used here, `s` which will interpret the specified section of the stack as an ASCII string.

### Example
The following example is a string instruction and prints the 5 topmost bytes on the stack interpreted as an ASCII string.

```
prss 5
```

Say that the stack looks like the following: \[72, 101, 108, 108, 111\] with the left side being towards the bottom of the stackand the right side being towards the top.

```
Hello
```

If you instead used the `prsb` variant, to interpret the stack section as bytes, the output would be the following.

```
[72, 101, 108, 108, 111]
```

Note that this instruction always ends on a newline.

## Stop
Halts the execution of the program.
Every program must end with a stop instruction, otherwise the program will not terminate correctly.

If the program ends on an empty line, then the program will have a decode error, because it tried to interpret the empty line as an instruction.

No flags are affected by this instruction.

### Format
The format of the stop instruction is always the same, as it is an unsized instruction.

```
stop 
```

## Call
Pushes a return address onto the stack and jumps to a new location in the program.
The return address is the line below the call instruction.

No flags are affected by this instruction.

### Format
The format of the call instruction is always the same, as it is an unsized instruction.

```
call operand
```

The operand is always interpreted as a word.

### Example
The following example calls the function starting on line 10.

```
call 10
```

## Return
Pops a word from the stack and jumps to the location indicated by the word.

No flags are affected by this instruction.

### Format
The format of the return instruction is always the same, as it is an unsized instruction.

```
ret
```

## And
Performs a bitwise AND operation between the two parameters and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the and instruction.

```
and* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a word instruction and performs a bitwise AND operation between register *a* and register *b* and stores the result in the former.

```
andw ra rb
```

## Or
Performs a bitwise OR operation between the two parameters and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the or instruction.

```
or* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a half instruction and performs a bitwise OR operation between register *a* and register *b* and stores the result in the former.

```
orh ra rb
```

## Xor
Performs a bitwise XOR operation between the two parameters and stores the result in the first parameter.

All flags are affected by this instruction.

### Format
This is a generalized format for the xor instruction.

```
xor* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a byte instruction and performs a bitwise XOR operation between register *a* and register *b* and stores the result in the former.

```
xorb ra rb
```

## Not
Performs a bitwise negation of a register.

All flags are affected by this instruction.

### Format
This is a generalized format for the not instruction.

```
not* register
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a quarter instruction and negates the bits in register *d*.

```
notq rd 
```

## Test
Performs a bitwise AND operation between the two parameters and discards the result.

All flags are affected by this instruction.

### Format
This is a generalized format for the test instruction.

```
tst* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a word instruction and performs a bitwise AND operation between register *a* and register *b*, after which the result is discarded.

```
tstw ra rb
```

## Compare
Subtracts the second parameter from the first and discards the result.

All flags are affected by this instruction.

### Format
This is a generalized format for the compare instruction.

```
cmp* register operand
```

Where `*` is replaced by any of the size suffixes.

### Example
The following example is a byte instruction and will subtract 1 from the value in register *g* and discard the result.

```
cmpb rg 1
```
