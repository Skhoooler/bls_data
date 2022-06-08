/************************************************************
 * Bureau of Labor Statistics Data Analysis
 * by David Doria
 * 
 * Retrieves data from the BLS api and performs statistical
 * analysis on it
 * 
 * Rust Documentation: "rustup docs --book"
 */

use std::{fs::File, io::Read};

use csv::{StringRecord};


fn main() {
    print!("######################################################\n");
    print!("#              Data Analysis Program                 #\n");
    print!("#                  by David Doria                    #\n");
    print!("######################################################\n");
    print!("\n");
    
    // Get the file path
    let file_path = "src/Top-Apps-in-Google-Play.csv";

    // Get the file
    let mut file = File::open(file_path).expect("File not found");

    // Turn the data into a string
    let mut raw_data = String::new();

    file.read_to_string(&mut raw_data).expect("Error while reading file");
    
    // Serialize the raw data
    let data = read_from_raw_data(&raw_data);

    // Set up data recepticles
    let mut top_ten: Vec<String> = Vec::new();
    let mut in_app_purchases = 0;
    let mut ad_supported = 0;
    let mut social_apps: Vec<String> = Vec::new();
    let mut games: Vec<String> = Vec::new();

    // Iterate through the records to gather data
    let mut i = 0;
    for record in data {
        // Gather top ten apps
        if i < 10 {
            top_ten.push(record[1].to_string());
        }

        // How many in app purchases
        if record[9] == String::from("True") {
            in_app_purchases = in_app_purchases + 1;
        }

        // How many ad-supported apps
        if record[8] == String::from("True") {
            ad_supported = ad_supported + 1;
        }

        // How many are social media
        if record[3] == String::from("Social") {
            social_apps.push(record[1].to_string());
        }

        // How many are games
        if record[3] == String::from("Arcade") || record[3] == String::from("Casual") {
            games.push(record[1].to_string());
        }

        i = i + 1;
    }


    print!("######################################################\n");
    print!("#          Top 10 Apps in Google Play Store          #\n");
    print!("######################################################\n");
    for app in top_ten {
        println!("{:?}", app);
    }
    println!();

    print!("######################################################\n");
    print!("#                  In app purchases                  #\n");
    print!("######################################################\n");
    println!("{} out of the top {} apps on the Google Play store include in-app purchases",
        in_app_purchases, i);
    println!();

    print!("######################################################\n");
    print!("#                 Ad Supported Apps                  #\n");
    print!("######################################################\n");
    println!("{} out of the top {} apps on the Google Play store are supported by ads",
        ad_supported, i);

    print!("######################################################\n");
    print!("#                Top Social Media Apps               #\n");
    print!("######################################################\n");
    println!("The following {} out of the top {} apps on the Google Play store are social media apps",
    social_apps.len(), i);
    for app in social_apps {
        println!("{:?}", app);
    }
    println!();

    print!("######################################################\n");
    print!("#                      Top Games                     #\n");
    print!("######################################################\n");
    println!("The following {} out of the top {} apps on the Google Play store are games",
    games.len(), i);
    for app in games {
        println!("{:?}", app);
    }
    println!();

}

fn read_from_raw_data(data :&str) -> Vec<StringRecord> {
    // Create a new csv reader from path
    let mut reader = csv::Reader::from_reader(data.as_bytes());

    // Return a vector of StringRecords
    let mut data: Vec<StringRecord> = Vec::new();
    // reader.records() returns an interator for the records it got from the csv
    for result in reader.records() {
        let record = result.unwrap();
        
        data.push(record);
    }

    return data;
}