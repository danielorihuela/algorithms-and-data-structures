use std::rc::Rc;
use std::collections::VecDeque;
use std::hash::Hash;

use crate::graph_utils::*;

pub fn topological_sort<U: Hash + Eq, T: Hash + Eq + Id<U>>(graph: &Graph<T, U>) -> VecDeque<SharedMutableNode<T>> {
    let nodes = &graph.nodes;

	let mut discovered = vec![];
	let mut sorted_nodes = VecDeque::new();
    for u in nodes.iter() {
		if !discovered.contains(&u.borrow().unique_id()) {
            dfs_visit(graph, u, &mut discovered, &mut sorted_nodes);
		}
    }

	sorted_nodes
}

fn dfs_visit<U: Hash + Eq, T: Hash + Eq + Id<U>>(graph: &Graph<T, U>, u: &SharedMutableNode<T>, discovered: &mut Vec<U>, sorted_nodes: &mut VecDeque<SharedMutableNode<T>>) {
    let neighbours = graph.adjacency_list.get(&u.borrow_mut().unique_id());
    if let Some(nodes) = neighbours {
        for v in nodes.iter() {
			if !discovered.contains(&v.borrow().unique_id()) {
			    discovered.push(v.borrow().unique_id());
                dfs_visit(graph, v, discovered, sorted_nodes);
			}
        }
    }

	sorted_nodes.push_front(Rc::clone(&u));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_works() {
	    #[derive(PartialEq, Eq, Hash)]
	    struct Node {
	        id: String,
			value: u8
	    }

		impl Constructor<Node, String> for Node {
			fn new(id: String) -> Node {
			    Node {id: id, value: 0}
			}
		}
 
	    impl Id<String> for Node {
	        fn unique_id(&self) -> String {
		        return self.id.clone()
		    }
	    }

        let socks = create_node::<Node, String>(String::from("socks"));
		socks.borrow_mut().value = 3;
        let undershorts = create_node::<Node, String>(String::from("undershorts"));
        let pants = create_node::<Node, String>(String::from("pants"));
        let shoes = create_node::<Node, String>(String::from("shoes"));
        let watch = create_node::<Node, String>(String::from("watch"));
        let shirt = create_node::<Node, String>(String::from("shirt"));
        let belt = create_node::<Node, String>(String::from("belt"));
        let tie = create_node::<Node, String>(String::from("tie"));
        let jacket = create_node::<Node, String>(String::from("jacket"));

        let nodes = create_node_list(&[
            &shirt,
            &watch,
            &undershorts,
            &socks,
            &tie,
            &jacket,
            &belt,
            &pants,
            &shoes,
        ]);

        let neighbours_lists = create_adjacency_lists(&[
            (&undershorts, create_node_list(&[&pants, &shoes])),
            (&socks, create_node_list(&[&shoes])),
            (&pants, create_node_list(&[&shoes, &belt])),
            (&watch, create_node_list(&[])),
            (&shoes, create_node_list(&[])),
            (&shirt, create_node_list(&[&tie, &belt])),
            (&belt, create_node_list(&[&jacket])),
            (&tie, create_node_list(&[&jacket])),
        ]);

        let graph = &Graph {
            nodes: Box::new(nodes),
            adjacency_list: neighbours_lists,
        };

        let sorted_nodes = topological_sort(graph);

    	let actual_sorted_nodes = sorted_nodes
			.iter()
        	.map(|x| x.borrow().unique_id())
        	.collect::<Vec<String>>();
		let expected_sorted_nodes = ["socks", "undershorts", "pants", "shoes", "watch", "shirt", "belt", "tie", "jacket"];
		assert_eq!(actual_sorted_nodes, expected_sorted_nodes);

    	let actual_sorted_value = sorted_nodes
			.iter()
        	.map(|x| x.borrow().value)
        	.collect::<Vec<u8>>();
		let expected_sorted_value = [3, 0, 0, 0, 0, 0, 0, 0, 0];
		assert_eq!(actual_sorted_value, expected_sorted_value);
	}
}
