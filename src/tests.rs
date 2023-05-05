use super::*;

#[test]
fn test_bfs() {
    let edges = vec![(1, 2), (2, 3), (2, 4), (3, 4), (4, 5), (5, 6)];
    let adj_list = adjacency_list(&edges);
    let expected = {
        let mut map = HashMap::new();
        map.insert(1, 0);
        map.insert(2, 1);
        map.insert(3, 2);
        map.insert(4, 2);
        map.insert(5, 3);
        map.insert(6, 4);
        map
    };
    let result = bfs(1, &adj_list);
    assert_eq!(result, expected);
}