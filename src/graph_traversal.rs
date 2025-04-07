use std::collections::{HashMap, HashSet};

fn dfs(node: &str, graph: &HashMap<&str, Vec<&str>>, visited: &mut HashSet<String>) {
    if !visited.contains(node) {
        print!("{} {} ", if visited.is_empty() { "" } else { "->" }, node);
        visited.insert(node.to_string());
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs(neighbor, graph, visited);
            }
        }
    }
}
pub fn test_dfs() {
    let graph: HashMap<&str, Vec<&str>> = HashMap::from([
        ("A", vec!["B", "C"]),
        ("B", vec!["D", "E"]),
        ("C", vec!["F"]),
        ("D", vec![]),
        ("E", vec!["F"]),
        ("F", vec![]),
    ]);
    let mut visited: HashSet<String> = HashSet::new();
    println!("Depth First Search Traversal:");
    dfs("A", &graph, &mut visited);
    println!("\n")
}
