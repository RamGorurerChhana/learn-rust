// Infamous fizzbuzz code sample in rust

fn main() {
    println!("Print fizzbuzz sequence");
    for n in 0..=100 {
        match (n % 5 == 0, n % 3 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("buzz"),
            (false, true) => println!("fizz"),
            (false, false) => println!("{n}"),
        };
    }
}
