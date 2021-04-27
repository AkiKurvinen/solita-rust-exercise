extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

struct Record {
    r_city_id: String,
    r_latitude: f64,
    r_longitude: f64,
}

fn find_min_max(cities: &Vec<Record>) -> [f64;4] {
// find min & max (latitude)
    let mut minlat: f64 = 0.0;
    let mut maxlat: f64 = 0.0;

    for x in cities {
        if minlat == 0.0 {minlat = x.r_latitude;}
        if maxlat == 0.0 && minlat != 0.0 {maxlat = x.r_latitude;}
        if minlat > x.r_latitude {minlat = x.r_latitude;}
        if maxlat < x.r_latitude {maxlat = x.r_latitude;}
    }
    //println!("\nminlat: {} \nmaxlat: {}", minlat, maxlat );

// find min & max (longitude)
    let mut minlon: f64 = 0.0;
    let mut maxlon: f64 = 0.0;

    for x in cities {
        if minlon == 0.0 {minlon = x.r_longitude;}
        if maxlon == 0.0 && minlon != 0.0 {maxlon = x.r_longitude;}
        if minlon > x.r_longitude {minlon = x.r_longitude;}
        if maxlon < x.r_longitude {maxlon = x.r_longitude;}
    }
    //println!("\nminlon: {} \nmaxlon: {}", minlon, maxlon );

// return array
        let array: [f64;4] = [minlat, maxlat, minlat, maxlat ];
        return array;
}

fn calc_lon(lon: f64) -> u8 {
    // Lon: y = 0,5882x + 5,3706
    let lon = 0.5882*lon + 5.3706;
    return lon as u8;
}
fn calc_lat(lat: f64) -> u8 {
    // Lat: y = 1,9968x - 70,148
    let lat = 1.996*lat - 70.148;
    return lat as u8;;
}
fn print(cities: &Vec<Record>){

//create table
    let width = 25;
    let height = 50;

    let mut map_vec = vec![vec!["     ".to_string(); width]; height];

    for x in cities {
      let lo =  calc_lon(x.r_longitude) as usize;
      let la =  calc_lat(x.r_latitude) as usize;

      // println!("{},{}", la, lo);

      map_vec[la][lo]  = x.r_city_id.to_string();
      // println!("{}",  map_vec[b][a] );
    }

// print table as map
    map_vec.reverse();

    for (i, row) in map_vec.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            print!("{}", col);
        }
        println!("");
    }
}

fn run() -> Result<(), Box<Error>> {
    let file_path ="city.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    //
    let mut cities : Vec<Record> = Vec::new();
    //

    for result in rdr.records() {
        let record = result?;

        let city_id = &record[0];
        let latitude: f64  = record[1].parse()?;
        let longitude: f64  = record[2].parse()?;
        /*
        println!("city_id: {:?}, latitude: {:?}, longitude: {:?}",
            city_id, latitude, longitude);

        */
        let entry = Record {
        r_city_id:city_id.to_string(),
        r_latitude:latitude,
        r_longitude:longitude
        };
        cities.push(entry);
    }

    // for x in &cities {println!("{},{},{}", x.r_city_id, x.r_latitude, x.r_longitude);}

    let array: [f64; 4] =  find_min_max(&cities);
    // println!("\nminlat: {}\nmaxlat: {}", array[0], array[1] );

    print(&cities);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}