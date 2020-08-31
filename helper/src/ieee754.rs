pub fn main() {
    let x: u32 = 0xC1440000;

    println!("{:08X} = {:032b}", x, x);

    let sign = x >> 31; // 1 bit
    let exponent = ((x & 0x7FFFFFFF) >> 23) - 127; // 8 bits
    let fraction = x & 0b01111111_11111111_11111111; // 23 bits

    println!(
        "Sign: {:0}, exponent: {}, fraction = 1.{:023b}",
        sign, exponent, fraction
    );
}
