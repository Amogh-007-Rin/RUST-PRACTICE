// In this file i was basically exploring the memory management and the garbage collection perforemd in RUST.

fn stack_fn(){
    // Declared few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of a:{} and b:{} is {}", a , b, c);
}

fn heap_fn(){
    // Create a string, wich is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let combined = format!("{} {}", s1,s2);
    println!("Heap function: Combined string is {}", combined);
}

fn update_string(){
    let mut s = String::from("Initial String");
    println!("Before Update {}", s);
    println!("Capactity: {}, Lenght:{}, size: {}, pointer:{}", s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);

    // Append some text to the previous string
    // and check the values of capacity, length and pointer after each update
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capactity: {}, Lenght:{}, size: {}, pointer:{}", s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);

    s.push_str(" One more addition");
    println!("After update: {}", s);
    println!("Capactity: {}, Lenght:{}, size: {}, pointer:{}", s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);

    s.push_str(" Second more addition");
    println!("After update: {}", s);
    println!("Capactity: {}, Lenght:{}, size: {}, pointer:{}", s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);

    s.push_str(" Third more addition");
    println!("After update: {}", s);
    println!("Capactity: {}, Lenght:{}, size: {}, pointer:{}", s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);
}

// Run this function individually to see how memory allocation changes with each iteration
fn continue_memory_updation(){
    let mut s = String::from("Start");
    for i in 0..100 {
        s.push_str(" More text");
        println!("After iteration {}: Capacity: {}, Length: {}, Size: {}, Pointer: {}", i, s.capacity(), s.len(), std::mem::size_of_val(&s), s.as_ptr() as usize);
    }
}

fn main() {
   stack_fn(); // Calls the function which uses stack memory
   heap_fn(); // Calls the function which uses heap memory
   update_string(); // Calls the function which calls size of variable at runtime
   
   println!();
   println!();
   println!();
   continue_memory_updation();
}
