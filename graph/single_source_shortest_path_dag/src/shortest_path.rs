use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

pub use topological_sort::graph_utils::{Graph, Id};
use topological_sort::sort::topological_sort;

type SharedMutableNode<U> = Rc<RefCell<U>>;

pub trait Distance {
    fn set_distance(&mut self, distance: i8);
    fn get_distance(&self) -> i8;
}

pub trait Parent<T> {
    fn set_parent(&mut self, parent: Option<&SharedMutableNode<T>>);
    fn get_parent(&self) -> &Option<SharedMutableNode<T>>;
}

pub trait NodeBehaviour<T, U>: Id<T> + Distance + Parent<U> {}

pub fn dag_shortest_paths<U: Hash + Eq, T: Hash + Eq + NodeBehaviour<U, T>>(
    graph: &Graph<T, U>,
    origin: &SharedMutableNode<T>,
    weights: &Vec<Vec<i8>>,
) {
    initialize_single_source(graph, origin);
    for (i, u_ref) in topological_sort(graph).iter().enumerate() {
        let neighbours = graph.adjacency_list.get(&u_ref.borrow().unique_id());
        if let None = neighbours {
            continue;
        }

        for (j, v_ref) in neighbours.unwrap().iter().enumerate() {
            relax(u_ref, v_ref, weights[i][j]);
        }
    }
}

fn initialize_single_source<U: Hash + Eq, T: Hash + Eq + NodeBehaviour<U, T>>(
    graph: &Graph<T, U>,
    origin: &SharedMutableNode<T>,
) {
    for node in graph.nodes.iter() {
        node.borrow_mut().set_distance(i8::MAX);
        node.borrow_mut().set_parent(None);
    }

    origin.borrow_mut().set_distance(0);
}

fn relax<T: Distance + Parent<T>>(u: &SharedMutableNode<T>, v: &SharedMutableNode<T>, weight: i8) {
    if i16::from(v.borrow().get_distance())
        > i16::from(u.borrow().get_distance()) + i16::from(weight)
    {
        v.borrow_mut()
            .set_distance(u.borrow().get_distance() + weight);
        v.borrow_mut().set_parent(Some(u));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hasher;
    use topological_sort::graph_utils::{
        create_adjacency_lists, create_node, create_node_list, Constructor,
    };

    #[test]
    fn dag_shortest_paths_work() {
        #[derive(PartialEq, Eq)]
        struct Node {
            id: String,
            distance: i8,
            parent: Option<SharedMutableNode<Node>>,
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

        dag_shortest_paths(graph, &s, &weights);

        assert_eq!(r.borrow().get_distance(), i8::MAX);
        assert_eq!(s.borrow().get_distance(), 0);
        assert_eq!(t.borrow().get_distance(), 2);
        assert_eq!(x.borrow().get_distance(), 6);
        assert_eq!(y.borrow().get_distance(), 5);
        assert_eq!(z.borrow().get_distance(), 3);

        assert!(r.borrow().get_parent().is_none());
        assert!(s.borrow().get_parent().is_none());

        let get_unique_id_from = |node: SharedMutableNode<Node>| {
            node.borrow()
                .get_parent()
                .as_ref()
                .unwrap()
                .borrow()
                .unique_id()
        };
        assert_eq!(get_unique_id_from(t), "s");
        assert_eq!(get_unique_id_from(x), "s");
        assert_eq!(get_unique_id_from(y), "x");
        assert_eq!(get_unique_id_from(z), "y");
    }
}
