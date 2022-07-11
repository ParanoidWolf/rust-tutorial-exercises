fn main() {
    // substitute {} with the given Variable
    println!("I'm learning {}!!", "Rust");

    // position arguments to access different data
    println!("{0} and {1} are two new languages which I'm learning for developing CLI tools. {1} is so much easier to write. But {0} is better for performance", "rust", "go");

    // named arguments
    println!("{Moz} is developing Rust now and {Goog} is developing Go", Moz = "Mozilla", Goog = "Google");

    // format characted argument
    println!("Base 10 : {}", 69420);
    println!("Base 2 : {:b}", 69420);
    println!("Base 8 : {:o}", 69420);
    println!("Base 16 : {:x}", 69420);
    println!("Base 16 : {:X}", 69420);

    // right align white space
    println!("{number:>5}", number = 44);
    // fill indent with zeroes
    println!("{number:0>5}", number = 44);
    // named argument for white space
    println!("{number:0>width$}", number = 44, width = 5);
}
