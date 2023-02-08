use std::error::Error;
use std::io;
use std::env;

use csv;
use serde::Deserialize;

use utm;
;
#[derive(Debug, Deserialize)]
struct SurveyPoint {
    point: u16,
    longitude: f64,
    latitude: f64,
    ahd: f64,
}

/// Reads data from `stdin` into a reader and prints all records.
/// 
/// # Error
/// 
/// If an error occurs, the error is returned to `main`.
fn read_from_stdin() -> Result<(), Box<dyn Error>> {
    // Create a new csv Reader
    let mut reader = csv::Reader::from_reader(io::stdin());
    
    // .records return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);

    }

    Ok(())
}

/// Reads data from a file into a reader and prints all records.
/// 
/// # Error
/// 
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;
    println!("{:?}", headers);

    // // `.records` return an iterator of the internal
    // // record structure
    // for result in reader.records() {
    //     let record = result?;

    //     println!("{:?}", record);
    // }

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: SurveyPoint = result?;

        println!("{:?}", record);

        let (northing, easting, _) = utm::to_utm_wgs84_no_zone(record.latitude, record.longitude);

        println!("{:?}", (northing, easting));
    }

    Ok(())
}

fn main() {
   
    // // If an error occurs print error
    // if let Err(e) = read_from_stdin() {
    //     eprintln!("{}", e);
    // }
    
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];
    println!("{:?}", file_path);
    
    // If an error occurs print errord
    if let Err(e) = read_from_file(file_path) {
        eprintln!("{}", e);
    }
    
}

