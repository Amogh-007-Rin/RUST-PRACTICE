// Conditional Statements --> Almost similar to JavaScript and nothing fancy here

fn conditional_statements(){

    let is_male: bool = true;
    let mut is_above_18: bool = false;

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

// Loops and Interations

fn for_loops(){
    
    let mut a = 0;
    for i in 0..11{
        a = a + i;
        print!("{} ", a);
    }
}



fn main() {
    conditional_statements();
    for_loops();
}
