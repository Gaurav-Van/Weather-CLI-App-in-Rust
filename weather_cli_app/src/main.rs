use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize the JSON Response from openWeatherMap API 
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String
}

// Struct to rep Weather Description 
#[derive(Deserialize, Debug)]
struct Weather{
    description: String
}

// Struct to represent the main weather parameters 
#[derive(Deserialize, Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64
}

//Struct to rep the wind information 
#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64
}

// Function to get the weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url: String = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric", city, country_code, api_key);
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather information 
fn display_weather_info(response: &WeatherResponse){
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;
    let weather_text: String = format!(
        "Weather in {}: {}

            [Temperature is {:.1}°C]
            [Humidity: {:.1}%]
            [Pressure: {:.1} hPa]
            [Wind Speed: {:.1} m/s]",
        response.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed
    );

   let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

fn main() {
    println!("{}", "Welcome to Weather Info!".bright_yellow());
    
    loop
    {
        println!("{}", "Please enter the name of the city:".bright_cyan());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        println!("{}", "Please enter the country code (For example: US for United States):".bright_cyan());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        let api_key = "facc478bc103461c1416d660ca0b851b";

        match get_weather_info(&city, &country_code, api_key){
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error fetching weather info: {} \nThis probably means either the City name or the Country code is incorrect and does not match which is why the value in the response is missing\n", err);
            }
        }

        println!("{}", "Do you want to search for weather in another city ? (yes/no):".bright_red());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the option");
        let input = input.trim().to_lowercase();

        if input == "yes" || input == "y" {
            continue;
        } else {
            break;
        }

     }

}

