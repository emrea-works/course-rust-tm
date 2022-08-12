use std::env;

pub fn run() {
    /* args: variable is a vector data (infinite length array)
     * that get assigned by env library and its methods which
     * will be called at execution time, which collects the
     * arguments that were given, just like as in array's push
     */
    let args: Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);

    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
