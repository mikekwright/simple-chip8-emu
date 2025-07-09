The Chip-8
============================

This file is really my personal note taking system for how the chip-8
should function, calling out any specific elements that might be
noteworthy.

Specifications
----------------------------

- [ ] Memory: `CHIP-8 has direct access to up to 4 kilobytes of RAM`

- [ ] Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
- [ ] A program counter, often called just “PC”, which points at the current instruction in memory
- [ ] One 16-bit index register called “I” which is used to point at locations in memory
- [ ] A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
- [ ] An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
- [ ] An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
- [ ] 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
  - [ ] VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag

Instruction Set
---------------------------------

CHIP-8 Instruction Set
<http://octo-ide.com>
N is a number between 0 and 15.
NN is a number between 0 and 255.
NNN is an address between 0 and 4095.
vx and vy are registers (0-F).
i is the memory index register.
Instructions in gray rows may modify the vF register.
Machinecode Octo Instruction Comments
00E0 clear
00EE return Exit a subroutine
1NNN jump NNN
2NNN NNN Call a subroutine
3XNN if vx != NN then
4XNN if vx == NN then
5XY0 if vx != vy then
6XNN vx := NN
7XNN vx += NN
8XY0 vx := vy
8XY1 vx |= vy Bitwise OR
8XY2 vx &= vy Bitwise AND
8XY3 vx ^= vy Bitwise XOR
8XY4 vx += vy vf = 1 on carry
8XY5 vx -= vy vf = 0 on borrow
8XY6 vx >>= vy vf = old least significant bit
8XY7 vx =- vy vf = 0 on borrow
8XYE vx <<= vy vf = old most significant bit
9XY0 if vx == vy then
ANNN i := NNN
BNNN jump0 NNN Jump to address NNN + v0
CXNN vx := random NN Random number 0-255 AND NN
DXYN sprite vx vy N vf = 1 on collision
EX9E if vx -key then Is a key not pressed?
EXA1 if vx key then Is a key pressed?
FX07 vx := delay
FX0A vx := key Wait for a keypress
FX15 delay := vx
FX18 buzzer := vx
FX1E i += vx
FX29 i := hex vx Set i to a hex character
FX33 bcd vx Decode vx into binary-coded decimal
FX55 save vx Save v0-vx to i through (i+x)
FX65 load vx Load v0-vx from i through (i+x)
