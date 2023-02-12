use bloorm::{execute_port, yhash};
use clap::Parser;

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

fn main() {
    let args = Args::parse();
    let hashstring = args.directory + " " + &args.command;
    let color = args.tint;
    let board_size = 32;
    let mut serial_string = String::with_capacity(128);

    let hashsum: u64 = hashstring.as_bytes().iter().map(|b| *b as u64).sum();

    for offset in 0..3 {
        let value = yhash(hashsum + offset) % board_size;
        serial_string.push_str(color.as_str());
        serial_string.push_str(&value.to_string());
        serial_string.push(',');
    }

    // lazy get rid of trailing comma
    serial_string.pop();
    serial_string.push_str("\r\n");

    execute_port(&args.serialport, &serial_string)
}
