#![allow(unused_mut, unused_variables)]

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);
    change(&mut arg);
    
    fn inspect(s: &String){
        if !s.ends_with("s"){
            println!("{} is singular", s)
        }else{
            println!("{} is plural", s)
        }
    }
    
    fn change(s: &mut String){
        if s.ends_with("s"){
            return;
        }else{
            s.push_str("s");
        }
    }
    println!("I have many {}", arg);
    
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
fn eat(s: String) -> bool{
    if s.starts_with("b") && s.contains("a"){
        true
    }else{
        false
    }
}

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);

fn bedazzle(s: &mut String){
    *s = "Sparkly".to_string();
}
}