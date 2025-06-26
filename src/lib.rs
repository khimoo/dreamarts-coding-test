use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub distance: f64,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, distance: f64) {
        self.nodes.entry(from).or_insert_with(Vec::new).push(Edge { to, distance });
    }

    pub fn get_all_nodes(&self) -> Vec<usize> {
        let mut nodes = HashSet::new();
        for (&from, edges) in &self.nodes {
            nodes.insert(from);
            for edge in edges {
                nodes.insert(edge.to);
            }
        }
        nodes.into_iter().collect()
    }

    pub fn find_longest_path(&self) -> (Vec<usize>, f64) {
        let all_nodes = self.get_all_nodes();
        let mut best_path = Vec::new();
        let mut best_distance = 0.0;

        for &start in &all_nodes {
            let mut visited = HashSet::new();
            let mut current_path = Vec::new();
            let (path, distance) = self.dfs(start, &mut visited, &mut current_path);
            if distance > best_distance {
                best_distance = distance;
                best_path = path;
            }
        }

        (best_path, best_distance)
    }

    fn dfs(&self, node: usize, visited: &mut HashSet<usize>, current_path: &mut Vec<usize>) -> (Vec<usize>, f64) {
        visited.insert(node);
        current_path.push(node);

        let mut best_path = current_path.clone();
        let mut best_distance = 0.0;

        if let Some(edges) = self.nodes.get(&node) {
            for edge in edges {
                if !visited.contains(&edge.to) {
                    let (path, distance) = self.dfs(edge.to, visited, current_path);
                    let total_distance = edge.distance + distance;
                    if total_distance > best_distance {
                        best_distance = total_distance;
                        best_path = path;
                    }
                }
            }
        }

        visited.remove(&node);
        current_path.pop();

        (best_path, best_distance)
    }
}

pub fn parse_input(input: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let mut graph = Graph::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid line format: {}", line).into());
        }
        let from: usize = parts[0].trim().parse()?;
        let to: usize = parts[1].trim().parse()?;
        let distance: f64 = parts[2].trim().parse()?;
        graph.add_edge(from, to, distance);
    }
    Ok(graph)
}
