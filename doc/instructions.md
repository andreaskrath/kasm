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

# Overview
- [Set](#Set)

**Arithmetic**
- [Addition](#Addition)
- [Subtraction](#Subtraction)
- [Multiplication](#Multiplication)
- [Division](#Division)
- [Remainder](#Remainder)

**Control Flow**
- [Stop](#Stop)
- [Call](#Call)
- [Return](#Return)
- [Jump](#Jump)
- [Compare](#Compare)
- [Test](#Test)

**Stack**
- [Push](#Push)
- [Pop](#Pop)

**Bitwise**
- [And](#And)
- [Or](#Or)
- [Xor](#Xor)
- [Not](#Not)

**Print**
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
This instruction will return a divide by zero error in case the divisor is zero.

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
This instruction will return a divide by zero error in case the divisor is zero.

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

### Error
This instruction will return a stack overflow error in case the stack cannot contain the eight bytes to be pushed onto it.

## Return
Pops a word from the stack and jumps to the location indicated by the word.

No flags are affected by this instruction.

### Format
The format of the return instruction is always the same, as it is an unsized instruction.

```
ret
```

### Error
This instruction will return a stack underflow error in case the stack contains less than eight bytes.

## Jump
Jumps to a given location in the program.

No flags are affected by this instruction.

### Format
This is a generalized format for the compare instruction.

```
j** operand
```

Where `**` is replaced by any of the jump variants.

Jump instructions are unsized operations, meaning the operand is always intepreted as a word.

Note that there is a special variant of jumps called relative jumps. Their purpose is to make function code more self-contained and easier to write.

You make a relative jump by prefixing the operand by either `+` or `-`, the former indicating a jump down, and the latter a jump up, in the source code.

This means that instead of hardcoding a jump location inside a function, you can use a relative jump and not have to think about the jump location changing if the code changes location in the file.

### Variants
There are 11 different jump variants.

|Name                    |Instruction|Condition           |
|:----------------------:|:---------:|:------------------:|
|Jump                    |`jmp`      |unconditional       |
|Jump If Zero            |`jiz`      |zf == 1             |
|Jump If Not Zero        |`jnz`      |zf == 0             |
|Jump If Overflow        |`jio`      |of == 1             |
|Jump If Not Overflow    |`jno`      |of == 0             |
|Jump If Sign            |`jis`      |sf == 1             |
|Jump If Not Sign        |`jns`      |sf == 0             |
|Jump If Greater         |`jig`      |of == 0 && zf == 0  |
|Jump If Lesser          |`jil`      |of == 1 && zf == 0  |
|Jump If Greater Or Equal|`jge`      |of == 0 \|\| zf == 1|
|Jump If Lesser Or Equal |`jle`      |of == 1 \|\| zf == 1|

### Example
The following example jumps to the location contained in register *a*, if the zero flag is set.

```
jiz ra
```

The following example performs a relative jump to the instruction two lines above the jump.

```
jmp -2
```

### Error
This instruction can indirectly cause an error, if the location being jumped to is either:
- not part of the source code, i.e. jumping to line 100 if the program is only 50 lines
- an empty line

In these cases the following interpretation loop will result in an error.

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
This instruction will return a stack overflow error in case the stack cannot contain the value specified to be pushed onto it.

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
This instruction will return a stack underflow error in case the stack contains less bytes than specified to be popped by the instruction.

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

### Error
This instruction can result in an IO error, if the register could not be written to the defined output.

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

### Error
This instruction can result in an IO error, if the stack section could not be written to the defined output.

# Preprocessing
The following section are part of the preprocessing step of the interpreter. In other words, these things happen before the program is interpretted.

## Data Section
The data section functions similar to a key-value store that can be used to defined meaningfully named constants for a program, and is entirely optional to use.

It is located at the tail end of the source code and is initialized with `DATA:` where the lines below defines the key-value pairs.

The key or name **must** be ascii uppercase letters, digits or underscores, for example `FIB_NUMBER_1` is a valid key, while `fib-number-1` is not.

The value of a key can be any value you would otherwise utilize in-place of the constant.
Therefore, depending on where you use the given constant it could be either a numerical value or a register.

The size of the numerical values and registers are defined by the operation they are used in relation to.
Meaning if the constant, `FIVE 5`, is used in relation to a byte-operation it will be interpretted as a byte.
And the constant, `FIVE_HUNDRED 500`, is used in relation to a byte-operation it will result in a decode error, when the interpreter attempts to decode that instruction.

The following is an example of a data section.
```
DATA:
  FIB_NUMBER_1 0
  FIVE_HUNDRED 500

  // empty lines are allowed with or without comments
  FIB_NUMBER_2 1
```

Anything below the `DATA:` marker is considered part of the data section, and will be parsed as such.
Therefore, any source code placed below the marker will result in a data processing error.

The constants in the data section are expanded at runtime, before the program is interpreted.
As such, using constants does not result in a performance loss when the program is interpretted, but it does carry a small overhead to perform the substitution process before interpretation starts.
This also means that the data section is not actually a valid part of the program, it is substituted and removed before the program is interpretted.

## Functions
Kasm supports functions that will be substituted for their jump destination before the program is interpretted.

Functions are defined with the `fn` keyword, following by the function name and ending with a colon. The function name must be snake case. Not following this format will result in a preprocessing error.

For example, if I want to define a function that increments register *a* it could look like this:
```
fn inc_ra:
  // function body
```

Usually you want end a function body on the `ret` instruction to return to the call site of the function, however this is not a hard requirement.

The function substitution process will ensure that:
- any place you call a function by name, such as `call inc_ra`, the function name is substituted with the location of the first line of the function that is called
- all functions adhere to the specified format
- any function call is valid, i.e. calling a function that is actually defined elsewhere in the program
- a given function name can only be defined once

Violating any of the above will result in an error.

