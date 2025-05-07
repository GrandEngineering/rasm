# RASM
A simple Rusty assembly language.
## Specs
This language is designer for an 1 byte op code and 3 arg bytes. The program memory is 2048 bytes, the registry memory is 256 bytes and the RAM is 65536 bytes.
## instruction set
- NOP  0x00
- HALT 0x01
- ADD  0x02
- SUB  0x03
- NOR  0x04
- AND  0x05
- XOR  0x06
- RSH  0x07
- LDI  0x08
- ADI  0x09
- JMP  0x0A
- BRH  0x0B
- CAL  0x0C
- RET  0x0D
- LOD  0x0E
- STR  0x0F
