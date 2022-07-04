pub fn run() {
    let mut hello = String::from("Hello");
    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    hello = String::from("Hello ");
    //hello = "Hello "; // not av for push or push_Str
    // Pushing char
    hello.push('W'); // cos push is for adding char types

    // 'o' character causes syntax error
    // hello.push('Wo');
    // Adding string
    hello.push_str("orld!");
    println!("{}", hello);
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is empty ?
    println!("Is empty? {}", hello.is_empty());

    // Contains
    println!("Contains 'World'? {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(3, s.len()); // assertion fails
    // if you wanna pass wihout an error, assertions must be true
    // that means if the right and left vallue is equal in the 
    // assertion, will be no output, just passes
    // So assertions are useful for error handling like milestones.



    println!('{}', s);
}
