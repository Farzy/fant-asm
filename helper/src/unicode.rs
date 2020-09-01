pub fn main() {
    let chars: Vec<u8> = vec![0xA0, 0x15, 65, 0xFE, 0x18, 65, 0xC3, 0xAA, 0x7C, 98];
    let s = String::from_utf8_lossy(&chars);

    println!("{:X?} = '{}'", chars, s);

    let s = "é œ ê ô € ñ";
    println!(
        "'{}' ({}) = {:X?} ({})",
        s,
        s.chars().count(),
        s.as_bytes(),
        s.len()
    );
}
