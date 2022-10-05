// Below code snippet shows some of the examples of string formatting in rust.
// Details documentation can be found here -
// https://doc.rust-lang.org/std/fmt/
//

use std::f64::consts::PI;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// define a Person struct
// the use of `derive` attribute automatically
// implements the `debug` trait for this struct
// which allow us to use :? or :#? in print statement for this struct
#[derive(Debug)]
struct Person {
    name: String,
    address: String,
    dob: (u32, u32, u32),
}

// implementation block for sturct Person
// new function helps to create a instance of the struct
impl Person {
    fn new(name: &str, address: &str, dob: (u32, u32, u32)) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
            dob,
        }
    }
}

// implement Display trait so that it can be printed using println
impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} ({}/{}/{}) - {}",
            self.name, self.dob.0, self.dob.1, self.dob.2, self.address
        )
    }
}

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

    // precision variable decides how many decimal places to be shown in the value of PI.
    let precision = 6;
    // format the value of PI to show the no of decimal places
    // as initialized by the precision variable
    println!(
        "Value of PI (upto {precision} decimal places): {:.precision$}",
        PI
    );

    // create a new instance of Person
    let person = Person::new("Sibaprasad", "Kolkata", (20, 7, 1975));
    // debug print Person
    println!("{person:?}");
    // debug print Person beautified
    println!("{person:#?}");
    // print value of person
    println!("{person}");
}
