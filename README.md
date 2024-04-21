# Welcome to bline, my own programming language

[here for more info](./docs/syntax/syntax.md)

post development note:

making a programming language is not as complicated as i thought but, it is COMPLICATED and i doomed my syntax and now it is too late,
tokenizing and parsing is not so complicated (except for expressions),
i tried to transpile it to c but my already written code would make it very difficult, if i ever come back to this project i'd re-write everything from scratch (except my expressions parser) boy did i hate parsing expressions,
i realized how semi-perfect C is after writing this, we should have followed C's syntax all along

Goodbye.

by the way i named it bline but i dont think it deserves a name, so you're free to use it, (not the logo tho).

Development: March 2024 - 21/Apr/2024


uhh if youre interested on using it you can:
clone the repo
cargo run ./file.bline,
gcc ./prototype01.c -o binary02 -I ./src/lib/
./binary02
