pub fn main(hex: u32) {
    println!("{:08X} = {:032b}", hex, hex);

    let sign = (hex >> 31) as u8; // 1 bit
    let exponent = (((hex & 0x7FFFFFFF) >> 23) as i16 - 127) as i8; // 8 bits
    let fraction = hex & 0b01111111_11111111_11111111; // 23 bits

    println!(
        "Sign: {:0}, exponent: {}, fraction = 0b1.{:023b}",
        sign, exponent, fraction
    );
}
