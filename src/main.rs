const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

use std::fmt::Display;

use dotenvy::var;
use reqwest::blocking::Client;
use serde_json::Value;

struct Output {
    text: String,
    tooltip: String,
}

impl Output {
    pub fn new(text: String, tooltip: String) -> Self {
        Self { text, tooltip }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{ \"text\": \"{}\", \"tooltip\": \"{}\" }}",
            self.text, self.tooltip
        )
    }
}

fn get_temperature(city: &str, api_key: &str) {
    let url = format!("{BASE_URL}?q={city}&appid={api_key}&units=metric");
    let response = Client::new().get(&url).send();
    let response = match response {
        Ok(resp) => resp,
        Err(e) => panic!("ERROR: {e}"),
    };
    let v: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    if v["cod"] == 200 {
        let temp = &v["main"]["temp"];
        let temp = temp.as_f64().unwrap().round() as i64;
        let feels_like = &v["main"]["feels_like"];
        let feels_like = feels_like.as_f64().unwrap().round() as i64;
        let location = v["name"].as_str().unwrap_or("ERR");
        let forecast = v["weather"][0]["description"].as_str().unwrap_or("ERR");
        let code = &v["weather"][0]["icon"];
        let icon = match code.as_str().unwrap() {
            "01d" => "󰖙",
            "01n" => "󰖔",
            "02d" => "󰖕",
            "03d" => "󰖐",
            "03n" => "󰖐",
            "04d" => "󰖐",
            "04n" => "󰖐",
            "02n" => "󰼱",
            "09" => "󰖖",
            "10d" => "󰼳",
            "10n" => "",
            "10n 11n" => "",
            "10d 11d" => "󰼲",
            "11" => "",
            "13d" => "",
            "13n" => "",
            "50d" => "",
            "50n" => "",
            _ => "",
        };

        let text = format!("{icon} {temp}°C");
        let tooltip = format!(
            "Weather in {}\rTemp: {temp}°C\rFeels like: {}°C\rCondition: {}",
            location, feels_like, forecast
        );

        let output = Output::new(text, tooltip);
        print!("{output}")
    } else {
        let message = &v["message"];
        panic!("Error: {message}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if let Some(path) = args.get(2) {
        path.clone()
    } else {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".env")
            .to_str()
            .unwrap()
            .to_string()
    };

    if let Err(_) = dotenvy::from_path(&file_path) {
        println!("Could not load .env file from: {}", file_path);
    }

    let city = if let Some(city_arg) = args.get(1) {
        city_arg.clone()
    } else if let Ok(city_env) = var("CITY") {
        city_env
    } else {
        println!("Usage: weather [optional City] [optional path to .env file]");
        println!("City can be provided as an argument or in the .env file.");
        println!("Default .env location is $HOME/.env");
        return;
    };

    let api_key = match var("WEATHER_API") {
        Ok(val) => val,
        Err(_) => {
            println!("Could not find WEATHER_API in environment or .env file.");
            return;
        }
    };

    get_temperature(&city, &api_key);
}
