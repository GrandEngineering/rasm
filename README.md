# RASM
A simple Rusty assembly language.
# RASM Instruction Set Reference

## Basic Instructions

### NOP (0x00)
**No Operation**
Does nothing, simply advances to the next instruction.

```
NOP
```

### HALT (0x01)
**Halt Execution**
Stops program execution.

```
HALT
```

## Arithmetic & Logic Operations

### ADD (0x02)
**Add Register**
Adds the value in the second register to the first register.

```
ADD r1 r2
```
*Parameters:*
- `r1`: Destination register (0-255)
- `r2`: Source register (0-255)

*Operation:* `r1 = r1 + r2`

### SUB (0x03)
**Subtract Register**
Subtracts the value in the second register from the first register.

```
SUB r1 r2
```
*Parameters:*
- `r1`: Destination register (0-255)
- `r2`: Source register (0-255)

*Operation:* `r1 = r1 - r2`

### NOR (0x04)
**Bitwise NOR**
Performs bitwise NOR operation between the values in two registers.

```
NOR r1 r2
```
*Parameters:*
- `r1`: Destination register (0-255)
- `r2`: Source register (0-255)

*Operation:* `r1 = ~(r1 | r2)`

### AND (0x05)
**Bitwise AND**
Performs bitwise AND operation between the values in two registers.

```
AND r1 r2
```
*Parameters:*
- `r1`: Destination register (0-255)
- `r2`: Source register (0-255)

*Operation:* `r1 = r1 & r2`

### XOR (0x06)
**Bitwise XOR**
Performs bitwise XOR operation between the values in two registers.

```
XOR r1 r2
```
*Parameters:*
- `r1`: Destination register (0-255)
- `r2`: Source register (0-255)

*Operation:* `r1 = r1 ^ r2`

### RSH (0x07)
**Right Shift**
Shifts the bits in the register one position to the right.

```
RSH r1
```
*Parameters:*
- `r1`: Register to shift (0-255)

*Operation:* `r1 = r1 >> 1`

## Immediate Operations

### LDI (0x08)
**Load Immediate**
Loads an immediate value into a register.

```
LDI r1 value
```
*Parameters:*
- `r1`: Destination register (0-255)
- `value`: 8-bit immediate value (0-255)

*Operation:* `r1 = value`

### ADI (0x09)
**Add Immediate**
Adds an immediate value to a register.

```
ADI r1 value
```
*Parameters:*
- `r1`: Destination register (0-255)
- `value`: 8-bit immediate value (0-255)

*Operation:* `r1 = r1 + value`

## Control Flow

### JMP (0x0A)
**Jump**
Unconditionally jumps to a specified address in program memory.

```
JMP address
```
*Parameters:*
- `address`: Program address to jump to (16-bit value)

*Operation:* `PC = address`

### BRH (0x0B)
**Branch if Not Zero**
Conditionally jumps to a specified address if a register's value is not zero.

```
BRH r1 address
```
*Parameters:*
- `r1`: Register to test (0-255)
- `address`: Program address to jump to if condition is met (16-bit value)

*Operation:* `if r1 != 0 then PC = address`

### CAL (0x0C)
**Call Subroutine**
Calls a subroutine at the specified address, saving the return address on the stack.

```
CAL address
```
*Parameters:*
- `address`: Subroutine address to call (16-bit value)

*Operation:*
1. Push current PC onto stack
2. `PC = address`

### RET (0x0D)
**Return from Subroutine**
Returns from a subroutine by popping the return address from the stack.

```
RET
```
*Operation:* `PC = pop from stack`

## Memory Operations

### LOD (0x0E)
**Load from Memory**
Loads a value from memory into a register.

```
LOD r1 address   # Traditional syntax
LOD r1 mN        # Alternative syntax using mN for memory address
```
*Parameters:*
- `r1`: Destination register (0-255)
- `address`: Memory address to read from (16-bit value)
- `mN`: Alternative notation for memory location N

*Operation:* `r1 = memory[address]`

### STR (0x0F)
**Store to Memory**
Stores a register value into memory.

```
STR r1 address   # Traditional syntax
STR r1 mN        # Alternative syntax using mN for memory address
```
*Parameters:*
- `r1`: Source register (0-255)
- `address`: Memory address to write to (16-bit value)
- `mN`: Alternative notation for memory location N

*Operation:* `memory[address] = r1`

*Special Case:* When storing to memory address 2 (`m2`), the value is interpreted as an ASCII character and printed to the console.

## Data Directives

### BYTE
**Define Byte**
Inserts a raw byte value directly into program memory.

```
BYTE value
```
*Parameters:*
- `value`: 8-bit value to insert (0-255)

### LABEL
**Define Label**
Creates a named reference to the current program address.

```
LABEL name
```
*Parameters:*
- `name`: Label identifier

## String Support

### String Definitions
Defines a named string constant.

```
#str tag "string content"
```

### PRINT Directive
Generates code to print a defined string.

```
PRINT tag [NL]
```
*Parameters:*
- `tag`: String identifier
- `NL`: Optional parameter to add a newline

*Note:* This expands to a series of LDI and STR instructions for each character.
