use serde_json::Value;
use std::env;
use std::collections::HashMap;
extern crate reqwest;

pub async fn process_command(args: core::str::SplitWhitespace<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = args.collect();
    if args.len() != 2 {
        println!("Usage: weather <city> <mode>");
        return Ok(());
    }
    println!("Getting weather for {}", args[0]);
    let city_name = args[0].to_string();
    let mode = args[1].to_string();
    // TODO: Parse depending on mode
    let result = request_api(city_name).await;
    match result {
        Ok(body) => {
            let data: Value = serde_json::from_str(&body)?;
            
            // Attempt to convert the Value into a HashMap<String, Value>
            if let Some(obj) = data.as_object() {
                let map: HashMap<String, Value> = obj.clone().into_iter().collect();
                if mode == "1" {
                    mode_one(map);
                }
            } else {
                println!("Invalid response.");
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

fn mode_one(map: HashMap<String, Value>) {
    let weather_object = map.get("list").unwrap().as_array().unwrap();
    let closest_forecast = &weather_object[0].as_object().unwrap();
    let closest_temperature_object = closest_forecast.get("main").unwrap().as_object().unwrap().get("temp");
    let temp_value = match closest_temperature_object {
        Some(temp_value) => temp_value.as_f64().unwrap_or(0.0),
        None => 0.0,
    } - 273.15;
    let temp_string = format!("The current temperature is {:.2} °C", temp_value);
    println!("{}", temp_string);
}
