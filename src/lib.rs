use std::fmt::Display;
use std::io::{self, BufRead, Write};

/// Analog of python's input. <br>
/// Macros read line from stdin. <br>
/// A prompt can be anything that has a trait Display implemented. <br>
/// ```rust
/// let input_string = input!("your_prompt").unwrap(); // unwrap to get string
/// let input_string = input!(display_impl_variable).unwrap();
/// // Or
/// match input!() {
///     Ok(input_string) => println!("{}", input_string);
///     Err(e) => println!("{}", e);
/// }
/// ```
#[macro_export]
macro_rules! input {
    ($prompt: literal) => {
        $crate::prompt_input($prompt)
    };
    ($prompt: expr) => {
        $crate::prompt_input($prompt)
    };
    () => {
        $crate::base_input()
    };
}

pub fn prompt_input(prompt: impl Display) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    base_input()
}

pub fn base_input() -> io::Result<String> {
    let mut input = String::new();
    let _ = io::stdin().lock().read_line(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    Ok(input)
}
