#![allow(dead_code)]
#![allow(unused_imports)]

use std::process::id;

pub trait Heap {
    fn push(&mut self, n: i32);
    fn pop(&mut self) -> Option<i32>;
}

#[derive(Debug)]
pub struct MaxHeap {
    heap: Vec<i32>,
    next: usize,
}

impl MaxHeap {
    pub fn new() -> MaxHeap {
        MaxHeap {
            heap: vec![],
            next: 0,
        }
    }

    fn heapify_up(&mut self, index: usize) {
        let mut idx = index;
        loop {
            if idx == 0 {
                return;
            }
            let parent = (idx - 1) / 2;
            if parent == idx {
                return;
            }
            if self.heap[parent] < self.heap[idx] {
                (self.heap[parent], self.heap[idx]) = (self.heap[idx], self.heap[parent]);
                // check the other child
                let other_child = if idx % 2 == 0 { idx - 1 } else { idx + 1 };
                if other_child < self.next && self.heap[other_child] > self.heap[parent] {
                    self.heapify_down(parent);
                }
                idx = parent;
                continue;
            }
            return;
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let mut idx = index;
        loop {
            let mut larger_child = 2 * idx + 1; // assume left child is larger
            if larger_child >= self.next {
                // there can be no right child, we're done here
                return;
            }
            if self.heap[larger_child] < self.heap[larger_child + 1] {
                larger_child += 1; // the right child is larger
            }
            // if the child is larger than the parent then swap them and
            // continue heapifying down, otherwise we're done
            if self.heap[larger_child] > self.heap[idx] {
                (self.heap[larger_child], self.heap[idx]) =
                    (self.heap[idx], self.heap[larger_child]);
                idx = larger_child;
                continue;
            }
            return;
        }
    }
}

impl Heap for MaxHeap {
    fn push(&mut self, n: i32) {
        if self.heap.len() <= self.next {
            self.heap.push(n);
        } else {
            self.heap[self.next] = n;
        }
        self.next += 1;
        self.heapify_up(self.next - 1);
    }
    fn pop(&mut self) -> Option<i32> {
        let n = *self.heap.first()?;
        self.heap[0] = self.heap[self.next - 1];
        // If we want to free memory we can uncomment the following and change
        // push() to always do self.heap.push().
        // self.heap.resize(self.next, 0);
        self.next -= 1;
        self.heapify_down(0);
        Some(n)
    }
}
