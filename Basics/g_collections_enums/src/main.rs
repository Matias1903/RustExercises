#![allow(unused_variables, unused_mut, dead_code)]

enum Shot{
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self{
            Shot::Bullseye => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) => 1,
            Shot::Miss => 0,
        }
    }
}

fn main() {
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    for coord in arrow_coords{
        coord.print_description();
        let shot = match coord.distance_from_center(){
            x if x < 1.0 => Shot::Bullseye,
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
        };
        shots.push(shot);
    }

    let mut total = 0;
    for shot in shots{
        total += shot.points();
    }

    println!("Final point total is: {}", total);
}


#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}


fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}