use sha2::{Sha256, Digest};
use md4::Md4;
use tiger::Tiger;
use serialport;
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
    fn execute_port(port_address: String, serial_string: String) -> () {
    let mut port = serialport::new(port_address, 115_200).open().expect("Failed to open port");
    let output = serial_string.as_bytes();
    port.write(output).expect("Write failed!");
    drop(port);
    }

    let args = Args::parse();
    let hashstring = args.directory + " " + &args.command;
    let color = args.tint;
    let board_size = 32;

    // can this be consolidated ??
    let sha2hasher = Sha256::new().chain_update(&hashstring).finalize();
    let md4hasher = Md4::new().chain_update(&hashstring).finalize();
    let tigerhasher = Tiger::new().chain_update(&hashstring).finalize();

    let sha2mod = sha2hasher.get(&sha2hasher.len()-1).expect("number should be an int between 0 and 255") % &board_size;
    let md4mod = md4hasher.get(&md4hasher.len()-1).expect("number should be an int between 0 and 255") % &board_size;
    let tigermod = tigerhasher.get(&tigerhasher.len()-1).expect("number should be an int between 0 and 255") % &board_size;

    let bit_vector = vec![sha2mod, md4mod, tigermod];
    let mut serial_string = String::from("");

    for value in &bit_vector {
        serial_string.push_str(&color.as_str());
        serial_string.push_str(&value.to_string());
        serial_string.push_str(",");
    }

    serial_string.pop();

    serial_string.push_str("\r\n");

    println!("{}", serial_string);
    execute_port(args.serialport, serial_string)
}
