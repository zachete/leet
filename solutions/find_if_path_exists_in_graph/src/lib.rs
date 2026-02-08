use std::collections::VecDeque;

pub fn find_if_path_exists_in_graph(
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
) -> bool {
    if source == destination {
        return true;
    }

    let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

    for edge in &edges {
        let a = edge[0];
        let b = edge[1];

        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut visited = vec![false; n as usize];

    queue.push_back(source);
    visited[source as usize] = true;

    while let Some(current) = queue.pop_front() {
        for &neighbour in &adj[current as usize] {
            if visited[neighbour as usize] == true {
                continue;
            }

            if neighbour == destination {
                return true;
            }

            visited[neighbour as usize] = true;
            queue.push_back(neighbour);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        let result = find_if_path_exists_in_graph(n, edges, source, destination);

        println!("Result: {:?}", result);

        // assert_eq!(result, );
    }
}
