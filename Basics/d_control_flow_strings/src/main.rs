#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum"{
            sum();
        } else if arg == "double"{
            double();
        }else {
            count(arg);
        };
    }
}

fn sum() {
    let mut sum = 0;

    for i in 7..=23{
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        count += 1;
        x *= 2;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut times = 1;
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //

    loop {
        print!("{} {}", arg, times);
        println!();
        times += 1;
        if times == 9 {
            break;
        }else{
            continue;
        }
    }
    

}
