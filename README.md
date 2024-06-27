# Python's like `input` in rust

This Rust library provides a macro that acts as an analog of Python's `input` function. It allows reading a line of input from standard input (stdin), with an optional prompt.

## Usage

```rust
use pyinput::input;

fn main() {
    // Using a string literal as the prompt
    let input_string = input!("Enter something: ").unwrap();
    println!("You entered: {}", input_string);

    // Using a variable that implements Display as the prompt
    let custom_prompt = String::from("Custom prompt: ");
    let input_string = input!(custom_prompt).unwrap();
    println!("You entered: {}", input_string);

    // Without any prompt
    println!("Enter something without a prompt:");
    match input!() {
        Ok(input_string) => println!("You entered: {}", input_string),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
