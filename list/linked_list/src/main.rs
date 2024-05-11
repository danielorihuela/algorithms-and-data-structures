use std::{fmt::Debug, ptr::NonNull};

#[derive(Debug, Clone)]
struct LinkedList<T: Clone> {
    head: Link<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;
fn next<T: Clone>(link: NonNull<Node<T>>) -> Link<T> {
    unsafe { (*link.as_ptr()).next }
}
fn next_mut<'a, T: Clone>(link: &'a mut NonNull<Node<T>>) -> &'a mut Link<T> {
    unsafe { &mut (*link.as_ptr()).next }
}
fn value<T: Clone>(link: NonNull<Node<T>>) -> T {
    unsafe { (*link.as_ptr()).value.clone() }
}

#[derive(Debug, Clone)]
struct Node<T: Clone> {
    value: T,
    next: Link<T>,
}

impl<T: Clone + Debug + PartialEq> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator { current: self.head }
    }

    fn search(&self, v: T) -> bool {
        self.iter().find(|n| value(*n) == v).is_some()
    }

    // I know the push and pop operations are inefficient.
    // I wanted to learn how to iterate over pointers and
    // modify them.
    fn push_back(&mut self, value: T) {
        let new_node =
            unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node { value, next: None }))) };

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut node = self.iter().last().unwrap();
            *next_mut(&mut node) = Some(new_node);
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        let last = self.iter().last();
        let before_last = self.iter().filter(|n| next(*n).is_some()).last();
        if let Some(mut before_last) = before_last {
            *next_mut(&mut before_last) = None;
        } else {
            self.head = None;
        }

        last.map(|v| value(v))
    }
}

struct LinkedListIterator<T: Clone> {
    current: Link<T>,
}

impl<T: std::clone::Clone> Iterator for LinkedListIterator<T> {
    type Item = NonNull<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            self.current = unsafe { (*node.as_ptr()).next };
            Some(node)
        } else {
            None
        }
    }
}

fn main() {
    let mut list = LinkedList::<u8>::new();
    list.push_back(5);
    list.push_back(7);
    list.push_back(9);
    list.push_back(8);

    print!("Elements in list: ");
    for item in list.iter() {
        print!("{}, ", value(item));
    }
    println!();

    println!("Element 7 in list? {}", list.search(7));

    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());

    println!("Element 7 in list? {}", list.search(7));
}
