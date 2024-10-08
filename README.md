# Krath Assembly
This repository contains an assembly interpreter for a custom dialect of assembly, named **kasm**, I have created.

It has been developed as a hobby project, and purely for my own learning and enjoyment.

It is only intended to be utilized as a command line application, and as such I have taken certain liberties in regards to things like syscalls.

# Goals
The following list contains my goals for the project:
- Implement all the features presented in the Feature Roadmap section.
- Have a hopefully bug-free and relatively performant implementation with good code practices.
- To be able to execute relatively large or complex programs.
- Developing a program in kasm should feel like any other assembly language.

# Non-goals
- To see any practical use as a compile target for anything, or to run any meaningful code.
- To accurately simulate a full CPU (logic gates, ALU etc.); for me it is only important that it feels like you are interacting with the CPU.

# Feature Roadmap
The following list is not necessarily in any specific order, however it encompasses the long term goals of this project:
- [x] Unsigned instruction set (also including instructions without a size component)
- [x] Data section of program
- [x] A debug mode (set with CLI flag) that allows incremental interpretation (stepping)
- [x] Call functions by name
- [x] Relative jump locations (+2, -2 or the like)
- [ ] Logging/tracing
- [ ] Signed instruction set
- [ ] Support float instructions
- [ ] Heap, and associated instructions (store, load, indexing etc.)
- [ ] Documentation of the architecture, instruction set etc.
- [ ] Revamp cli to utilize commands (run, debug, prepare) instead of flags
