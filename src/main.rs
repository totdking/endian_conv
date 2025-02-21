// fn main() {
//     let input: u64 = 0x0A00000000000000;

//     // Interpreting as Big-Endian (default human reading)
//     println!("Big-Endian Interpretation: {}", input);

//     // Convert to bytes (Little-Endian)
//     let bytes = input.to_le_bytes(); // Convert to little-endian byte array
//     let le_value = u64::from_le_bytes(bytes); // Convert back to u64

//     println!("Little-Endian Interpretation: {}", le_value);
// }

// fn main() {
//     let hex_str = "0x0A00000000000000"; // OpenVM input

//     // Parse hex as a big-endian u64
//     let big_endian_value = u64::from_str_radix(&hex_str[2..], 16).expect("Invalid hex input");

//     // Convert to little-endian by swapping bytes
//     let little_endian_value = big_endian_value.to_le(); // Ensures proper endian conversion

//     println!("Big-Endian Interpretation: {}", big_endian_value);
//     println!("Little-Endian Interpretation: {}", little_endian_value);
//     // assert_eq!(little_endian_value, 10);
// }

// use byteorder::{ByteOrder, LittleEndian};
// fn main() {
//     let number: u32 = 10;
//     // Convert to little-endian bytes
//     let mut little_endian_bytes = [0; 4];
//     LittleEndian::write_u32(&mut little_endian_bytes, number);

//     println!("Little-endian bytes: {:?}", little_endian_bytes);

//     // Convert back from little-endian bytes to integer
//     let restored_number = LittleEndian::read_u32(&little_endian_bytes);

//     println!("Restored number: {}", restored_number);
// }

use byteorder::{ByteOrder, LittleEndian};

fn main() {
    let number: u64 = 2_000_000_000; // Example number

    // Convert to little-endian bytes
    let mut le_bytes = [0; 8]; // u64 is 8 bytes
    LittleEndian::write_u64(&mut le_bytes, number);

    // Convert bytes to hex string
    let hex_string = format!("0x{}", le_bytes.iter().map(|b| format!("{:02X}", b)).collect::<String>());

    println!("Hex string (little-endian): {}", hex_string);

}