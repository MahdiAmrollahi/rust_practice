// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

/// This function returns a static string slice after a delay
///
/// # Returns
/// A static reference to a string slice "Hello, "
fn msg_hello() -> &'static str {
    // Import the Duration struct from std::time module
    use std::time::Duration;
    // Make the current thread sleep for 1000 milliseconds (1 second)
    std::thread::sleep(Duration::from_millis(1000));
    // Return the static string "Hello, "
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let msg1 = thread::spawn(move || msg_hello());
    let msg2 = thread::spawn(move || msg_thread());
    let msg3 = thread::spawn(move || msg_excited());
    let (msg1, msg2, msg3) = (
        msg1.join().unwrap(),
        msg2.join().unwrap(),
        msg3.join().unwrap(),
    );
    println!("{} {} {}", msg1, msg2, msg3);
}
