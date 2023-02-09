use sha2::{Sha256, Digest};
use md4::Md4;
use tiger::Tiger;
// use serialport;
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
    //     fn execute_port() {
//     let mut port = serialport::new("/dev/tty.usbmodem3101", 115_200).open().expect("Failed to open port");
//     port.set_timeout(Duration::new(2,0)).expect("setting timeout failed!");
//     let output = "r0,r1,r2,r3,r4\r\n".as_bytes();
//     port.write(output).expect("Write failed!");
//     drop(port)
//     }
//
    let args = Args::parse();

    let hashstring = args.directory + " " + &args.command;
    println!("{}", hashstring);

    println!("{}", args.serialport);
    println!("{}", args.tint);
    println!("{}", args.command);

    let sha2hash = Sha256::new().chain_update(hashstring.clone()).finalize();
    let md5hash = Md4::new().chain_update(hashstring.clone()).finalize();
    let tigerhash = Tiger::new().chain_update(hashstring.clone()).finalize();
    println!("Binary hash: {:?}", sha2hash);
    println!("Binary hash: {:?}", md5hash);
    println!("Binary hash: {:?}", tigerhash);
}
