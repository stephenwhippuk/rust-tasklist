# rust-tasklist
learning exercise to play with traits and tests

## Purpose and Overview
I would never choose Rust for an application like this for a RL project, as the needs for performance are far outweighed by the convenience and maintainability of a higher level language. However as a learning exercise, a Simple task List maintenance system is a nice simple example

it will:
- have a file reader and writer
- have a keyboard reader
- have a command parser
- have a Display system

the system will be controlled by a command line interface, that will need to be context based 

## Traits
Each of the mentioned objects will need to be a trait that can be implemented for the real system and yet can be alteratively mocked to test other parts, thus allowing the mini projecft to serve its purpose as a learning tool
