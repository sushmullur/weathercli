pub fn process_command(args: core::str::SplitWhitespace) {
    let args: Vec<&str> = args.collect();
    if args.len() != 2 {
        println!("Usage: weather <city> <country>");
        return;
    }
    println!("Getting weather for {}, {}", args[0], args[1]);
}