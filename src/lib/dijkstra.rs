fn dijkstra(s: usize, N: usize, adj: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let inf = 1e8 as i64;
    let mut dist = vec![inf; N];
    dist[s] = 0;
    let mut que = BinaryHeap::new();
    que.push((0, s.to_owned()));
    let mut visited = vec![false; N];
    while !que.is_empty() {
        let (_, v) = que.pop().unwrap();
        visited[v] = true;
        for i in 0..adj[v].len() {
            let (to, cost) = adj[v][i];
            if cost + dist[v] < dist[to] && !visited[to] {
                dist[to] = dist[v] + cost;
                que.push((-dist[to], to));
            }
        }
    }
    dist
}
