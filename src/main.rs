// use sha2::{Sha256, Digest};
use serialport;

fn main() {
    // println!("Hello, world!");
    // let mut hasher = Sha256::new();
    // let data = b"Hello world!";
    // hasher.update(data);
    // // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    // hasher.update("String data");
    // // Note that calling `finalize()` consumes hasher
    // let hash = hasher.finalize();
    // println!("Binary hash: {:?}", hash);

    let mut port = serialport::new("/dev/tty.usbmodem3101", 115_200).open().expect("Failed to open port");
    let output = "r1,r18\n\n".as_bytes();
    port.write(output).expect("Write failed!");
}
