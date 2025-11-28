// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use colored::Colorize;
use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    On,
    Off,
}

#[derive(Debug)]
enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // Add code here to spawn a thread to control the light bulb
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;
        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!(
                            "{}",
                            format!(
                                "Light is {:?} - Color changed to ({}, {}, {})",
                                light_status, r, g, b
                            )
                            .truecolor(r, g, b)
                        );
                    }
                    LightMsg::On => {
                        light_status = LightStatus::On;
                        println!("{}", "Light turned ON".yellow());
                    }
                    LightMsg::Off => {
                        light_status = LightStatus::Off;
                        println!("{}", "Light turned OFF".dimmed());
                    }
                    LightMsg::Disconnect => {
                        light_status = LightStatus::Off;
                        println!("Disconnecting...");
                        break;
                    }
                }
            } else {
                light_status = LightStatus::Off;
                println!("Channel disconnected");
                break;
            }
        }
        light_status
    })
}

fn main() {
    let (s, r) = unbounded();
    let handle = spawn_light_thread(r);

    s.send(LightMsg::On).expect("failed to send On");
    s.send(LightMsg::ChangeColor(255, 0, 0))
        .expect("failed to send ChangeColor");
    s.send(LightMsg::ChangeColor(0, 255, 0))
        .expect("failed to send ChangeColor");
    s.send(LightMsg::Off).expect("failed to send Off");
    s.send(LightMsg::ChangeColor(0, 0, 255))
        .expect("failed to send ChangeColor");
    s.send(LightMsg::Disconnect)
        .expect("failed to send Disconnect");

    let final_status = handle.join().expect("failed to join thread");
    println!("Final status: {:?}", final_status);
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
