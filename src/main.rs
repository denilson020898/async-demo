use futures::future::try_join_all;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ClientBuilder::new().build()?;
    let now = Instant::now();

    // let mut gets = Vec::new();
    // for id in 1..=100 {
    //     let get = get_todo(&client, id);
    //     gets.push(get);
    // }
    // let result = try_join_all(gets).await?;

    let mut result = Vec::new();
    for id in 1..=100 {
        let res = get_todo(&client, id).await?;
        result.push(res);
    }

    println!("elapsed: {} secs", now.elapsed().as_secs_f64());
    println!("result : {:#?}", result.last().unwrap());
    Ok(())
}

async fn get_todo(client: &Client, id: i32) -> Result<Value, Box<dyn Error>> {
    let base = "https://jsonplaceholder.typicode.com/todos/";
    let addr = format!("{}{}", base, id);
    let result = client.get(&addr).send().await?.json().await?;
    Ok(result)
}
