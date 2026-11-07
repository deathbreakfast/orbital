//! Cycle breaking and longest-path depths.

use std::collections::VecDeque;

use super::Edge;

/// Marks the edges that close a cycle. Walks depth first from each node in
/// input order, following edges in input order; an edge into a node still on
/// the walk's stack is a back edge. Iterative, so a long chain can't overflow
/// the call stack.
pub(crate) fn back_edges(node_count: usize, edges: &[Edge]) -> Vec<bool> {
    let mut out_edges: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (i, e) in edges.iter().enumerate() {
        out_edges[e.from].push(i);
    }
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        New,
        OnStack,
        Done,
    }
    let mut state = vec![State::New; node_count];
    let mut back = vec![false; edges.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for root in 0..node_count {
        if state[root] != State::New {
            continue;
        }
        state[root] = State::OnStack;
        stack.push((root, 0));
        while let Some(top) = stack.last_mut() {
            let (node, next) = *top;
            if let Some(&edge) = out_edges[node].get(next) {
                top.1 += 1;
                let to = edges[edge].to;
                match state[to] {
                    State::New => {
                        state[to] = State::OnStack;
                        stack.push((to, 0));
                    }
                    State::OnStack => back[edge] = true,
                    State::Done => {}
                }
            } else {
                state[node] = State::Done;
                stack.pop();
            }
        }
    }
    back
}

/// Longest path from a source to each node over acyclic `edges`: 0 for a
/// node with no predecessors, else one more than its deepest predecessor.
pub(crate) fn depths(node_count: usize, edges: &[Edge]) -> Vec<u32> {
    let mut succ: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    let mut indegree = vec![0_usize; node_count];
    for e in edges {
        succ[e.from].push(e.to);
        indegree[e.to] += 1;
    }
    let mut depth = vec![0_u32; node_count];
    let mut queue: VecDeque<usize> = (0..node_count).filter(|&n| indegree[n] == 0).collect();
    while let Some(node) = queue.pop_front() {
        for &to in &succ[node] {
            depth[to] = depth[to].max(depth[node] + 1);
            indegree[to] -= 1;
            if indegree[to] == 0 {
                queue.push_back(to);
            }
        }
    }
    depth
}
