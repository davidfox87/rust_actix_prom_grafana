pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 100;
    println!("{:?}", numbers);

    println!("Single value: {}", numbers[0]);

    println!("Array length: {}", numbers.len());

    // get a slice from an array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}