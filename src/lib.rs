use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Edge {
    pub endpoints: (usize, usize),
    pub distance: f64,
}

impl Edge {
    /// この辺の、指定したノードから見た反対側の端点を返す
    pub fn other(&self, node: usize) -> usize {
        if self.endpoints.0 == node {
            self.endpoints.1
        } else {
            self.endpoints.0
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<Edge>,
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, distance: f64) {
        if from == to {
            return;
        }
        let idx = self.edges.len();
        self.edges.push(Edge {
            endpoints: (from, to),
            distance,
        });
        self.adjacency.entry(from).or_insert_with(Vec::new).push(idx);
        self.adjacency.entry(to).or_insert_with(Vec::new).push(idx);
    }

    pub fn get_all_nodes(&self) -> Vec<usize> {
        self.adjacency.keys().copied().collect()
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

        if let Some(edge_indices) = self.adjacency.get(&node) {
            for &idx in edge_indices {
                let edge = &self.edges[idx];
                let next = edge.other(node);
                if !visited.contains(&next) {
                    let (path, distance) = self.dfs(next, visited, current_path);
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
