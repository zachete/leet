pub fn find_if_path_exists_in_graph(
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
) -> Vec<Vec<i32>> {
    let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

    for edge in &edges {
        let a = edge[0];
        let b = edge[1];

        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }

    // TODO: implement main logic

    adj
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
