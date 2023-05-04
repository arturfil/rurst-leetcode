use std::{collections::{BinaryHeap}};

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k);

    for p in &points {
        let d = p[0] * p[0] + p[1] * p[1];
        heap.push((d, vec![p[0], p[1]]));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.into_iter().map(|(_, p)| p).collect()
}

