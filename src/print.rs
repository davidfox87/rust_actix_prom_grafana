pub fn run() {
    println!("Hello from the print.rs file!");

    println!("{} is age {}", "David", 34);

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}",
         "David", "Nottingham", "code");

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name="David",
        activity="guitar"
    );

    // Placeholder for debugging
    println!("{:?}", (12, true, "hello"));

    println!("10+10={}", 10+10);
}

