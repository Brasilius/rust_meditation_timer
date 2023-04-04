use std::process::exit;
use std::thread;
use std::time::Duration;
use std::io;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, Sink};
fn main() {
    println!("Hello, user! Welcome to meditation timer!");
    println!("Please enter the number of minutes you would like to meditate for in minutes: ");
    let mut timer_for_meditate_input = String::new();
    io::stdin().read_line(&mut timer_for_meditate_input)
        .expect("Failed to read line");
    let timer: i32 = timer_for_meditate_input.trim().parse().unwrap();
    println!("You have entered: {}", timer);
    //pass ownership of timer to timer_countdown function
    timer_countdown(timer);
    let file = File::open("src/vine-boom.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    sink.append(source);
    sink.play();
}
fn timer_countdown(input: i32){
    // Path: src/timer_countdown.rs
    // i32 is the number of minutes to meditate for
    // i32 is passed from main.rs
let seconds = input * 60;
thread::sleep(Duration::from_secs(seconds as u64));
println!("Your meditation timer has finished!");
exit(0);
}
