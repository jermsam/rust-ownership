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

    let y = give_ownership(); // y takes ownership of give_ownership's return value
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
    println!("Expensive deep clone x = {} and y = {}.", x,y)
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
    y; // return y
} // y moves out of scope into whatever calls it;