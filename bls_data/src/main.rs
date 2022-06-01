/************************************************************
 * Bureau of Labor Statistics Data Analysis
 * by David Doria
 * 
 * Retrieves data from the BLS api and performs statistical
 * analysis on it
 * 
 * Rust Documentation: "rustup docs --book"
 */

use std::collections::HashMap;

use reqwest;


fn main() {
    print!("######################################################\n");
    print!("#  Bureau of Labor Statistics Data Analysis Program  #\n");
    print!("#                  by David Doria                    #\n");
    print!("######################################################\n");
    print!("\n");

    get_data();
}

// Get the data from an API
#[tokio::main]
// Returns either a result, or a Box-wrapped error
async fn get_data() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}