# YARVE (Yet Another RISC-V Emulator)

RISC-V emulator in Rust. The main purpose of this project is 
for me to get some programming experience in Rust and to learn 
about the RISC-V ISA.

The ultimate goal of this project is to eventually run Linux 
on it, but this is going to take a while. So the first step
would be to implement the RV64G unprivileged ISA with some 
basic IO peripherals. Once this is running I'll continue 
working on the privileged  ISA and anything else required to 
run Linux.

Next to the emulator I also want to implement a basic dynamic
(dis)assembler library, which should allow RISC-V machine code to be
generated and modified in rust.
