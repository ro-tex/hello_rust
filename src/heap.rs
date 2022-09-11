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
        if self.next == 0 {
            return;
        }
        let mut idx = index;
        loop {
            let mut larger_child = 2 * idx + 1; // assume left child is larger
            if larger_child >= self.next - 1 {
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

/**
USE_MORE_MEM allows us to make a tradeoff between speed and memory usage.

When USE_MORE_MEM is set to false, we'll truncate the heap vector each time we
pop an element off the heap. This will keep memory usage down but will involve
more allocations in case we later need to regrow the vector to its previous size
but the adjacent memory is no longer available. In that case the entire vector
will need to be reallocated.

When USE_MORE_MEM is true we won't truncate it. This will reduce the number of
memory operations we do but our heap will no longer return memory to the OS.
*/
const USE_MORE_MEM: bool = false;

impl Heap for MaxHeap {
    fn push(&mut self, n: i32) {
        if USE_MORE_MEM {
            if self.heap.len() <= self.next {
                self.heap.push(n);
            } else {
                self.heap[self.next] = n;
            }
        } else {
            self.heap.push(n);
        }

        self.next += 1;
        self.heapify_up(self.next - 1);
    }
    fn pop(&mut self) -> Option<i32> {
        let n = *self.heap.first()?;
        self.heap[0] = self.heap[self.next - 1];
        // If we want to free memory we can uncomment the following and change
        // push() to always do self.heap.push(). This is slower but it uses
        // less memory.
        self.next -= 1;
        if !USE_MORE_MEM {
            self.heap.truncate(self.next);
        }
        // println!(">>> next is {}", self.next);
        self.heapify_down(0);
        Some(n)
    }
}
