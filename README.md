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
| [                      | `while (*pre) {`                                      |
| ]                      | `}`                                                   |


## Run the code
```
>> cargo build
or
>> cargo build --release

>> cargo run -- resources/hello_world.bf
```