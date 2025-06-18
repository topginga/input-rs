//! Adds many helpful macros for various crates
//! Currently has standard, rdev, clearscreen, and rand macros

/// Processes user input and returns it as a String.
/// Enter text within the parameters to print that text before receiving input.

// STOCK MACROS

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
macro_rules! loop_for_secs {
    ($time:expr, $func:block) => {
        let time = $time as f64;
        let start = std::time::Instant::now();
        let duration = std::time::Duration::from_secs_f64(time);
        loop {
            if start.elapsed() >= duration {
                break;
        }
        $func
        }
    };
}

// RDEV MACROS

/// Imports rdev dependencies for macros
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! import_rdev {
    () => {
        use rdev::*;
        use rdev::EventType::*;
        use rdev::Key::*;
    };
}

/// Presses a specific key
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! press_key {
    ($key:ident) => {
        rdev::simulate(&rdev::EventType::KeyPress(rdev::Key::$key)).expect("Error pressing key");
    };
}

/// Releases a specific key
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! release_key {
    ($key:ident) => {
        rdev::simulate(&rdev::EventType::KeyRelease(rdev::Key::$key)).expect("Error releasing key");
    };
}

/// Starts the rdev input listener
/// Pass the name of the target function as an argument
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! start_input_listener {
    ($target_function:ident) => {
        std::thread::spawn(|| {
            if let Err(error) = rdev::listen($target_function) {
                println!("Error: {error:?}");
            }
        });
    };

    ($target_function:ident, $var1:expr) => {
        std::thread::spawn(|| {
            if let Err(error) = rdev::listen(move |event: Event| detect_input(event, $var1)) {
                println!("Error: {error:?}");
            }
        });
    };

    ($target_function:ident, $var1:expr, $var2:expr) => {
        std::thread::spawn(|| {
            if let Err(error) = rdev::listen(move |event: Event| detect_input(event, $var1, $var2)) {
                println!("Error: {error:?}");
            }
        });
    };

    ($target_function:ident, $var1:expr, $var2:expr, $var3:expr) => {
        std::thread::spawn(|| {
            if let Err(error) = rdev::listen(move |event: Event| detect_input(event, $var1, $var2, $var3)) {
                println!("Error: {error:?}");
            }
        });
    };

    ($target_function:ident, $var1:expr, $var2:expr, $var3:expr, $var4:expr) => {
        std::thread::spawn(|| {
            if let Err(error) = rdev::listen(move |event: Event| detect_input(event, $var1, $var2, $var3, $var4)) {
                println!("Error: {error:?}");
            }
        });
    };
}

/// Quickly presses then releases a specific key
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! click_key {
    ($key:ident) => {
        rdev::simulate(&rdev::EventType::KeyPress(rdev::Key::$key)).expect("Error pressing key");
        std::thread::sleep(std::time::Duration::from_secs_f64(0.01));
        rdev::simulate(&rdev::EventType::KeyRelease(rdev::Key::$key)).expect("Error releasing key");
    };
}

/// Moves the mouse relatively from it's current position
/// Only works in 3d games, will not move the cursor
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! move_mouse_rel {
    ($x:expr, $y:expr) => {
        simulate(&rdev::EventType::MouseMoveRelative { delta_x: $x, delta_y: $y }).expect("Error moving mouse");
    };
}

/// Moves the mouse to a specific set of coordinates
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! move_mouse_to {
    ($x:expr, $y:expr) => {
        simulate(&rdev::EventType::MouseMove { x: $x, y: $y }).expect("Error moving mouse");
    };
}

/// Presses then releases a specified mouse button
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! click_mouse {
    ($button:ident) => {
        rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::$button)).expect("Error pressing button");
        std::thread::sleep(std::time::Duration::from_secs_f64(0.01));
        rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::$button)).expect("Error releasing button");
    };
}

/// Presses down a specified mouse button
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! press_mouse {
    ($button:ident) => {
        rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::$button)).expect("Error pressing button");
    };
}

/// Releases a specific mouse button
#[cfg(feature = "rdev")]
#[macro_export]
macro_rules! release_mouse {
    ($button:ident) => {
        rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::$button)).expect("Error releasing button");
    };
}

// RAND MACROS

/// Imports the necessary rand dependencies
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! import_rand {
    () => {
        use rand::Rng;
    };
}

/// Generates a random number from a given range
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! random_num {
    ($low:expr, $high:expr) => {
        rand::rng().random_range($low..=$high)
    };
}

// CLEARSCREEN MACROS

/// Clears the contents of the terminal
#[cfg(feature = "clearscreen")]
#[macro_export]
macro_rules! clear {
    () => {
        clearscreen::clear().expect("Failed to clear screen");
    };
}
