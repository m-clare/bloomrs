use clap::Parser;
use md4::Md4;
use serialport::{self, SerialPort};
use sha2::Sha256;
use tiger::Tiger;
use digest::Digest;

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
    fn execute_port(port_address: String, serial_string: String) -> () {
        let mut port : Box<dyn SerialPort>;
        loop {
            if let Ok(p) = serialport::new(&port_address, 115_200).open() {
                port = p;
                break;
            }
        }
        let output = serial_string.as_bytes();
        port.write(output).expect("Write failed!");
        drop(port);
    }

    let args = Args::parse();
    let hashstring = args.directory + " " + &args.command;
    let color = args.tint;
    let board_size = 32;
    let mut serial_string = String::from("");

    for byte in [
        Sha256::new().chain_update(&hashstring).finalize().last().unwrap(),
        Md4::new().chain_update(&hashstring).finalize().last().unwrap(),
        Tiger::new().chain_update(&hashstring).finalize().last().unwrap(),
    ] {
        let value = byte % &board_size;
        serial_string.push_str(&color.as_str());
        serial_string.push_str(&value.to_string());
        serial_string.push_str(",");
    }

    // lazy get rid of trailing comma
    serial_string.pop();
    serial_string.push_str("\r\n");

    println!("{}", serial_string);
    execute_port(args.serialport, serial_string)
}
