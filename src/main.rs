use std::result;

mod usb;

const MAXIMUM_AS_BYTES: [u8; 4] = f32::MAX.to_le_bytes();
const MINIMUM_AS_BYTES: [u8; 4] = f32::MIN.to_le_bytes();

fn main() -> () {}

fn decode_command(command: [String; 3]) -> [[u8; 8]; 2] {
    match command[0].as_str() {
        "sthm" => set_home(),
        _ => panic!(""),
    }
}

fn set_home() -> [[u8; 8]; 2] {
    [
        protected_to_long(&MAXIMUM_AS_BYTES, &MAXIMUM_AS_BYTES), // Enter "Special Mode"
        [0u8; 8],
    ]
}

fn go_to(x: f32, y: f32) -> [[u8; 8]; 2] {
    [
        protected_to_long(&x.to_le_bytes(), &y.to_le_bytes()),
        [0u8; 8],
    ]
}

fn toolhead_up() -> [[u8; 8]; 2] {
    [
        protected_to_long(&MAXIMUM_AS_BYTES, &MAXIMUM_AS_BYTES), // Enter "Special Mode"
        protected_to_long(&MAXIMUM_AS_BYTES, &MAXIMUM_AS_BYTES), // Send "High Command"
    ]
}

fn toolhead_down() -> [[u8; 8]; 2] {
    [
        protected_to_long(&MAXIMUM_AS_BYTES, &MAXIMUM_AS_BYTES), // Enter "Special Mode"
        protected_to_long(&MINIMUM_AS_BYTES, &MINIMUM_AS_BYTES), // Send "Low Command"
    ]
}

fn protected_to_long(a: &[u8; 4], b: &[u8; 4]) -> [u8; 8] {
    let mut result = [0u8; 8];
    result[..4].copy_from_slice(a);
    result[4..].copy_from_slice(b);
    result
}

fn long_to_protected(a: &[u8; 8]) -> [[u8; 4]; 2] {
    let mut result_zero = [0u8; 4];
    let mut result_one = [0u8; 4];

    result_zero.copy_from_slice(&a[..4]);
    result_one.copy_from_slice(&a[4..]);

    [result_zero, result_one]
}
// Commands
// ---
// STHM - SeT HoMe
// GT - Go To
// THUP - ToolHead UP
// THDN - ToolHead DowN
