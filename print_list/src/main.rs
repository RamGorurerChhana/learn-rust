// In this example we are going to implement Display trait for Vec<T>.
// Since `Display` trait and `Vec<T>` type both belong to std module
// we cannot implement Display for Vec<T> directly.
// But we can’t implement external traits on external types.
// So we are going to create a local type List<T> which will contain Vec<T>

use std::fmt::{Display, Formatter, Result};

// Define a tuple struct List<T> which will contain one element Vec<T>
// Since we are going to implement Display trait for List<T>
// then T has to be trait bound Display
#[derive(Debug)]
struct List<T: Display>(Vec<T>);

// implement Display for List
// the resulting string will be of the format
// [0: T, 1: T, 2: T]
// 0, 1, 2 denotes the positional index
// so for List<u32>(vec![1, 2, 3]) it will print
// [0: 1, 1: 2, 2: 3]
impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (idx, elem) in self.0.iter().enumerate() {
            if idx > 0 {
                write!(f, ", {}: {}", idx, elem)?;
            } else {
                write!(f, "{}: {}", idx, elem)?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    println!("printing list with display trait");
    // create a list of type List<u32>
    let list: List<u32> = List(vec![1, 2, 3, 4, 5]);
    // print the list of List<u32>
    println!("{list}");

    // create a list of type List<&str>
    let list: List<&str> = List(vec!["one", "two", "three"]);
    // print the list of List<&str>
    println!("{list}");
}
