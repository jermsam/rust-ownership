fn main() {
    println!("Hello, world!");

    /*
    If a type implements the Copy trait, variables that use it do not move,
     but rather are trivially copied, making them still valid after assignment to another variable.
    */
    string_literal_copy();

    sting_shallow_copy();
    string_deep_copy();

    let x = "Sam";  // x comes into scope
    make_copy(x); // &str is Copy, so it's okay to still use x afterward
    println!("{} is still valid", x);
    let y = String::from("Sam"); // y comes into scope
    take_ownership(y); // y moves into take_ownership
    // y is no longer valid here.
    let x = give_ownership(); // y takes ownership of give_ownership's return value
    let y = take_and_give_back(x); // x is moved into takes_and_gives_back,  which also moves its return value into y
    println!("y = {} is what is valid here", y);

    let z = get_length(y); //get_length takes ownership of y, gets its length and return both as a tuple that it moves into z
    // y is invalid here
    println!("Length of {} is {}", z.0, z.1);
    let length = get_length_with_ref_arg(&(z.0)); // fn uses value at Z but never takes ownership of it
    // thus tuple Z is still valid here also fn moves its return value to length
    println!("Length of {} is {}", z.0,length); // both length and z are valid here
    println!("borrowing - use it but dont own it");
    let mut s = String::from("Sam");
    mutable_references(&mut s);
    reference_rules();
}

// string literals, easy copy

fn string_literal_copy() { // x and y variables here implement the Copy trait
    // double quote str literals, single quote chars, put underscore for unused vars
    let x = "Sam"; // x enters scope .... RAII bind the value 5 to x
    let y = x; // y enters into scope ... make a copy of the value in x and bind it to y
    println!("both x = {} and y = {} in scope", x, y);
    println!("This is indeed what is happening, \
    because integers are simple values with a known, \
    fixed size, and these two 5 values are pushed onto the stack.");
} // x and y go out of scope and both copies of content are popped off the stack

// String , ownership managed copy

fn sting_shallow_copy() {
    // :: for namespacing - request memory allocator for space in the heap
    // A String is made up of 3 parts (that are stored on the stack)
    // 1. A pointer to the memory on the heap: that holds the contents of the string
    // 2. The Length - which is how much memory (in bytes) the contents of the string is currently holding
    // 3. The Capacity - which is the total amount of memory the string has received from the memory allocator
    let x = String::from("Sam"); //
    let y = x; // x's pointer, length and capacity on the heap are copied but not its content on the heap
    // x become invalid here to avoid a double free error (dropping allocated memory more than once)
    // which can cause memory corruption that can potentially lead to security vulnerabilities

    println!("y = {}  while x is invalid", y);
    println!("We say x moved to y to mean that at y = x; \
    the pointer, length and capacity for x were copied into y but not its content")
} // drop is called, x and y go out of scope but memory is only cleared for y since x is invalid

fn string_deep_copy() {
    let x = String::from("Sam"); // x comes into scope
    let y = x.clone(); // y enters into scope ... make a copy of the value in x and bind it to y
   // x is not moved into y hence still valid
    println!("Expensive deep clone x = {} and y = {}.", x,y);
} // drop is called, x and y go out of scope and both copies of content are dropped the stack

// FN and ownership

fn make_copy (x: &str){ // comes into scope
    println!("{} is a string literal", x);
} // x gets out of scope and nothing happens

fn take_ownership (y: String) { // comes into scope
    println!("{} is a String", y);
} // y gets out of scope and drop is called and the backing memory is cleared

fn give_ownership() -> String {
    let y = String::from("Sam"); // comes into scope
    y // return y
} // y moves out of scope into whatever calls it;

fn take_and_give_back (arg: String) -> String { // arg comes into scope
  arg // a_string is returned and moves out to the calling function
}

// if we want a fn to take ownership of a value, use it and then return

fn get_length(arg: String) -> (String, i32) {
    let length: i32 = arg.len() as i32;
    (arg, length)
}

/*
A reference is like a pointer in that it’s an address
we can follow to access the data stored at that address
Unlike a pointer, a reference
is guaranteed to point to a valid value of a particular type
for the life of that reference.
*/

fn get_length_with_ref_arg(arg: &String) ->  i32 { // arg is a reference to a String
    let length: i32 = arg.len() as i32;
    println!("creating a reference without owning is called borrowing");
    length
} // Here, arg goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn mutable_references( arg: &mut String) {
    // push pushes char push_str pushes str
    arg.push_str(", says hi!");
    println!("Strings and references can be mutable. Primitives like string literals are not")
}

/*
Users of an immutable reference don’t expect the value to suddenly change out from under them!
 However, multiple immutable references are allowed because no one who is just reading the data
 has the ability to affect anyone else’s reading of the data.
*/
fn reference_rules () {
    let mut s = String::from("hello");

    // {
    //     let r1 = &mut s; // you cant have another mutable reference here
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    //
    // let r2 = &mut s;
    // println!("{} can be used here", r2)

    // let r1 = &s; // no problem - read only borrowing
    // let r2 = &s; // no problem - read only borrowing
    // let r3 = &mut s; // BIG PROBLEM - writable borrowing, c'mon you can't mutate s b4 its been used
    // println!("{}, {}, and {}", r1, r2, r3);

    /*
    Note that a reference’s scope starts from where it is introduced
    and continues through the last time that reference is used.
    */
    let r1 = &s; // no problem - read 0nly
    let r2 = &s; // no problem - read only
    println!("{} and {}", r1, r2); // - used so &r1 and &r2 go out of scope
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem - you can borrow it again as writable
    println!("{}", r3);
    /*
    The ability of the compiler to tell that a reference is no longer being used at a point
    before the end of the scope is called Non-Lexical Lifetimes (NLL for short),
    */
}