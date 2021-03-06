use std::collections::HashMap;

use crate::reader::split_lines;

type Node = String;
type NodeList = Vec<String>;
type Path = (Vec<String>, bool);
type EdgeMap = HashMap<Node, NodeList>;

enum NodeType {
    Start,
    End,
    Small,
    Big,
}

fn node_type(node: &str) -> NodeType {
    match node {
        "start" => NodeType::Start,
        "end" => NodeType::End,
        x if x.chars().all(char::is_lowercase) => NodeType::Small,
        _ => NodeType::Big,
    }
}

fn append(new: &str, path: &Path) -> Path {
    let (mut p, d) = path.clone();
    p.push(new.to_string());
    (p, d)
}

fn find_paths(input: &str, allow_second_small: bool) -> Vec<Path> {
    let lines = split_lines(input);
    let mut edges = EdgeMap::new();
    for line in lines {
        parse(&line, &mut edges);
    }
    let mut routes: Vec<Path> = edges
        .get("start")
        .unwrap()
        .iter()
        .map(|n| (vec!["start".to_string(), n.to_string()], false))
        .collect();
    let mut finished_routes = Vec::<Path>::new();
    while !routes.is_empty() {
        let cur_path = routes.pop().unwrap();
        let cur_node = cur_path.0.last().unwrap();
        let outgoing = edges.get(cur_node).unwrap();
        for next_node in outgoing.iter() {
            let node_type = node_type(next_node);
            match node_type {
                NodeType::Start => {}
                NodeType::End => {
                    let p = append(next_node, &cur_path);
                    finished_routes.push(p);
                }
                NodeType::Small => {
                    let (p, has_second) = &cur_path;
                    if allow_second_small {
                        let in_cur_path = p.contains(next_node);
                        let do_insert = !in_cur_path || (in_cur_path && !*has_second);
                        if do_insert {
                            let (p, _) = append(next_node, &cur_path);
                            routes.push((p, in_cur_path || *has_second));
                        }
                    } else {
                        if !p.contains(next_node) {
                            let p = append(next_node, &cur_path);
                            routes.push(p);
                        }
                    }
                }
                NodeType::Big => {
                    let p = append(next_node, &cur_path);
                    routes.push(p);
                }
            }
        }
    }

    finished_routes
}

fn parse(input: &str, edges: &mut EdgeMap) {
    let mut matches = input.split("-");
    let node_1 = matches.next().unwrap().to_string();
    let node_2 = matches.next().unwrap().to_string();
    {
        let list_1 = edges
            .entry(node_1.clone())
            .or_insert_with(|| NodeList::new());
        list_1.push(node_2.to_string());
    }
    {
        let list_2 = edges
            .entry(node_2.clone())
            .or_insert_with(|| NodeList::new());
        list_2.push(node_1.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_single() {
        let input = "start-A";
        let mut edges = EdgeMap::new();
        parse(input, &mut edges);

        assert_eq!(vec!["A".to_string()], *edges.get("start").unwrap());
        assert_eq!(vec!["start".to_string()], *edges.get("A").unwrap());
    }

    #[test]
    fn part_one_small() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(10, find_paths(input, false).len());
    }

    #[test]
    fn part_one() {
        let input = "FK-gc
gc-start
gc-dw
sp-FN
dw-end
FK-start
dw-gn
AN-gn
yh-gn
yh-start
sp-AN
ik-dw
FK-dw
end-sp
yh-FK
gc-gn
AN-end
dw-AN
gn-sp
gn-FK
sp-FK
yh-gc";
        assert_eq!(3713, find_paths(input, false).len());
    }

    #[test]
    fn part_two_small() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(36, find_paths(input, true).len());
    }

    #[test]
    fn part_two() {
        let input = "FK-gc
gc-start
gc-dw
sp-FN
dw-end
FK-start
dw-gn
AN-gn
yh-gn
yh-start
sp-AN
ik-dw
FK-dw
end-sp
yh-FK
gc-gn
AN-end
dw-AN
gn-sp
gn-FK
sp-FK
yh-gc";
        assert_eq!(91292, find_paths(input, true).len());
    }
}
