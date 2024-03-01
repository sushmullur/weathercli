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
    let result = request_api(city_name).await;
    match result {
        Ok(body) => {
            let data: Value = serde_json::from_str(&body)?;
            
            // Attempt to convert the Value into a HashMap<String, Value>
            if let Some(obj) = data.as_object() {
                let map: HashMap<String, Value> = obj.clone().into_iter().collect();
                if mode == "1" {
                    mode_one(map);
                } else if mode == "2" {
                    mode_two(map);
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

// Mode 1 should display the current temperature and weather description.
fn mode_one(map: HashMap<String, Value>) {
    let weather_object = map.get("list").unwrap().as_array().unwrap();
    let closest_forecast = &weather_object[0].as_object().unwrap();
    let temp = closest_forecast.get("main").unwrap().as_object().unwrap().get("temp").unwrap().as_f64().unwrap() - 273.15;
    let weather_description = closest_forecast.get("weather").unwrap().as_array().unwrap()[0].as_object().unwrap().get("description").unwrap().as_str().unwrap();
    let temp_string = format!("The current temperature is {:.2} °C.", temp);
    let description_string = format!("The weather is currently: {}.", weather_description);
    println!("{}", temp_string);
    println!("{}", description_string);
}

// Weather description - Done
// Temperature, real feel, minimum and max temperature, humidity, and pressure. - Done
// Clouds, wind speed and direction.
// Rain percentage (if it is raining)
fn mode_two(map: HashMap<String, Value>) {
    let weather_object = map.get("list").unwrap().as_array().unwrap();
    let closest_forecast = &weather_object[0].as_object().unwrap();
    let weather_description = closest_forecast.get("weather").unwrap().as_array().unwrap()[0].as_object().unwrap().get("description").unwrap().as_str().unwrap();
    let temp = closest_forecast.get("main").unwrap().as_object().unwrap().get("temp").unwrap().as_f64().unwrap() - 273.15;
    let real_feel = closest_forecast.get("main").unwrap().as_object().unwrap().get("feels_like").unwrap().as_f64().unwrap() - 273.15;
    let temp_min = closest_forecast.get("main").unwrap().as_object().unwrap().get("temp_min").unwrap().as_f64().unwrap() - 273.15;
    let temp_max = closest_forecast.get("main").unwrap().as_object().unwrap().get("temp_max").unwrap().as_f64().unwrap() - 273.15;
    let humidity = closest_forecast.get("main").unwrap().as_object().unwrap().get("humidity").unwrap().as_f64().unwrap();
    let pressure = closest_forecast.get("main").unwrap().as_object().unwrap().get("pressure").unwrap().as_f64().unwrap();
    let rain_chance = closest_forecast.get("pop").unwrap().as_f64().unwrap() * 100.0;
    let wind_object = closest_forecast.get("wind").unwrap().as_object().unwrap();
    let wind_speed = wind_object.get("speed").unwrap().as_f64().unwrap() * 3.6;
    let wind_direction = wind_object.get("deg").unwrap().as_i64().unwrap();
    println!("Weather: {}.", weather_description);
    println!("Temperature: {:.2} °C.\nReal Feel: {:.2} °C.", temp, real_feel);
    println!("Temperature range: {:.2} °C - {:.2} °C.", temp_min, temp_max);
    println!("Humidity: {}%.", humidity);
    println!("Atmospheric pressure: {} hPa.", pressure);
    println!("Chance of rain: {:.0}%.", rain_chance);
    println!("Wind speed: {:.2} km/hr.\nWind direction: {}°.", wind_speed, wind_direction);
}