fn main() {

//prinln!("{}",s);// variable s is not valid here since it hasn't been declared yet
    let _s : &str = "hello";    // variable is valid here
    
    println!("{}",_s); // variable is valid here because it has already been declared.
    
    // s can do stuff here.


    let mut _d = String::from(_s);
    
    //println!("{_d}");

    _d = String::from("ahoy"); //Hello that was assigned is cleared from memory.

    //println!("{_d}");

    _d.push_str(",world"); // push_str() appends a literal to a string

    println!("{_d}");


    //Memory Allocation in Integers 
    let _x = 5;
    let _y = _x;

    //Memory Allocation in String
    let s1 = String::from("hello");
    let _s2 = s1;

    //Cloning the value of s1 thus cloning the heap and not the moving the data.
    let _s3 = _s2.clone();

    println!("_s2:{_s2}, _s3:{_s3}");
    
    let s = String::from("hello");  // s comes into scope

    //takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let _f = takes_ownership(s);    //value returned from function can be assigned here.Transferring Ownership here.
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

 

} 
// the scope of s ends here. //scope of d also ends here
// Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.



fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
}

 