use std::{fmt::Display, ptr::NonNull};

use rand::Rng;

struct SkipList<T> {
    head: NonNull<Node<T>>,
    max_level: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    value: Option<T>,
    forward: Vec<Link<T>>,
}

fn value<T: Clone>(node: NonNull<Node<T>>) -> Option<T> {
    unsafe { (*node.as_ptr()).value.clone() }
}
fn forward<T>(node: NonNull<Node<T>>) -> Vec<Link<T>> {
    unsafe { (*node.as_ptr()).forward.clone() }
}
fn forward_mut<T: Clone>(link: &mut NonNull<Node<T>>) -> &mut [Link<T>] {
    unsafe { &mut (*link.as_ptr()).forward }
}

impl<T> SkipList<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + PartialOrd,
{
    fn build(max_level: usize) -> Self {
        Self {
            head: unsafe {
                NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                    value: None,
                    forward: vec![None; max_level],
                })))
            },
            max_level,
        }
    }

    fn search(&self, v: &T) -> bool {
        let mut node = self.head;
        for i in (0..self.max_level).rev() {
            let mut next = forward(node)[i];
            while next.and_then(value).is_some_and(|value| &value < v) {
                node = next.unwrap();
                next = forward(node)[i];
            }
        }

        let node = forward(node)[0];
        node.map(value).is_some_and(|n| n.as_ref() == Some(v))
    }

    fn insert(&mut self, v: &T) {
        let mut update = vec![None; self.max_level];
        let mut node = self.head;
        for i in (0..self.max_level).rev() {
            let mut next = forward(node)[i];
            while next.and_then(value).is_some_and(|value| &value < v) {
                node = next.unwrap();
                next = forward(node)[i];
            }
            update[i] = Some(node);
        }

        let node = forward(node)[0];
        if node.map(value).is_some_and(|n| n.as_ref() == Some(v)) {
            println!("{} is already in the list", v);
        }

        let level = rand::thread_rng().gen_range(0..self.max_level);
        let mut x = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value: Some(v.clone()),
                forward: vec![None; self.max_level],
            })))
        };
        for i in 0..=level {
            if update[i].is_none() {
                forward_mut(&mut x)[i] = forward(self.head)[i];
                forward_mut(&mut self.head)[i] = Some(x);
            } else {
                forward_mut(&mut x)[i] = forward(update[i].unwrap())[i];
                forward_mut(&mut update[i].unwrap())[i] = Some(x);
            }
        }
    }

    fn delete(&mut self, v: &T) {
        let mut update = vec![None; self.max_level];
        let mut node = self.head;
        for i in (0..self.max_level).rev() {
            let mut next = forward(node)[i];
            while next.and_then(value).is_some_and(|value| &value < v) {
                node = next.unwrap();
                next = forward(node)[i];
            }
            update[i] = Some(node);
        }

        let node = forward(node)[0];
        if node.map(value).is_some_and(|n| n.as_ref() == Some(v)) {
            for i in 0..self.max_level {
                if let Some(mut update_i) = update[i] {
                    if forward(update_i)[i] != node {
                        break;
                    } else {
                        forward_mut(&mut update_i)[i] = forward(node.unwrap())[i];
                    }
                }
            }
        }
    }
}

impl Display for SkipList<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut matrix = Vec::<Vec<i32>>::new();

        for i in 0..self.max_level {
            let mut row = vec![0; matrix.first().unwrap_or(&Vec::<i32>::new()).len()];
            if i == 0 {
                row.push(0);
            }

            let mut node = forward(self.head)[i];
            while let Some(v) = node.and_then(value) {
                node = forward(node.unwrap())[i];

                if i == 0 {
                    row.push(v);
                } else {
                    let j = matrix[0].iter().position(|a| a == &v).unwrap();
                    row[j] = v;
                }
            }

            if i == 0 {
                row.push(0);
            }
            matrix.push(row);
        }

        let format_line = |l: &Vec<i32>| {
            let mut new_l = l
                .iter()
                .map(|v| {
                    if v == &0 {
                        "------".to_string()
                    } else {
                        format!("{:^6}", v)
                    }
                })
                .collect::<Vec<String>>();
            let length = new_l.len();
            new_l[length - 1] = "None".to_string();
            new_l[0] = "None".to_string();
            new_l.join(" -> ")
        };

        let matrix = matrix
            .iter()
            .rev()
            .map(format_line)
            .collect::<Vec<String>>();

        write!(f, "{}", matrix.join("\n"))
    }
}

fn main() {
    let mut list = SkipList::build(6);
    let mut i = 0;

    let mut v = 0;
    while i < 10 {
        let value = rand::thread_rng().gen_range(0..1000);
        if i == 5 {
            v = value;
        }
        list.insert(&value);
        i += 1;
    }
    println!("{}", list);

    println!("\n\nSearch before deleting {} = {}", v, list.search(&v));
    list.delete(&v);
    println!("Search after deletnig {} = {}", v, list.search(&v));
    println!("\n\n{}", list);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_list_build() {
        let max_level = 10;
        let list = SkipList::<u8>::build(max_level);
        assert_eq!(max_level, forward(list.head).len());
    }

    #[test]
    fn test_search() {
        let node_a = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::from(Node {
                value: Some(12),
                forward: vec![None; 4],
            })))
        };
        let node_b = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::from(Node {
                value: Some(10),
                forward: vec![Some(node_a), None, None, None],
            })))
        };
        let node_c = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::from(Node {
                value: Some(7),
                forward: vec![Some(node_b), Some(node_a), None, None],
            })))
        };
        let node_d = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::from(Node {
                value: Some(2),
                forward: vec![Some(node_c), Some(node_b), None, None],
            })))
        };

        let mut values = SkipList::<u8>::build(4);
        forward_mut(&mut values.head)[0] = Some(node_d);
        forward_mut(&mut values.head)[1] = Some(node_d);
        forward_mut(&mut values.head)[2] = Some(node_b);

        let in_list = &[2u8, 7, 10, 12];
        for v in in_list {
            assert!(values.search(&v));
        }

        for v in 0..15 {
            if !in_list.contains(&v) {
                assert!(!values.search(&v));
            }
        }
    }

    #[test]
    fn test_insert() {
        let mut list = SkipList::build(4);

        let values = (0..100)
            .map(|_| rand::thread_rng().gen_range(0..1000))
            .collect::<HashSet<i32>>();
        for i in values {
            assert!(!list.search(&i));
            list.insert(&i);
            assert!(list.search(&i));
        }
    }

    #[test]
    fn test_delete() {
        let mut list = SkipList::build(4);

        for i in 0..100 {
            list.insert(&i);
        }
        for i in 0..100 {
            assert!(list.search(&i));
        }

        let values = (0..100)
            .map(|_| rand::thread_rng().gen_range(0..1000))
            .collect::<HashSet<i32>>();
        for i in &values {
            list.delete(&i);
        }
        for i in 0..100 {
            if values.contains(&i) {
                assert!(!list.search(&i));
            } else {
                assert!(list.search(&i));
            }
        }
    }
}
