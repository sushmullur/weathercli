pub fn process_command() {
    println!("Usage: weather <city> <mode>");
    println!("These are the available weather modes: ");
    println!("1: General temperature and weather");
    println!("2: Advanced temperature and stats");
    println!("3: Hourly forecast");
    println!("All units are in the metric system.");
    println!("To convert, enter: convert <quantity> <new unit>");
    println!("Example: convert 23.5 fahrenheit");
}