use std::{fmt::Debug, ptr::NonNull};

#[derive(Debug, Clone)]
struct LinkedList<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;
fn next<T: Clone>(link: NonNull<Node<T>>) -> Link<T> {
    unsafe { (*link.as_ptr()).next }
}
fn prev<T: Clone>(link: NonNull<Node<T>>) -> Link<T> {
    unsafe { (*link.as_ptr()).prev }
}
fn next_mut<'a, T: Clone>(link: &'a mut NonNull<Node<T>>) -> &'a mut Link<T> {
    unsafe { &mut (*link.as_ptr()).next }
}
fn prev_mut<'a, T: Clone>(link: &'a mut NonNull<Node<T>>) -> &'a mut Link<T> {
    unsafe { &mut (*link.as_ptr()).prev }
}
fn value<T: Clone>(link: NonNull<Node<T>>) -> T {
    unsafe { (*link.as_ptr()).value.clone() }
}

#[derive(Debug, Clone)]
struct Node<T: Clone> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T: Clone + Debug + PartialEq> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator { current: self.head }
    }

    fn search(&self, v: T) -> bool {
        self.iter().find(|n| value(*n) == v).is_some()
    }

    fn push_front(&mut self, value: T) {
        let new_node = Some(unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                prev: None,
                next: self.head,
            })))
        });

        if self.head.is_none() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            *prev_mut(&mut self.head.unwrap()) = new_node;
            self.head = new_node;
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Some(unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                prev: self.tail,
                next: None,
            })))
        });

        if self.head.is_none() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            *next_mut(&mut self.tail.unwrap()) = new_node;
            self.tail = new_node;
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head == self.tail {
            let value = value(self.head.unwrap());
            self.head = None;
            self.head = None;
            return Some(value);
        }

        let head = self.head.unwrap();
        let value = value(head);
        let next = next(head);
        if let Some(mut next) = next {
            *prev_mut(&mut next) = None;
        }
        self.head = next;

        Some(value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }

        if self.tail == self.head {
            let value = value(self.tail.unwrap());
            self.head = None;
            self.tail = None;
            return Some(value);
        }

        let tail = self.tail.unwrap();
        let value = value(tail);
        let prev = prev(tail);
        if let Some(mut prev) = prev {
            *next_mut(&mut prev) = None;
        }
        self.tail = prev;

        Some(value)
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
    for (pos, &i) in [1, 7, 2, 20].iter().enumerate() {
        println!("Pushing element {}", i);
        if pos % 2 == 0 {
            list.push_front(i);
        } else {
            list.push_back(i);
        }
    }
    print!("Elements in list: ");
    for item in list.iter() {
        print!("{}, ", value(item));
    }
    println!();

    println!("Element 2 in list? {}", list.search(2));
    println!("{:?} removed from the front", list.pop_front());
    println!("Element 2 in list? {}", list.search(2));
    print!("Elements in list: ");
    for item in list.iter() {
        print!("{}, ", value(item));
    }
    println!();

    println!("{:?} removed from the back", list.pop_back());
    print!("Elements in list: ");
    for item in list.iter() {
        print!("{}, ", value(item));
    }
    println!();

    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());
    println!("{:?} removed", list.pop_back());

    println!("Pushing element 2");
    list.push_front(2);
    print!("Elements in list: ");
    for item in list.iter() {
        print!("{}, ", value(item));
    }
    println!();
}
