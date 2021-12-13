use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let graph = parse(path).unwrap();

    let res_1 = solve(false, &graph);
    let res_2 = solve(true, &graph);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parse(path: &Path) -> io::Result<Graph> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut g = Graph::new();

    for line in buf.lines() {
        g.insert_edge(&line.unwrap());
    }

    Ok(g)
}

fn solve(can_revisit: bool, graph: &Graph) -> u32 {
    graph.dfs(0, can_revisit)
}

#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>,
    label_map: HashMap<String, usize>,
    vertex_map: HashMap<usize, String>,
    is_small: Vec<bool>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: vec![],
            label_map: HashMap::new(),
            vertex_map: HashMap::new(),
            is_small: vec![],
        }
    }

    fn insert_edge(&mut self, edge: &str) {
        let (start_label, end_label) = {
            let x = edge.split_terminator("-").collect::<Vec<&str>>();
            (x[0], x[1])
        };

        let start_idx = self.insert_node(start_label);
        let end_idx = self.insert_node(end_label);

        self.adj_list[start_idx].push(end_idx);
        self.adj_list[end_idx].push(start_idx);
    }

    fn insert_node(&mut self, label: &str) -> usize {
        if let Some(idx) = self.label_map.get(label) {
            return *idx;
        }
        let idx = self.adj_list.len();

        self.label_map.insert(label.to_string(), idx);
        self.vertex_map.insert(idx, label.to_string());

        self.adj_list.push(vec![]);

        self.is_small.push(label.starts_with(|c: char| c.is_lowercase()));

        idx
    }

    fn get_neighbours(&self, vertex: usize) -> &Vec<usize> {
        &self.adj_list[vertex]
    }

    fn is_start(&self, vertex: usize) -> bool {
        "start" == self.vertex_map.get(&vertex).unwrap()
    }

    fn is_end(&self, vertex: usize) -> bool {
        "end" == self.vertex_map.get(&vertex).unwrap()
    }

    fn is_small(&self, vertex: usize) -> bool {
        self.is_small[vertex]
    }

    fn dfs(&self, start: usize, can_revisit: bool) -> u32 {
        fn helper(g: &Graph, vertex: usize, marked: &mut Vec<bool>, can_revisit: bool, can_unmark: bool) -> u32 {
            if g.is_end(vertex) {
                return 1;
            }

            if g.is_small(vertex) {
                marked[vertex] = true;
            }

            let mut result = 0;

            for nbr in g.get_neighbours(vertex) {
                if marked[*nbr] && can_revisit && !g.is_start(*nbr) {
                    result += helper(g, *nbr, marked, false, false);

                }
                if !marked[*nbr] {
                    result += helper(g, *nbr, marked, can_revisit, true);
                }
            }

            if can_unmark {
                marked[vertex] = false;
            }

            result
        }

        helper(self, start, &mut vec![false; self.adj_list.len()], can_revisit, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path_1 = Path::new("test_input_1.txt");
        let graph_1 = parse(path_1).unwrap();
        let can_revisit = false;
        let res = solve(can_revisit, &graph_1);
        assert_eq!(10, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input_1.txt");
        let graph = parse(path).unwrap();
        let can_revisit = true;
        let res = solve(can_revisit, &graph);
        assert_eq!(36, res);
    }
}
