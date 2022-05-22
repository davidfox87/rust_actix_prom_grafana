pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 100;
    // numbers.push(5);
    // numbers.push(6);
    // numbers.pop();

    println!("{:?}", numbers);

    println!("Single value: {}", numbers[0]);

    println!("Vector length: {}", numbers.len());

    // get a slice from an array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}