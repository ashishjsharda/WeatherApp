use warp::{Rejection, Reply};
use reqwest::Error;

pub async fn get_weather(city: String) -> Result<impl Reply, Rejection> {
    match fetch_weather(&city).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn fetch_weather(city: &str) -> Result<WeatherResponse, Error> {
    let url = format!("http://api.weatherapi.com/v1/current.json?key=t5q67&q={}", city);
    let response = reqwest::get(&url).await?.json().await?;
    Ok(response)
}
