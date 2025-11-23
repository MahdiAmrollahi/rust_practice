// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
// * Use a loop to continuously prompt the user for input until they enter

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Turning off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    println!("Enter a power command:");
    loop {
        let mut input = String::new();
        let status = io::stdin().read_line(&mut input);
        
        if status.is_ok() {
            let input = input.trim().to_lowercase();
            
            // Exit loop if user enters empty string (optional, but good for UX)
            if input == "" {
                continue;
            }

            let state = match input.as_str() {
                "off" => Some(PowerState::Off),
                "sleep" => Some(PowerState::Sleep),
                "reboot" => Some(PowerState::Reboot),
                "shutdown" => Some(PowerState::Shutdown),
                "hibernate" => Some(PowerState::Hibernate),
                _ => None,
            };

            match state {
                Some(s) => {
                    print_power_action(s);
                    // Breaking on shutdown/off seems appropriate for a "power" simulator
                    // but the prompt cut off "until they enter". 
                    // I'll assume we just keep looping unless it's a terminal state, 
                    // or just loop forever as per "continuously prompt".
                    // However, usually "Off" implies stopping.
                    if input == "off" || input == "shutdown" {
                        break;
                    }
                }
                None => println!("Invalid command"),
            }
        } else {
            break;
        }
    }
}
