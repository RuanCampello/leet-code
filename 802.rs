use std::collections::HashMap;

pub fn is_safe(graph: &Vec<Vec<i32>>, safe_nodes: &mut HashMap<usize, bool>, idx: usize) -> bool {
    if let Some(is_safe) = safe_nodes.get(&idx) {
        return *is_safe;
    }

    safe_nodes.insert(idx, false); // mark initially as false in case of cycles through the path

    for &neighbour in &graph[idx] {
        if !is_safe(graph, safe_nodes, neighbour as _) {
            return *(safe_nodes.get(&idx)).unwrap_or(&false); // false actually
        }
    }

    safe_nodes.insert(idx, true); // if all neighbours are safe, it's safe!
    true
}

pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut safe_nodes: HashMap<usize, bool> = HashMap::new();

    for i in 0..graph.len() {
        if is_safe(&graph, &mut safe_nodes, i) {
            result.push(i as _);
        }
    }

    result
}

fn main() {
    assert_eq!(
        eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ]),
        vec![2, 4, 5, 6]
    )
}
