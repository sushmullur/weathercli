pub fn process_command(args: core::str::SplitWhitespace<'_>) {
    let args: Vec<&str> = args.collect();
    if args.len() != 2 {
        println!("Usage: convert <quantity> <new unit>");
        return;
    }

    let quantity = args[0].to_string();
    let unit = args[1].to_string();

    println!("Converting {} into {}", quantity, unit);
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    return 0.0;
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    return 0.0;
}