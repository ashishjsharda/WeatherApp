mod handlers;

use warp::Filter;

#[tokio::main]
async fn main() {
    let weather_route = warp::path!("weather" / String)
        .and(warp::get())
        .and_then(handlers::get_weather);

    warp::serve(weather_route).run(([127, 0, 0, 1], 3030)).await;
}
