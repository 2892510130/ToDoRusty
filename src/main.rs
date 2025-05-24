use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // The first argument is the program name
    println!("Program name: {}", args[0]);
    
    // Iterate through arguments
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
    
    // Access specific arguments
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    }
}
