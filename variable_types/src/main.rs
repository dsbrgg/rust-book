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

    // floating points
    // Rust has two primitive floating points
    // f32  && f64
    // default type is f64 since it's roughly
    // the same speed as f32 but with more precision
    
    let _x_f64 = 2.0;  // -> f64
    let _x_f32: f32 = 3.0;  // -> f32

    // boolean
    // booleans have one byte in size

    let _is_true = true;
    let _is_false: bool = false;

    // char type
    // char is the most primitive alphabetic type in Rust
    // character literals denoted with single quotes (eg. '')
    // string literals arev denoted with double quotes (eg. "")
    // it represents a unicode scalar value that ranges from
    // U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    println!("From unicode: \u{00DF}");

    // tuples
    // tuples are a way of grouping together values
    // with different types into one compound type
    // once declared, they can't grow or shrink in size

    // declaring instantly
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // accessing tuple through index

    let _index = _tup.2;

    println!("Last tuple element is: {}", _index);

    // destructuring tuple
    let tup2 = (400, 6.9, 2);

    let (_x, _y, _z) = tup2;

    println!("The value of y is: {}", _y);

    // arrays
    // arrays in Rust must have elements of the same type
    // like tuples, they must also are of fixed length
    // arrays are good when you want your information
    // allocated on the stack rather than on the heap
    // or when you want to ensure you always 
    // have a fixed number of elements

    let _array = [ 1, 2, 3, 4, 5 ];

    // declaring with [ type, length ]
    let _array_2: [ i32; 5 ] = [ 6, 7, 8, 9, 10 ];

    // When accessing index out of bounds in the array
    // Rust will panick and throw an error because of
    // its safety principles. Other low level
    // programming languages will allow to access invalid memory
}
