# 8bit Computer Emulator

This program emulates [Ben Eater](https://youtube.com/beneater)'s 8bit computer.
It is a very rudimentary computer (it only has 16B of RAM), but can be useful
as a teaching tool or a toy.

# Description

The computer is modular, so elements could be added somewhat easily.
The default modules are:

* 2 general purpose registers (A and B)
* An Arithmetic Logic Unit (ALU) capable of adding and substracting the values
    in A and B
* 16B of RAM
* An output register, to see values in decimal

# Usage

You can press `C` to toggle between single-stepping mode and run mode.
In single-stepping mode, the `Return` key is used to step through cycles.
In run mode, the clock can be slowed down or sped up using `PageDown` and
`PageUp` respectively.
The modules can be reset using the `R` key (useful if the computer halts after
finishing a program).
Pressing  `Esc` closes the program.

# Piston

I originally used [Piston](https://github.com/PistonDevelopers/piston)
(a pure Rust game engine) to display the computer, but the compilation times
and the size of the build artifacts made me reconsider it. Instead of throwing
it away, I put the Piston renderer behind a feature flag. If you want to use
it, compile with:

    $ cargo build --release --features piston

Without this feature flag, the renderer uses SDL2.
