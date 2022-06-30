use std::{fs::File, io::BufReader};

use las::{Reader, Read};


fn check_bounds(reader: &mut las::Reader) {
    let mut min: f64 = f64::INFINITY;
    let mut max: f64 = f64::NEG_INFINITY;

    let head = reader.header();
    let num_points = head.number_of_points();
    let mut counter: u64 = 0;

    for raw_point in reader.points() {
        counter += 1;
        if counter % 10000000 == 0 {
            println!("at {:.0}% ...", (counter as f64 / num_points as f64) * 100.0);
        }
        let p = raw_point.unwrap();
        if p.x < min {
            min = p.x;
        } 
        if p.x > max {
            max = p.x;
        }
    }

    println!("my extends: [ min: {} to max: {} ]", min, max);
}

fn reduce(reader: &mut las::Reader, count: u64) {
    let head = reader.header();
    let num_points = head.number_of_points();
    let mut counter: u64 = 0;
    for _ in reader.points() {
        counter += 1;
        if counter % 10000000 == 0 {
            println!("at {:.0}% ...", (counter as f64 / num_points as f64) * 100.0);
        }
        if counter > count { break; }
        // let p = raw_point.unwrap();
        // if p.x < min {
        //     min = p.x;
        // } 
        // if p.x > max {
        //     max = p.x;
        // }
    }
}

fn main() {
    let path = String::from("./assets/autzen.las");

    let file = File::open(path).expect("can't open!, wrong path!");
    let buf = BufReader::new(file);
    let mut reader = Reader::new(buf).expect("Not a las / laz!");

    let head = reader.header();
    println!("header:");
    println!("version: {}", head.version());
    println!("count: {} points.", head.number_of_points());
    println!("extends: {:?} points.", head.bounds());

    check_bounds(&mut reader);
    // reduce(&mut reader, 1000);

    println!("done!:");
}
