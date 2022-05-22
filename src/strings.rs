pub fn run(){
    let mut hello = String::from("hello ");

    println!("{}", hello);

    println!("Length of string is {}", hello.len());

    hello.push_str("World");

    println!("{}", hello.contains("World"));
    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word);
    }
}   