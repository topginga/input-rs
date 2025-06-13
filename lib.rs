//! The 'input!()' macro acts like 'input()' from python to easily gather user input.

/// Processes user input and returns it as a String.
/// Enter text within the parameters to print that text before receiving input.
#[macro_export]
macro_rules! input {
() => {{
    use::std::io;
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading line");
    user_input.trim().to_string()
}};

($display:expr) => {{
    use::std::io;
    println!("{}", $display);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading line");
    user_input.trim().to_string()
}};
}
