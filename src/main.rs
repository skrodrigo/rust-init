fn main() {
    let number: i8 = 8;
    let number2: i32 = 32;
    let number3: i64 = 64;
    let number4: i128 = 128;
    let number5: isize = -1;
    let number6: usize = 1;
    let is_valid: bool = true || false;
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

    println!("Hello, world! my numbers is {}, {}, {}, {}, {}, {}, and my bool is {}, and my tuple is {}", 
    number, 
    number2,
    number3,
    number4,
    number5,
    number6,
    is_valid,
    my_tuple.1
    );
}
