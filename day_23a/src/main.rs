use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub name: &'a str,
    pub connections: HashSet<&'a str>,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            connections: HashSet::new()
        }
    }

    pub fn add_connection(&mut self, other: &'a str) {
        self.connections.insert(other);
    }

    pub fn connect(&mut self, other: &mut Self) {
        self.add_connection(other.name);
        other.add_connection(self.name);
    }
}

type Graph<'a> = HashMap<&'a str, Node<'a>>;

fn new_graph<'a>(input: &'a str) -> Graph<'a> {
    let mut graph = Graph::new();

    for line in input.split('\n') {
        let mut parts = line.split('-');
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let has_left = graph.contains_key(left);
        let has_right = graph.contains_key(right);

        if has_left && has_right {
            graph.get_mut(left).unwrap().add_connection(right);
            graph.get_mut(right).unwrap().add_connection(left);
        } else if has_left {
            let mut right_n = Node::new(right);

            graph.get_mut(left).unwrap().connect(&mut right_n);

            graph.insert(right, right_n);
        } else if has_right {
            let mut left_n = Node::new(left);

            graph.get_mut(right).unwrap().connect(&mut left_n);

            graph.insert(left, left_n);
        } else {
            let mut left_n = Node::new(left);
            let mut right_n = Node::new(right);

            left_n.connect(&mut right_n);

            graph.insert(left, left_n);
            graph.insert(right, right_n);
        }
    }

    graph
}

#[derive(Debug, Clone)]
pub struct Threes<'a> {
    set: HashSet<(&'a str, &'a str, &'a str)>
}

impl<'a, 'b> Threes<'a> {
    pub fn new() -> Self {
        Self {
            set: HashSet::new()
        }
    }

    pub fn insert(&mut self, item: (&'a str, &'a str, &'a str)) -> bool {
        let mut vec_form = vec![item.0, item.1, item.2];
        vec_form.sort();
        let new_item = (vec_form[0], vec_form[1], vec_form[2]);

        self.set.insert(new_item)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }
}

#[allow(unused)]
fn find_t_threes<'a>(graph: Graph<'a>) -> Threes<'a> {
    let mut ts = Vec::new();

    for &node_name in graph.keys() {
        if node_name.starts_with('t') {
            ts.push(node_name);
        }
    }

    let mut threes = Threes::new();

    for t in ts {
        let node = graph.get(t).unwrap();

        for &c in &node.connections {
            let node = graph.get(c).unwrap();
            for &i in &node.connections {
                let node = graph.get(i).unwrap();
                if let Some(_) = node.connections.iter().find(|&&s|s==t) {
                    threes.insert((t, c, i));
                }
            }
        }
    }

    threes
}

fn find_group(graph: &Graph, id: &str, current_members: &mut Vec<String>) {
    let node = graph.get(id).unwrap();

    let mut new_members = vec![];

    for con in &node.connections {
        let node = graph.get(con).unwrap();

        if current_members.iter().filter(|&m| {
            !node.connections.contains(m.as_str())
        }).collect::<Vec<_>>().len() == 0 {
            current_members.push(con.to_string());
            new_members.push(con);
        }
    }

    for &con in new_members {
        find_group(graph, con, current_members);
    }
}

fn find_max_group(graph: Graph) -> Vec<String> {
    let mut max_group = vec![];

    let len = graph.len();

    let mut checked = 0;
    for n in graph.keys() {
        let mut group = Vec::new();
        find_group(&graph, n, &mut group);
        if group.len() > max_group.len() {
            max_group = group;
        }
        checked += 1;
        println!("{}/{}", checked, len);
    }

    return max_group;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let graph = new_graph(&input);

    let mut max_g = find_max_group(graph);
    max_g.sort();

    println!("{}", max_g.join(","));
    //println!("{:#?}", threes);
}
