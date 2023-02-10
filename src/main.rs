use std::env;
use std::error::Error;
use std::io;

use csv;
use serde::{Deserialize, Serialize};

use utm;

#[derive(Debug, Deserialize)]
struct SurveyPoint {
    point: u16,
    longitude: f64,
    latitude: f64,
    ahd: f64,
}

#[derive(Debug, Serialize)]
struct PENZD {
    point: u16,
    easting: f64,
    northing: f64,
    level: f64,
    description: String,
}

/// Reads data from a file into a reader and prints all records.
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str) -> Result<Vec<PENZD>, Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;
    println!("{:?}", headers);

    let mut penzd_points: Vec<PENZD> = vec![];

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: SurveyPoint = result?;

        let (northing, easting, _) = utm::to_utm_wgs84_no_zone(record.latitude, record.longitude);
        
        let penzd = PENZD {
            point: record.point,
            easting: easting,
            northing: northing,
            level: record.ahd,
            description: "".to_string(),
        };

        penzd_points.push(penzd);

    }

    Ok(penzd_points)
}

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];

    // If an error occurs print errord
    let utm_projections = match read_from_file(file_path) {
        Ok(penzd_points) => penzd_points,
        Err(e) => return Err(e),
    };

    println!("{:?}", utm_projections);

    Ok(())
}