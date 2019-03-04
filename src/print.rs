pub fn run() {
    // Basic formatting
    println!("{} is from {}", "Mayushi", "Japan");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named arguments
    println!("{name} likes to play {activity}", name ="John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
