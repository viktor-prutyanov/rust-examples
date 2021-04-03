const USAGE: &str = "Usage: adder <arg1> <arg2>";

fn add(a: i8, b: i8) -> i8 {
    a + b
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("Arguments are {:?}", args);

    if args.len() != 3 {
        eprintln!("Invalid number of arguments!");
        eprintln!("{}", USAGE);
        std::process::exit(-1);
    }

    let a = match args[1].parse::<i8>() {
        Err(e) => { panic!("Error in 1st operand: {}", e)},
        Ok(a) => a
    };

    let b = match args[2].parse::<i8>() {
        Err(e) => { panic!("Error in 2nd operand: {}", e)},
        Ok(b) => b
    };

    println!("{0} + {1} = {2}", a, b, add(a, b));
}
