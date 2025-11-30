const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

use dotenvy::{dotenv, var};
use reqwest::blocking::Client;
use serde_json::Value;

fn check_internet() -> bool {
    let response = Client::new().get("https://google.com").send();
    match response {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_from_file(file: &str) -> String {
    dotenvy::from_filename(file).expect(&format!("Could not find {file}"));
    let api_key = var("WEATHER_API");
    match api_key {
        Ok(val) => return val,
        Err(_) => panic!("Could not find WEATHER_API in {file}"),
    }
}

fn get_temperature(city: &str, file: &str) {
    if !check_internet() {
        println!("No Internet");
        return;
    }

    let _ = dotenv();
    let api_key = var("WEATHER_API");

    let api_key = match api_key {
        Ok(val) => val,
        Err(_) => get_from_file(file),
    };

    let url = format!("{BASE_URL}?q={city}&appid={api_key}&units=metric");
    let response = Client::new().get(&url).send();
    let response = match response {
        Ok(resp) => resp,
        Err(_) => panic!("Could not connect to the weather service"),
    };
    let v: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    if v["cod"] == 200 {
        let temp = &v["main"]["temp"];
        let temp = temp.as_f64().unwrap().round() as i64;
        let code = &v["weather"][0]["icon"];
        let icon = match code.as_str().unwrap() {
            "01d" => "󰖙 ",
            "01n" => "󰖔 ",
            "02d" => "󰖕 ",
            "03d" => "󰖐 ",
            "03n" => "󰖐 ",
            "04d" => "󰖐 ",
            "04n" => "󰖐 ",
            "02n" => "󰼱 ",
            "09" => "󰖖 ",
            "10d" => "󰼳 ",
            "10n" => " ",
            "10n 11n" => " ",
            "10d 11d" => "󰼲 ",
            "11" => " ",
            "13d" => " ",
            "13n" => " ",
            "50d" => " ",
            "50n" => " ",
            _ => "",
        };
        print!("{icon} {temp}°C");
    } else {
        let message = &v["message"];
        panic!("Error: {message}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: weather [City] [Path to .env file] (optional)");
        return;
    }
    let city = &args[1];
    let file = if args.len() > 2 { &args[2] } else { "~/.env" };
    get_temperature(city, file);
}
