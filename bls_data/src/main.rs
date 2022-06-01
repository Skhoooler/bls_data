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

extern crate csv;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use std::io;
use reqwest;
use dotenv::dotenv;
use std::env;

fn main() {
    print!("######################################################\n");
    print!("#  Bureau of Labor Statistics Data Analysis Program  #\n");
    print!("#                  by David Doria                    #\n");
    print!("######################################################\n");
    print!("\n");
    
    // Get the file
    let file_path = r#"src\Top-Apps-in-Google-Play.csv"#;
    let file = File::open(file_path);



    // Reads data from a file
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result.expect("a csv record");
        println!("{:?}", record);
    }
    //get_data();
}

// Get the data from an API
#[tokio::main] // This method uses the main method of the tokio crate
// Returns either a result, or a Box-wrapped error
async fn get_data() -> Result<(), Box<dyn std::error::Error>> {
    // Get api key from my .env file
    dotenv().ok(); 
    let api_key = env::var("API_KEY")?;
    

    // Set up the URL for the API Call
    let url = "https://api.bls.gov/publicAPI/v2/timeseries/data/";
    let headers = [("Content-type", "application/json"), ("registrationKey", &api_key), ("startyear", "2021"), ("endyear", "2021"), ("seriesid", "WMU00140201020000001300002500"), ("catalog","true")];

    // Make the API Call
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .form(&headers)
        .send()
        .await?;
    /*let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;*/
    println!("{:#?}", response);
    Ok(())
}