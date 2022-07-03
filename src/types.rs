pub fn run() {
    // Default integer is "i32"
    let x = 1;
    // Default is "f64"
    let y = 2.5;

    // Add elpicit type
    let z: i64 = 1493204975987;
    // Find max size
    println!("MAX i32: {}", std::i32::MAX);
    println!("MAX i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    let is_grater = 10 < 5;

    println!("{:?}", (x, y, z, is_active, is_grater));

    // Char
    let a1 = 'a'; // if single quotes then it is a char
                  //let a2 = 'ab'; // syntax error
    let a3 = '\u{1F600}';

    //println!("a1: {}, a2: {}", a1, a2); // throws error:
    println!("a1: {}, a3: {}", a1, a3); // throws error:
}
