use serde_json::Value;
use std::env;
use std::collections::HashMap;
extern crate reqwest;

pub async fn process_command(args: core::str::SplitWhitespace<'_>) -> Result<(), Box<dyn std::error::Error>> {
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
            let data: Value = serde_json::from_str(&body)?;
            
            // Attempt to convert the Value into a HashMap<String, Value>
            if let Some(obj) = data.as_object() {
                let map: HashMap<String, Value> = obj.clone().into_iter().collect();
                // Use the map as needed
                let weather_data = map.get("list");
                println!("{:?}", weather_data);
            } else {
                println!("The received JSON is not an object.");
            }
            
            Ok(())
        },
        Err(err) => {
            Err(err.into())
        }
    }
}

async fn request_api(city_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = env::var("API_URL")?;
    let api_key = env::var("API_KEY")?;
    let id = env::var("ID")?;

    let url = format!("{}q={}&id={}&appid={}", api_url, city_name, id, api_key);
    let body = reqwest::get(&url).await?.text().await?;

    Ok(body)
}
