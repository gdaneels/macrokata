////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.
macro_rules! math {
    ($first:literal plus $second:literal) => {
       $first + $second 
    };
    (square $factor:literal) => {
       $factor * $factor 
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(lol plus 5));
    print_result(math!(square 2));
}
