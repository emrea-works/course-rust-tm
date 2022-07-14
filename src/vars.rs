pub fn run() {
    let name = "Brad";
    //let age = 37;
    // age = 38 // error: cannot assign twice to immutable variable.
    let mut age = 37;
    age = 38; // warns that first value won't be read.
              // which is good, that means an unnecessarry variable used
    println!("My name is {}, and my age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
