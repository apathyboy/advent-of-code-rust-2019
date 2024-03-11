use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(6);

struct DAG {
    // Adjacency list representation: node name -> Vec of names of nodes it points to
    adj_list: HashMap<String, Vec<String>>,
    reverse_adj_list: HashMap<String, Vec<String>>,
}

impl DAG {
    fn new() -> Self {
        DAG {
            adj_list: HashMap::new(),
            reverse_adj_list: HashMap::new(),
        }
    }

    // Add a node to the graph
    fn add_node(&mut self, name: &str) {
        self.adj_list
            .entry(name.to_string())
            .or_insert_with(Vec::new);
    }

    // Add a directed edge from node `from` to node `to`
    // This implicitly adds the nodes to the graph if they don't exist
    fn add_edge(&mut self, from: &str, to: &str) {
        // Since all weights are 1, we don't store the weight explicitly
        self.add_node(from); // Ensure the node exists
        self.add_node(to); // Ensure the node exists

        // Add the edge
        if let Some(nodes) = self.adj_list.get_mut(from) {
            nodes.push(to.to_string());
        }

        self.reverse_adj_list
            .entry(to.into())
            .or_default()
            .push(from.into());
    }

    // Calculate depths of all nodes
    fn calculate_depths(&self, root: &str) -> HashMap<String, usize> {
        let mut depths = HashMap::new();
        let mut queue = VecDeque::new();

        // Initialize the BFS from the root
        depths.insert(root.to_string(), 0);
        queue.push_back(root.to_string());

        while let Some(node) = queue.pop_front() {
            let node_depth = depths[&node];

            // Visit all neighbors
            if let Some(neighbors) = self.adj_list.get(&node) {
                for neighbor in neighbors {
                    // If the neighbor is not visited yet
                    if !depths.contains_key(neighbor) {
                        queue.push_back(neighbor.to_string());
                        depths.insert(neighbor.to_string(), node_depth + 1);
                    }
                }
            }
        }

        depths
    }
}

struct Graph2 {
    // Use a HashMap to store adjacency list, representing an undirected graph
    adj_list: HashMap<String, Vec<String>>,
}

impl Graph2 {
    fn new() -> Self {
        Graph2 {
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

    // BFS to find the shortest path in an undirected graph
    fn shortest_path_undirected(&self, source: &str, target: &str) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new(); // Track visited nodes and their distances from source

        visited.insert(source.to_string(), 0);
        queue.push_back(source.to_string());

        while let Some(current) = queue.pop_front() {
            let distance = visited[&current];

            if current == target {
                // Found the target node
                return Some(distance);
            }

            if let Some(neighbors) = self.adj_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        visited.insert(neighbor.to_string(), distance + 1);
                        queue.push_back(neighbor.to_string());
                    }
                }
            }
        }

        None // Target not reachable from source
    }
}

fn parse_edge(line: &str) -> (&str, &str) {
    line.split_once(')').unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = DAG::new();

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
    let mut graph = Graph2::new();

    for line in input.lines() {
        let (parent, child) = parse_edge(line);
        graph.add_edge(parent, child);
    }

    if let Some(distance) = graph.shortest_path_undirected("YOU", "SAN") {
        Some(distance - 2)
    } else {
        None
    }
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
