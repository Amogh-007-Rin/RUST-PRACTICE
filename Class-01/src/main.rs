use std::string;

// Variables
fn variables(){
    let a = -10;  // This is how we define an integer
    let b: u32 = 30;  // This is how we define an positive only integer
    let c: f32 = 20.000; // This is how we define an flote value

    println!("a: {}, b: {}, c: {}", a, b, c);
}

// Booleans
fn booleans(){
    let is_male: bool = true;  // This is how we define booleans // Also this is an immutable variable
    let mut is_above_18 : bool = false; // This is how we define booleans // This is a mutable varable.
    

    // This is how we use the conditional statements
    if is_male{
        println!("you are a male");
        is_above_18 = true;  // We can change the value of this variable since this is a mutable variable.

    } else{
        println!("You are not a male")
    }
    
    if is_male && is_above_18{
        println!("You are an adult male")
    }
}

// Strings ---> This will seems to be easy but its kinda trick sometime. 

fn strings(){
    let greetings = String::from("Hello World");
    println!("{}", greetings);
}

fn main() {
    println!("----Variables----");
    variables();
    println!("----booleans----");
    booleans();
    println!("----Strings----");
    strings();
    println!();
}
