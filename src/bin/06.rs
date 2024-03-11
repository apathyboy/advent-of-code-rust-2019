use pathfinding::prelude::bfs;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{VisitMap, Visitable};
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(6);

struct DirectedGraph {
    graph: DiGraph<String, ()>, // Use unit type for edge weights since they're not used
    name_to_index: HashMap<String, NodeIndex>,
}

impl DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            graph: DiGraph::new(),
            name_to_index: HashMap::new(),
        }
    }

    fn add_node(&mut self, name: &str) -> NodeIndex {
        let name_string = name.to_string();
        *self
            .name_to_index
            .entry(name_string.clone())
            .or_insert_with(|| self.graph.add_node(name_string))
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let from_index = self.add_node(from);
        let to_index = self.add_node(to);
        self.graph.add_edge(from_index, to_index, ());
    }

    fn calculate_depths(&self, root: &str) -> HashMap<String, usize> {
        let mut depths = HashMap::new();
        if let Some(&root_index) = self.name_to_index.get(root) {
            let mut queue = VecDeque::new();
            let mut visited = self.graph.visit_map();

            queue.push_back((root_index, 0)); // NodeIndex and depth
            visited.visit(root_index);

            while let Some((node_index, depth)) = queue.pop_front() {
                let node_name = &self.graph[node_index];
                depths.insert(node_name.clone(), depth);

                for neighbor in self.graph.neighbors(node_index) {
                    if !visited.is_visited(&neighbor) {
                        queue.push_back((neighbor, depth + 1));
                        visited.visit(neighbor);
                    }
                }
            }
        }
        depths
    }
}

struct UndirectedGraph {
    // Use a HashMap to store adjacency list, representing an undirected graph
    adj_list: HashMap<String, Vec<String>>,
}

impl UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph {
            adj_list: HashMap::new(),
        }
    }

    // Modified add_edge function to ignore the direction by adding both directions
    fn add_edge(&mut self, from: &str, to: &str) {
        self.adj_list
            .entry(from.into())
            .or_default()
            .push(to.into());
        self.adj_list
            .entry(to.into())
            .or_default()
            .push(from.into()); // Add the reverse edge
    }

    // Use BFS from the pathfinding crate to find the shortest path
    fn shortest_path(&self, start: &str, goal: &str) -> Option<Vec<String>> {
        let start = start.to_owned();
        let goal = goal.to_owned();
        bfs(
            &start,
            |p| self.neighbors(p), // Expand function
            |p| *p == goal,        // Success function
        )
    }

    // Function to get neighbors of a node
    fn neighbors(&self, node: &str) -> Vec<String> {
        self.adj_list.get(node).cloned().unwrap_or_else(Vec::new)
    }
}

fn parse_edge(line: &str) -> (&str, &str) {
    line.split_once(')').unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = DirectedGraph::new();

    for line in input.lines() {
        let (parent, child) = parse_edge(line);
        graph.add_node(parent);
        graph.add_node(child);

        graph.add_edge(parent, child);
    }

    let depths = graph.calculate_depths("COM");

    let mut orbits = 0;
    for (_, depth) in depths {
        orbits += depth;
    }

    Some(orbits)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut graph = UndirectedGraph::new();

    for line in input.lines() {
        let (parent, child) = parse_edge(line);
        graph.add_edge(parent, child);
    }

    graph
        .shortest_path("YOU", "SAN")
        .map(|distance| distance.len() - 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
