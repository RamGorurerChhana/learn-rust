// Below code snippet shows some of the examples of string formatting in rust.
// Details documentation can be found here -
// https://doc.rust-lang.org/std/fmt/
//

fn main() {
    // print normal text
    println!("Hello, This is pretty print examples");

    // print a variable
    let name = "Sibaprasad";
    println!("Hello {name}");

    let tuple = ("Sibaprasad", 75, "Kolkata");
    // Debug print
    // :? is added in the formatting for debug print
    println!("{tuple:?}");
    // Debug print beautified
    // :#? is added for debug print in beautified format
    println!("{tuple:#?}");

    let x = 5;
    // adding left padding with space
    println!("{x:>5}");
    // adding left padding with zeros
    println!("{x:0>5}");
    // adding right padding with zeros
    println!("{x:0<5}");

    let num = 635_732_usize;
    // print value in base 10
    println!("{num}");
    // print value in base 2
    println!("{num:b}");
    // print value in base 2, 0b appended at the beginning
    println!("{num:#b}");
    // print value in base 8
    println!("{num:o}");
    // print value in base 8, 0o appended at the beginning
    println!("{num:#o}");
    // print value in base 16
    println!("{num:x}");
    // print value in base 16, 0x appended at the beginning
    println!("{num:#x}");
}
