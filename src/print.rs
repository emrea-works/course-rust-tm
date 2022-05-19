pub fn run() {
    // Print Console
    println!("Hello from print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Mark",
        activity = "Football"
    );

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait, Tuple
    println!("{:?}", (12, true, "Text"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
