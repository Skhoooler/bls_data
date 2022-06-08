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

    // Iterate through the records to gather data
    let mut i = 0;
    for record in data {
        // Gather top ten apps
        if i < 10 {
            top_ten.push(record[1].to_string());
        }


        i = i + 1;
    }


    print!("######################################################\n");
    print!("#          Top 10 Apps in Google Play Store          #\n");
    print!("######################################################\n");
    for app in top_ten {
        println!("{:?}", app);
    }
    
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