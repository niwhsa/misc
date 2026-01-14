use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse default max-heap ordering to create a min-heap.
        // Dijkstra greedily processes the lowest-cost node first.
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(adj_list: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj_list.len()];
    let mut pq = BinaryHeap::new();

    dist[start] = 0;
    pq.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = pq.pop() {
        if cost > dist[position] { continue; }

        for &(neigh, weight) in &adj_list[position] {
            let new_cost = cost + weight;
            if new_cost >= dist[neigh] { continue; }

            dist[neigh] = new_cost;
            pq.push(State {
                cost: new_cost,
                position: neigh,
            });
        }
    }
    dist
}

fn main() {
    // Graph:
    //     1
    //   0 --- 1
    //   |     |
    // 4 |     | 2
    //   |     |
    //   2 --- 3
    //     1
    let graph = vec![
        vec![(1, 1), (2, 4)], // 0 -> 1 (cost 1), 0 -> 2 (cost 4)
        vec![(0, 1), (3, 2)], // 1 -> 0 (cost 1), 1 -> 3 (cost 2)
        vec![(0, 4), (3, 1)], // 2 -> 0 (cost 4), 2 -> 3 (cost 1)
        vec![(1, 2), (2, 1)], // 3 -> 1 (cost 2), 3 -> 2 (cost 1)
    ];

    let distances = dijkstra(&graph, 0);

    assert_eq!(distances, vec![0, 1, 4, 3]); // 0->0=0, 0->1=1, 0->2=3 (via 1->3->2), 0->3=3 (via 1)
    println!("Distances from node 0: {:?}", distances);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_graph() {
        // 0 --1-- 1 --2-- 2
        let graph = vec![vec![(1, 1)], vec![(0, 1), (2, 2)], vec![(1, 2)]];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, 3]);
    }

    #[test]
    fn test_single_node() {
        let graph = vec![vec![]];
        assert_eq!(dijkstra(&graph, 0), vec![0]);
    }

    #[test]
    fn test_disconnected() {
        // 0 -- 1    2 (disconnected)
        let graph = vec![vec![(1, 1)], vec![(0, 1)], vec![]];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, usize::MAX]);
    }

    #[test]
    fn test_shortest_of_multiple_paths() {
        // 0 --10-- 2
        // |        |
        // 1        1
        // |        |
        // 1 --1--- 2
        // Path 0->2 direct = 10, path 0->1->2 = 2
        let graph = vec![
            vec![(1, 1), (2, 10)],
            vec![(0, 1), (2, 1)],
            vec![(0, 10), (1, 1)],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, 2]);
    }

    #[test]
    fn test_from_different_start() {
        let graph = vec![vec![(1, 5)], vec![(0, 5), (2, 3)], vec![(1, 3)]];
        assert_eq!(dijkstra(&graph, 2), vec![8, 3, 0]);
    }

    #[test]
    fn test_graph_with_cycle() {
        // 0 -> 1 -> 2 -> 0 (cycle)
        let graph = vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, 2]);
    }
}
