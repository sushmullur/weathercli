use std::env;
extern crate reqwest;

pub async fn process_command(args: core::str::SplitWhitespace<'_>) -> Result<(), reqwest::Error> {
    let args: Vec<&str> = args.collect();
    if args.len() != 2 {
        println!("Usage: weather <city> <country>");
        return Ok(());
    }
    println!("Getting weather for {}, {}", args[0], args[1]);
    let city_name = args[0].to_string();
    let result = request_api(city_name).await;
    match result {
        Ok(body) => {
            println!("{}", body);
            Ok(()) // Return success
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(err) // Propagate the error
        }
    }
}

async fn request_api(city_name: String) -> Result<String, reqwest::Error> {
    let api_url = env::var("API_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();
    let id = env::var("ID").unwrap();

    let url = format!("{}q={}&id={}&appid={}", api_url, city_name, id, api_key);
    let body = reqwest::get(url).await?.text().await?;

    println!("{}", body);

    Ok(body)
}