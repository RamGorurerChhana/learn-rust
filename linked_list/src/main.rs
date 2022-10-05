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

    // prepend node to the linked list
    // Note: self parameter is not a reference so it takes the ownership
    fn prepend(self, val: i32) -> Self {
        Self::Node(val, Box::new(self))
    }

    // insert element at a position in the list
    // Note: self parameter is a mutable reference
    // does not take the ownership
    fn insert(&mut self, val: i32, pos: usize) {
        let mut target = self;
        for _ in 0..pos {
            match target {
                Self::Nil => panic!("IndexOutOfBoundsError"),
                Self::Node(_, next) => {
                    target = next;
                }
            }
        }
        // TODO: Implement the below line without cloning the list
        // Cloning is an expensive operation
        let list = target.clone().prepend(val);
        *target = list;
    }

    // calculate length of the linked list
    fn len(&self) -> usize {
        match self {
            Self::Node(_, next) => 1 + next.len(),
            Self::Nil => 0_usize,
        }
    }

    // find an element in the linked list returns Option<usize>
    // if the element is found then return Some(index)
    // index starts from zero. otherwise None.
    fn find(&self, val: i32) -> Option<usize> {
        match self {
            Self::Node(v, _) if *v == val => Some(0),
            Self::Node(v, next) if *v != val => {
                if let Some(v) = next.find(val) {
                    Some(v + 1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    let mut list = LinkedList::new();
    list = list.prepend(30);
    list = list.prepend(32);
    list = list.prepend(43);
    println!("{list:?}");
    println!("{}", list.len());
    println!("{:?}", list.find(300));
    list.insert(45, 1);
    println!("{list}");
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
    fn test_prepend() {
        let list = LinkedList::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);
        assert_eq!(3, list.len());
        assert_eq!("3 -> 2 -> 1".to_string(), list.to_string());
        assert_eq!(Some(0), list.find(3));
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert(0, 0);
        list.insert(1, 1);
        list.insert(2, 2);
        list.insert(3, 3);
        assert_eq!(4, list.len());
        assert_eq!("0 -> 1 -> 2 -> 3".to_string(), list.to_string());
        assert_eq!(Some(3), list.find(3));
        list.insert(99, 2);
        assert_eq!(Some(2), list.find(99));
    }
    #[test]
    #[should_panic]
    fn test_insert_invalid() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
    }
}
