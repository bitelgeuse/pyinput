// MIT License
//
// Copyright (c) 2024 Kostya Tatoshin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
