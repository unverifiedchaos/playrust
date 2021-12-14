use reqwest::Error;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Coffee {
    title: String,
    description: String,
    ingredients: Vec<String>,
    id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let req_url = format!("https://api.sampleapis.com/coffee/hot");
        println!("{}", req_url);//converts string to input
        let response = reqwest::get(&req_url).await?;
        let drinks: Vec<Coffee> = response.json().await?;
        println!("{:?}", drinks);

    } else {

        let result: usize = args[1].parse().unwrap();
        println!("{:?}", result);
        let req_url = format!("https://api.sampleapis.com/coffee/hot");
        let response = reqwest::get(&req_url).await?;
        let drinks: Vec<Coffee> = response.json().await?;
        println!("Title: {:?}\n", drinks[result].title);
        println!("Description: {:?}\n", drinks[result].description);
        println!("Ingredients: {:?}\n", drinks[result].ingredients);
    }
    Ok(())
}

