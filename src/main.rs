use reqwest::Error;
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
    let req_url = format!("https://api.sampleapis.com/coffee/hot");
    println!("{}", req_url);//converts string to input
    let response = reqwest::get(&req_url).await?;
    let drinks: Vec<Coffee> = response.json().await?;
    println!("{:?}", drinks[2].title);
    Ok(())
}
