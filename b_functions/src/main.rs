#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    println!("Volume is {}", volume(width, height, depth));
}

fn volume(a:i32, b:i32, c:i32) -> i32{
    let result = a * b * c;

    result
}

fn area_of(x: i32, y: i32) -> i32 {

    let result = x * y;

    result
}