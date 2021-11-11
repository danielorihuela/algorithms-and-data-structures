use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;

pub(crate) type SharedMutableNode<T> = Rc<RefCell<T>>;

pub trait Constructor<T, U> {
	fn new(id: U) -> T;
}

pub trait Id<T> {
    fn unique_id(&self) -> T;
}

pub struct Graph<T, U> {
    pub nodes: Box<Vec<SharedMutableNode<T>>>,
    pub adjacency_list: HashMap<U, Vec<SharedMutableNode<T>>>,
}

pub fn create_node<T: Constructor<T, U> + Id<U>, U>(id: U) -> SharedMutableNode<T> {
    Rc::new(RefCell::new(T::new(id))) 
}

pub fn create_node_list<T>(nodes: &[&SharedMutableNode<T>]) -> Vec<SharedMutableNode<T>> {
    nodes
        .iter()
        .map(|x| Rc::clone(x))
        .collect::<Vec<SharedMutableNode<T>>>()
}

pub fn create_adjacency_lists<U: Hash + Eq, T: Hash + Eq + Id<U>>(
    node_neighbours: &[(&SharedMutableNode<T>, Vec<SharedMutableNode<T>>)],
) -> HashMap<U, Vec<SharedMutableNode<T>>> {
    node_neighbours
        .iter()
        .map(|(node, neighbours)| (node.borrow().unique_id(), neighbours.clone()))
        .collect::<HashMap<U, Vec<SharedMutableNode<T>>>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_graph() {
	    #[derive(PartialEq, Eq, Hash)]
	    struct Node {
	        value: u8,
	    }

		impl Constructor<Node, u8> for Node {
			fn new(id: u8) -> Node {
			    Node {value: id}
			}
		}
 
	    impl Id<u8> for Node {
	        fn unique_id(&self) -> u8 {
		        return self.value.clone()
		    }
	    }

        let node_one = create_node::<Node, u8>(1);
        let node_two = create_node::<Node, u8>(2);

        let nodes = create_node_list(&[
            &node_one,
            &node_two,
        ]);

        let neighbours_lists = create_adjacency_lists(&[
            (&node_one, create_node_list(&[&node_two])),
            (&node_two, create_node_list(&[])),
        ]);

        let graph = &Graph {
            nodes: Box::new(nodes),
            adjacency_list: neighbours_lists,
        };

		assert_eq!(graph.nodes[0].borrow().unique_id(), 1);
	}
}