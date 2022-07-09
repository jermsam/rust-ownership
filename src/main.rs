fn main() {
    println!("Hello, world!");
    string_literal_copy();
}

// string literals, easy copy

fn string_literal_copy() {
    // double quote str literals, single quote chars, put underscore for unused vars
    let x = "Sam"; // x enters scope .... RAII bind the value 5 to x
    let y = x; // y enters into scope ... make a copy of the value in x and bind it to y
    println!("both {} and {} in scope", x, y);
    println!("This is indeed what is happening, \
    because integers are simple values with a known, \
    fixed size, and these two 5 values are pushed onto the stack.");
}