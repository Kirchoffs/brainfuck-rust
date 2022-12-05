# brainfuck-rust

## Definition
From Wikipedia
| Brainfuck command      | C equivalent                                          |
| ---------------------- | ----------------------------------------------------- |
| (Program Start)        | `char array[30000] = {0};` <br/> `char *ptr = array;` |
| >                      | `ptr++;`                                              |
| <                      | `ptr--`                                               |
| +                      | `*ptr++;`                                             |
| -                      | `*ptr--;`                                             |
| .                      | `putchar(*ptr);`                                      |
| ,                      | `*ptr = getchar()`                                    |
| [                      | `while (*ptr) {`                                      |
| ]                      | `}`                                                   |


## Run the code
Install and use nightly version
```
>> rustup toolchain list
>> rustup toolchain install nightly
>> rustup default nightly
```

Run the code
```
>> cargo build
or
>> cargo build --release

>> cargo run --bin vanilla -- resources/hello_world.bf
>> cargo run --bin ir -- resources/hello_world.bf
>> cargo run --bin jit -- resources/hello_world.bf
or
>> cargo run --bin jit --release -- resources/mandelbrot.bf
```