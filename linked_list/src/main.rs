// In this code sample we are going to implement Linked List
// with enum based representation

use std::fmt::{Display, Formatter, Result};

// define a enum which contains two variants
// 1st variant contains data field and pointer to next element
// 2nd variant contains Nil which indicates end of linked list
#[derive(Debug, Clone)]
enum LinkedList {
    Node(i32, Box<LinkedList>),
    Nil,
}

// implementation of methods for LinkedList
impl LinkedList {
    // create a new empty linked list
    fn new() -> Self {
        LinkedList::Nil
    }

    // push node to the linked list
    // Note: self parameter is not a reference so it takes the ownership
    fn push(self, val: i32) -> Self {
        Self::Node(val, Box::new(self))
    }

    // pop node from the head of the linked list
    // Note: self parameter is not a reference so it takes the ownership
    // it returns Option because the list could be empty
    fn pop(self) -> (Self, Option<i32>) {
        match self {
            Self::Nil => (self, None),
            Self::Node(v, next) => (*next, Some(v)),
        }
    }

    // calculate length of the linked list
    fn len(&self) -> usize {
        match self {
            Self::Node(_, next) => 1 + next.len(),
            Self::Nil => 0,
        }
    }

    // find an element in the linked list returns Option<&i32>
    fn find(&self, val: i32) -> Option<&i32> {
        match self {
            Self::Node(v, _) if *v == val => Some(v),
            Self::Node(v, next) if *v != val => next.find(val),
            _ => None,
        }
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Node(v, next) => match **next {
                Self::Nil => write!(f, "{}", v),
                _ => write!(f, "{} -> {}", v, next),
            },
            Self::Nil => write!(f, ""),
        }
    }
}

fn main() {
    println!("Enum based Linked List representation");
    let list = LinkedList::new();
    let list = list.push(30);
    let list = list.push(32);
    let list = list.push(43);
    println!("{list:?}");
    println!("{}", list.len());
    println!("{:?}", list.find(300));
    println!("{list}");
    println!("size of `list` in bytes: {}", std::mem::size_of_val(&list));
    let (list, e) = list.pop();
    println!("{e:?}");
    println!("{list}");
    println!("size of `list` in bytes: {}", std::mem::size_of_val(&list));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blank_list() {
        let list = LinkedList::new();
        assert_eq!(0, list.len());
        assert_eq!("".to_string(), list.to_string());
        assert_eq!(None, list.find(1));
    }

    #[test]
    fn test_list() {
        let list = LinkedList::new();
        let list = list.push(1);
        let list = list.push(2);
        let list = list.push(3);
        assert_eq!(3, list.len());
        assert_eq!("3 -> 2 -> 1".to_string(), list.to_string());
        assert_eq!(Some(&3), list.find(3));
        assert_eq!(None, list.find(4));

        let (list, elem) = list.pop();
        assert_eq!(2, list.len());
        assert_eq!(Some(3), elem);

        let (list, elem) = list.pop();
        assert_eq!(1, list.len());
        assert_eq!(Some(2), elem);

        let (list, elem) = list.pop();
        assert_eq!(0, list.len());
        assert_eq!(Some(1), elem);

        let (list, elem) = list.pop();
        assert_eq!(0, list.len());
        assert_eq!(None, elem);
    }
}
