use std::collections::VecDeque;

struct Graph<T> {
    item: Vec<T>,
    graph: Vec<Vec<usize>>,
}

impl<T> Graph<T> {
    fn new(size: usize) -> Graph<T> {
        let item: Vec<T> = Vec::new();
        let mut graph = Vec::new();
        for i in 0..size {
            graph.push(Vec::new());
        }
        Graph::<T> {
            item: item,
            graph: graph,
        }
    }
    fn push_item(&mut self, item: T) {
        self.item.push(item);
    }
    fn add_edge(&mut self, node: usize, edge: usize) {
        self.graph[node].push(edge);
    }
    fn dfs(&self, start: usize, end: usize) -> Vec<&T>{
        let mut ans = Vec::new();
        let mut waiting = VecDeque::new();
        let mut done = Vec::new();
        for _i in 0..self.item.len() {
            done.push(0);
        }
        done[start] = 2;
        ans.push(&self.item[start]);
        for i in &self.graph[start] {
            if done[*i] == 0 {
                done[*i] = 1;
                waiting.push_back(*i);
            }
        }
        while waiting.len() > 0 {
            let cur = waiting.pop_back().unwrap();
            if done[cur] != 2 {
                done[cur] = 2;
                ans.push(&self.item[cur]);
                if cur == end {
                    break;
                }
                for i in &self.graph[cur] {
                    if done[*i] == 0 {
                        done[*i] = 1;
                        waiting.push_back(*i);
                    }
                }
            }
        }
        ans
    }

    fn bfs(&self, start: usize, end: usize) -> Vec<&T> {
        let mut ans = Vec::new();
        let mut waiting = VecDeque::new();
        let mut done = Vec::new();
        for _i in 0..self.item.len() {
            done.push(0);
        }
        done[start] = 2;
        ans.push(&self.item[start]);
        for i in &self.graph[start] {
            if done[*i] == 0 {
                done[*i] = 1;
                waiting.push_back(*i);
            }
        }
        while waiting.len() > 0 {
            let cur = waiting.pop_front().unwrap();
            if done[cur] != 2 {
                done[cur] = 2;
                ans.push(&self.item[cur]);
                if cur == end {
                    break;
                }
                for i in &self.graph[cur] {
                    if done[*i] == 0 {
                        done[*i] = 1;
                        waiting.push_back(*i);
                    }
                }
            }
        }
        ans
    }
}

#[test]
fn test () {
    let mut graph = Graph::new(9);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(3, 7);
    graph.add_edge(2, 5);
    graph.add_edge(2, 6);
    graph.add_edge(6, 8);
    for i in 0..9 {
        graph.push_item(i + 1);
    }
    let ans1 = graph.dfs(0, 8);
    let ans2 = graph.bfs(0, 8);
    assert_eq!(ans1, vec![&1, &3, &7, &9]);
    assert_eq!(ans2, vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
}