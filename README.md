# project_mips
### Mips VM simulating basic operation of memory with binary feedback

### Overview
This is an operational demonstration of a simple mips processor and memory cache operation. This while in it's simplest form is intnded to help show how the mips processor can convert instructions into binary form and pass them through various components. The purpose of this software is intended to help provide hands on experience with the mips system and to allow students to write individual lines of mips instruction assembly code and watch in real time as the simulated memory cache is updated. 

### Installation
This repository can be compiled on any system that runs a Rust compiler. Operation, inspection and modification of the code has been setup to be as easy as possible. Installation of the Rust language and compiler can be found [here](https://www.rust-lang.org/tools/install)
- Download the source code
- Unzip project_mips in the location you would like to run it from
- Open up a terminal 
- Use the cd command to change the terminal directory to the location of the cargo.toml file in project_mips
- enter `cargo run`


### Operation
Only a limited number of commands are currently supported and can only be entered individually through the command line. 
- Add (Adds the contents of 2 selected registers and store the result in a third)
- Sub (Uses 2's compliment to subtract the contents of 2 selected registers and store the result in a third)
- And (Binary and the contents of two registers and store the result in a third)
- Or (Binary or the contents of two registers and store the result in a third)
- Addi (Add the contents of a register to a 16 bit integer and store it in a selected register)

Each of these instructions will create a binary instruction and pass it through simulated components to edit the necessary register. To select a register simply use the register code format of "R#". Register support includes from R0 to R31.

### Issues
Errors and issues can be reported on [github](github.com/Z-Walt_734/project_mips)

### Future additions
- [] Create Outside Memory support
- [] Add LW SW support
- [] Complete Mips Instruction Operation
- [] Add instruction file support
- [] Upload to Rust Cargo 
- [] Add GUI
