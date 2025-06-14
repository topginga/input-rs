//! Adds many helpful macros to speed up development and reduce imports

/// Processes user input and returns it as a String.
/// Enter text within the parameters to print that text before receiving input.
#[macro_export]
macro_rules! input {
() => {{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error reading line");
    user_input.trim().to_string()
}};

($display:expr) => {{
    println!("{}", $display);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error reading line");
    user_input.trim().to_string()
}};
}

/// Sleeps for a specific duration of time, measured in seconds.
#[macro_export]
macro_rules! sleep {
    ($time:expr) => {
        let time = $time as f64;
        std::thread::sleep(std::time::Duration::from_secs_f64(time));
    };
}

/// Creates a loop which runs for a specified duration of time in seconds.
/// First parameter is the time, 2nd parameter is the code to loop.
/// Be sure to put the code you intent to loop inside a block.
#[macro_export]
macro_rules! loop_for {
    ($time:expr, $func:block) => {
        let start = std::time::Instant::now();
        let duration = std::time::Duration::from_secs($time);
        loop {
            if start.elapsed() >= duration {
                break;
        }
        $func
        }
    };
}
