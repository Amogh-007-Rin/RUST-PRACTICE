// Advance String Concepts --> Here we will know that why RUST is really hard to programme compared to JavaScript/TypeScript.


fn strings(){
    let greetings = String::from("Hello World"); // This will be the syntax to create a immutable string
    println!("{}", greetings);

    // If we try to print any of the character of this string greeting, thats where the things begins to get complecated.

    // char1 = greetings[0];
    // println!("{}", char1);
    // This thing will not work, compiler will through an error.
    

    // This is the new thing which we have to write in order to access a character in the string.
    let char1 = greetings.chars().nth(1);  // This is not an character (Its an optional character)
    // println!("{}", char1); We cannot log it like this.

    println!("Character at index 1 is: {}", char1.unwrap()); // One way to resolving the compilation error but gives error at run-time.  

    // The optimum way to do this is : Using the Match Case Statements
    match char1{
        Some(c) =>  println!("Character at index 1 is: {}", c),
        None => println!("No character at index")
    }

}

fn main() {
    strings();
}
