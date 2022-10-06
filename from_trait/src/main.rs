// In this code sample we will implement From trait
// So that we can convert between two types
// this will allow to use `from` and `into` functions on any value

use std::convert::From;

// define struct to represent a Point on 2D plane
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// implement methods for Point
impl Point {
    // create new Point
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // method to get the equation of a line
    // equation for line (y−y1)/(x−x1) = (y1−y2)/(x1−x2)
    // (x−x1)(y1−y2) = (y−y1)(x1−x2)
    // (y1−y2)x - (x1−x2)y + (y1−y2)x1 + (x1−x2)y1 = 0
    fn line(&self, other: &Self) -> String {
        let mut cx = self.y - other.y;
        let mut cy = self.x - other.x;
        let mut cc = self.x * cy + other.y * cx;
        let mut s = String::new();
        // if two points are same then return empty string
        if cx == 0 && cy == 0 {
            return s;
        }
        // if x coefficient is negative
        // then multiply all coefficients with -1
        if cx < 0 {
            cx = -cx;
            cy = -cy;
            cc = -cc;
        }

        // add x variable to the equation
        if cx == 1 {
            s = "x".to_string();
        } else if cx != 0 {
            s = format!("{cx}x");
        }

        // add y variable to the equation
        if s.is_empty() {
            if cy == 1 {
                s = "y".to_string();
            } else if cy == -1 {
                s = "-y".to_string();
            } else if cy != 0 {
                s = format!("{cy}y");
            }
        } else if cy == 1 {
            s = format!("{} + y", s);
        } else if cy == -1 {
            s = format!("{} - y", s);
        } else if cy < 0 {
            s = format!("{} - {}y", s, -cy);
        } else if cy > 0 {
            s = format!("{} + {}y", s, cy);
        }

        // add constant to the coefficient to the equation
        if cc < 0 {
            s = format!("{} - {} = 0", s, -cc);
        } else if cc > 0 {
            s = format!("{} + {} = 0", s, cc);
        }
        s
    }
}

// define a tuple struct for point
// first value is x coordinate and second value is y
#[derive(Debug, Clone, Copy)]
struct SimplePoint(i32, i32);

impl From<SimplePoint> for Point {
    fn from(item: SimplePoint) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let p1 = Point::new(3, -5);
    let p2 = SimplePoint(7, 7);
    let line = p1.line(&p2.into());
    println!("{line}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_into() {
        let p1 = Point::new(3, -5);
        let p2 = SimplePoint(7, 7);
        // value of p2 is moved in the next line
        // we have derived Clone, Copy so that value can be copied
        let p3 = Point::from(p2);
        let line1 = p1.line(&p3);
        let line2 = p1.line(&p2.into());
        assert_eq!(line1, line2);
    }
}
