fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    fn bunny_nibbles<T: Bite>(item: &mut T){
        item.bite();
        item.bite();
        item.bite();
        item.bite();
    }
    
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}
trait Bite{
    fn bite(self: &mut Self){
    
    }
}

#[derive(Debug)]
struct Grapes{
    amount_left: i32,
}

impl Bite for Grapes{
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}