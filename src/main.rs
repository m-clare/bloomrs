use clap::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// Program to determine LEDs to light up
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Serial port for neotrellis
    #[arg(short, long)]
    serialport: String,

    /// LED color to indicate progress/success/failure
    #[arg(short, long)]
    tint: String,

    /// Working directory
    #[arg(short, long)]
    directory: String,

    /// Command
    #[arg(short, long)]
    command: String,
}

fn execute_port(port_address: &str, serial_string: &str) {
    let output = serial_string.as_bytes();
    loop {
        if let Ok(mut port) = serialport::new(port_address, 115_200).open() {
            port.write_all(output).expect("Write failed!");
            break;
        }
    }
}

fn main() {
    let args = Args::parse();
    let hashstring = args.directory + " " + &args.command;
    let color = args.tint;
    let board_size = 32;
    let mut serial_string = String::with_capacity(128);

    for offset in 0..3 {
        let mut hasher = DefaultHasher::default();
        hasher.write(hashstring.as_bytes());
        let value = (hasher.finish() + offset) % board_size;

        serial_string.push_str(color.as_str());
        serial_string.push_str(&value.to_string());
        serial_string.push(',');
    }

    // lazy get rid of trailing comma
    serial_string.pop();
    serial_string.push_str("\r\n");

    execute_port(&args.serialport, &serial_string)
}
