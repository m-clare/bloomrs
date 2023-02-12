pub use util::yhash;

pub fn execute_port(port_address: &str, serial_string: &str) {
    let output = serial_string.as_bytes();
    loop {
        if let Ok(mut port) = serialport::new(port_address, 115_200).open() {
            port.write_all(output).expect("Write failed!");
            break;
        }
    }
}

mod util {
    // stolen from https://github.com/eldruin/wyhash-rs
    const P0: u64 = 0xa076_1d64_78bd_642f;
    const P1: u64 = 0xe703_7ed1_a0b4_28db;

    fn wymum(a: u64, b: u64) -> u64 {
        let r = u128::from(a) * u128::from(b);
        ((r >> 64) ^ r) as u64
    }

    // the original "wyrng" mutates its argument, hence the namechange here
    pub fn yhash(val: u64) -> u64 {
        let val = val.wrapping_add(P0);
        wymum(val, val ^ P1)
    }
}
