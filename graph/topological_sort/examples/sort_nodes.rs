use topological_sort::graph_utils::*;
use topological_sort::sort::*;

fn main() {
    #[derive(PartialEq, Eq, Hash)]
    struct Node {
        id: String,
        value: u8,
    }

    impl Constructor<Node, String> for Node {
        fn new(id: String) -> Node {
            Node { id: id, value: 0 }
        }
    }

    impl Id<String> for Node {
        fn unique_id(&self) -> String {
            return self.id.clone();
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

    for node in sorted_nodes {
        println!(
            "Node \"{}\" has value {}",
            node.borrow().unique_id(),
            node.borrow().value
        );
    }
}
