use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use single_source_shortest_path_dag::graph_utils::*;
use single_source_shortest_path_dag::shortest_path::*;

fn main() {
    type SharedMutableNode<Node> = Rc<RefCell<Node>>;

    #[derive(PartialEq, Eq)]
    struct Node {
        id: String,
        distance: i8,
        parent: Option<SharedMutableNode<Node>>,
        satellite_data: u8,
    }

    impl Hash for Node {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl Constructor<Node, String> for Node {
        fn new(id: String) -> Node {
            Node {
                id: id,
                distance: i8::MAX,
                parent: None,
                satellite_data: 0,
            }
        }
    }

    impl Id<String> for Node {
        fn unique_id(&self) -> String {
            self.id.clone()
        }
    }

    impl Distance for Node {
        fn set_distance(&mut self, distance: i8) {
            self.distance = distance;
        }

        fn get_distance(&self) -> i8 {
            self.distance
        }
    }

    impl Parent<Node> for Node {
        fn set_parent(&mut self, parent: Option<&SharedMutableNode<Node>>) {
            if let Some(value) = parent {
                self.parent = Some(Rc::clone(value));
            } else {
                self.parent = None;
            }
        }

        fn get_parent(&self) -> &Option<SharedMutableNode<Node>> {
            &self.parent
        }
    }

    impl NodeBehaviour<String, Node> for Node {}

    let r = create_node::<Node, String>(String::from("r"));
    let s = create_node::<Node, String>(String::from("s"));
    let t = create_node::<Node, String>(String::from("t"));
    let x = create_node::<Node, String>(String::from("x"));
    let y = create_node::<Node, String>(String::from("y"));
    let z = create_node::<Node, String>(String::from("z"));

    let nodes = create_node_list(&[&r, &s, &t, &x, &y, &z]);

    let neighbours_lists = create_adjacency_lists(&[
        (&r, create_node_list(&[&s, &t])),
        (&s, create_node_list(&[&t, &x])),
        (&t, create_node_list(&[&x, &y, &z])),
        (&x, create_node_list(&[&y, &z])),
        (&y, create_node_list(&[&z])),
    ]);

    let graph = &Graph {
        nodes: Box::new(nodes),
        adjacency_list: neighbours_lists,
    };

    let weights = vec![vec![5, 3], vec![2, 6], vec![7, 4, 2], vec![-1, 1], vec![-2]];

    let origin = &s;
    dag_shortest_paths(graph, origin, &weights);

    for node in graph.nodes.iter() {
        if let Some(value) = node.borrow().get_parent() {
            println!(
                "Node \"{}\" parent is {}. Distance to parent = {}.",
                node.borrow().unique_id(),
                value.borrow().unique_id(),
                node.borrow().get_distance()
            );
        } else {
            print!(
                "Node \"{}\" does not have parent. ",
                node.borrow().unique_id()
            );
            if node.borrow().unique_id() == origin.borrow().unique_id() {
                print!("This node is the origin");
            }
            println!();
        }
    }
}
