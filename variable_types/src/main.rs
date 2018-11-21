fn main() {
    // infering types
    // when many types are possible, data type must be annotated
    let test: u32 = "42".parse().expect("Not a number!");

    println!("Parsed correctly: {}", test);

    // integer types
    // signed numbers can store ( -(2n - 1) to 2n - 1 - 1 )
    // eg. i8 =  -(27) to 27 - 1 = -128 to 127
    // eg. u8 = 0 to 28 - 1 = 0 to 255
    // ---------------------------------
    // isize integer type depens on computer architecture
    // 64 bits = 64-bit architecture
    // ---------------------------------
    // integer literals can be
    // - Decimal: 10_000
    // - Hex: 0xff
    // - Octal: 0o77
    // - Binary: 0b1111_0000
    // - Byte(u8 only): b'A'
    // ---------------------------------
    // Rust defaults integers to i32
    // pro-tip: isize/usize are usedwhen indexing collections
    // ---------------------------------

    let integer8: i8 = -0b0000_1111;
    let integer16: i16 = 30_000;
    let integer32: i32 = -0o777;
    let integer_arch: isize = 0xffffff;
    let bin_u8: u8 = b'A';

    println!("Integer 8: {}", integer8);
    println!("Integer 16: {}", integer16);
    println!("Integer 32: {}", integer32);
    println!("Isize: {}", integer_arch);
    println!("bin_u8: {}", bin_u8);
}
