extern crate cronjob;
use cronjob::CronJob;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use reqwest::Error;


// #[derive(Deserialize, Debug)]
struct price_response {
    price: f32,
    prod: String,
    updated_at: i32,
}

fn main() {
    // Create the `CronJob` object.
    let mut cron = CronJob::new("Test Cron", on_cron);
    cron.set_checking_interval(1000);
    // Start the cronjob.
    cron.start_job();
}


struct DetailResult{
    t: u32,
    o: f32,
    c: f32,
    h: f32,
    l: f32,
}

struct PolygonResponse {
    ticker: String,
    status: String,
    results: Vec<DetailResult>,
}

async fn call_polygon(prod: String) -> price_response  { // Result<price_response, Error>
    let api_url = "https://api.polygon.io/v2/aggs/ticker/MSFT/range/1/minute/2023-08-02/2023-08-07?adjusted=true&sort=asc&limit=1&apiKey=pgmOkQ5F2KAteLewCArt3m3IUtWppOhP";
    let client = reqwest::Client::new();
    let response = client
        .get(api_url)
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("Response: {:?}", response);
    
    // let response = reqwest::get(api_url).await?;
    // if response.status().is_success() {
    //     // Deserialize the JSON response into the ApiResponse struct
    //     let api_response: PolygonResponse = response.json().await?;

    //     // Do something with the API data
    //     println!("Response: {:?}", api_response.status);
    // } else {
    //     // Handle the case when the API call was not successful
    //     println!("API call failed with status code: {:?}", response.status());
    // }
    let res = price_response { price: 14.2, prod: String::from("mic"), updated_at: 12488751 };
    res
    
    // price_response { price: 14.2, prod: String::from("mic"), updated_at: 12488751 }
}

// Our cronjob handler.
async fn on_cron(name: &str) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();

    let prod_lisst: Vec<_> = Vec::from(["MSFT","AAPL","BNKU","AMZN"]);


    println!("{}", datetime.format("%d/%m/%Y %T"));
    let price = call_polygon(String::from("stirng")).await;
    println!("{:?}: It's time!", price.price );
}
// 1690963200000
// 1690963200000

 // https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html