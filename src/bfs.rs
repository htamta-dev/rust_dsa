use std::{collections::{HashMap, HashSet, VecDeque}, vec};
fn bfs<T: std::cmp::Eq+ std::hash::Hash+Copy>(graph: &mut HashMap<T, Vec<T>>, start: T)->Vec<T>{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start);
    visited.insert(start);
    while !queue.is_empty() {
        if let Some(vertex) = queue.pop_front(){
            result.push(vertex);
            if let Some(neighbours) = graph.get(&vertex){
                for neighbour in neighbours{
                    if !visited.contains(neighbour){
                        queue.push_back(*neighbour);
                        visited.insert(*neighbour);
                    }
                }
            }
        }
    }
    result
}

pub fn test_bfs(){
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    graph.insert('A', vec!['B', 'C']);
    graph.insert('B', vec!['A', 'D', 'E']);
    graph.insert('C', vec!['A', 'F']);
    graph.insert('D', vec!['B']);
    graph.insert('E', vec!['B', 'F']);
    graph.insert('F', vec!['C', 'E']);
    let result = bfs(&mut graph, 'A');
    println!("Breadth First Search Traversal:");
    let mut first = true;
    for node in result {
        if !first {
            print!(" -> ");
        }
        print!("{}", node);
        first = false;
    }
    println!();
}