/*
Crude implementation of a Min Heap (Priority Queue) (u32),
specifically engineered for the MST algorithm analysis.
Not all standard priority queue methods are implemented/needed.
Supplemented with a HashMap for fast lookup needed for MST.
Author: Khalid Akash, 2018
*/

use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub struct MinHeap {
    tree: Vec<(u32, u32)>,             /* node, edge */
    lookup_table: HashMap<u32, usize>, /* node, index in heap*/
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap {
            tree: Vec::new(),
            lookup_table: HashMap::new(),
        }
    }
    pub fn with_capacity(size: usize) -> MinHeap {
        MinHeap {
            tree: Vec::with_capacity(size),
            lookup_table: HashMap::with_capacity(size),
        }
    }
    pub fn print(&self) {
        let mut level: u32 = 0;
        let mut current_lvl_index = 0;
        for (i, elem) in self.tree.iter().enumerate() {
            eprint!("[{} ({})] ", elem.0, elem.1);
            current_lvl_index += 1;
            if i == 0 || u32::pow(2, level) == current_lvl_index {
                current_lvl_index = 0;
                level += 1;
                eprintln!("-- {}", level);
            }
        }
        level += 1;
        eprintln!("-- {}", level);
    }
    #[inline]
    fn get_parent_idx(index: usize) -> usize {
        let new_index = if index == 0 { 0 } else { (index - 1) / 2 };
        new_index
    }
    #[inline]
    fn get_left_child_idx(max_idx: usize, index: usize) -> usize {
        let mut new_index = 2 * index + 1;
        if new_index > max_idx {
            new_index = index;
        }
        new_index
    }
    #[inline]
    fn get_right_child_idx(max_idx: usize, index: usize) -> usize {
        let mut new_index = 2 * index + 2;
        if new_index > max_idx {
            new_index = index;
        }
        new_index
    }
    #[allow(dead_code)]
    fn is_heap_valid(&self, index: usize) -> bool {
        let max_idx = self.tree.len() - 1;
        let left_index = MinHeap::get_left_child_idx(max_idx, index);
        let right_index = MinHeap::get_right_child_idx(max_idx, index);
        if left_index != index {
            if self.tree[index].1 > self.tree[left_index].1 {
                return false;
            }
            if !self.is_heap_valid(left_index) {
                return false;
            }
        }
        if right_index != index {
            if self.tree[index].1 > self.tree[right_index].1 {
                return false;
            }
            if !self.is_heap_valid(right_index) {
                return false;
            }
        }
        true
    }
    /*Recursive heapify*/
    fn heapify(&mut self, mut index: usize) {
        let max: usize = self.tree.len() - 1;
        /*Sift Up*/
        if self.tree[MinHeap::get_parent_idx(index)].1 > self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_parent_idx(index)].0)
            {
                *val0 = index;
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_parent_idx(index);
            }
            self.tree.swap(index, MinHeap::get_parent_idx(index));
            index = MinHeap::get_parent_idx(index);
            self.heapify(index);
            self.heapify(MinHeap::get_parent_idx(index));
        }
        /* Sift Down.left */
        else if self.tree[MinHeap::get_left_child_idx(max, index)].1 < self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_left_child_idx(max, index)].0)
            {
                *val0 = index;
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_left_child_idx(max, index);
            }
            self.tree
                .swap(index, MinHeap::get_left_child_idx(max, index));
            index = MinHeap::get_left_child_idx(max, index);
            self.heapify(index);
            self.heapify(MinHeap::get_left_child_idx(max, index));
        }
        /* Sift Down.right */
        else if self.tree[MinHeap::get_right_child_idx(max, index)].1 < self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_right_child_idx(max, index)].0)
            {
                *val0 = index;
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_right_child_idx(max, index);
            }
            self.tree
                .swap(index, MinHeap::get_right_child_idx(max, index));
            index = MinHeap::get_right_child_idx(max, index);
            self.heapify(index);
            self.heapify(MinHeap::get_right_child_idx(max, index));
        }
    }
    pub fn decrease_key(&mut self, node: u32, new_edge: u32) -> bool {
        let mut index: usize = 0;
        if let Some(val) = self.lookup_table.get(&node) {
            index = *val as usize;
        } else {
            return false;
        }
        if index + 1 > self.tree.len() {
            return false;
        }
        self.tree[index].1 = new_edge;
        self.heapify(index);
        true
    }
    pub fn push(&mut self, new_val: (u32, u32)) {
        if self.lookup_table.contains_key(&new_val.0) {
            return;
        }
        self.tree.push(new_val);
        let new_node_index: usize = self.tree.len() - 1;
        self.lookup_table.insert(new_val.0, new_node_index);
        self.heapify(new_node_index);
    }
    pub fn pop(&mut self) -> Option<u32> {
        if self.tree.is_empty() {
            return None;
        }
        let ret = self.tree.swap_remove(0);
        self.lookup_table.remove_entry(&ret.0);
        if !self.tree.is_empty() {
            match self.lookup_table.get_mut(&self.tree[0].0) {
                Some(val) => *val = 0,
                None => return None,
            };
            self.heapify(0);
        }
        Option::Some(ret.0)
    }
    fn peek_node(&self, node: u32) -> Option<u32> {
        match self.lookup_table.get(&node) {
            Some(val) => Option::Some(*val as u32),
            None => None,
        }
    }
    pub fn get_current_weight(&self, node: u32) -> u32 {
        let index = self.peek_node(node).unwrap();
        self.tree[index as usize].1
    }
    pub fn contains(&self, node: u32) -> bool {
        self.lookup_table.contains_key(&node)
    }
    pub fn is_empty(&self) -> bool {
        if self.tree.len() == 0 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_vec_test() {
        eprintln!("Push Vector Test");
        {
            let mut test_heap = MinHeap::new();
            test_heap.push((1, 10));
            test_heap.push((2, 9));
            test_heap.push((3, 8));
            test_heap.push((9, 2));
            test_heap.push((5, 1));
            test_heap.push((15, 3));
            test_heap.push((13, 1000));
            test_heap.push((12, 1200));
            assert_eq!(test_heap.tree[0].0, 5 as u32);
        }
        {
            let mut test_heap = MinHeap::new();
            test_heap.push((15, 213123));
            test_heap.push((3, 1000));
            test_heap.push((69, 1));
            test_heap.push((13, 2322));
            test_heap.push((12, 2321));
            test_heap.push((13, 1));
            assert_eq!(test_heap.tree[0].0, 69);
        }
    }
    #[test]
    fn push_map_test() {
        eprintln!("Push Map Test");
        let mut test_heap = MinHeap::new();
        test_heap.push((3, 100));
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.lookup_table.get(&3), Some(&0));
        test_heap.push((1, 10));
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.lookup_table.get(&1), Some(&0));
        test_heap.push((5, 5));
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.lookup_table.get(&5), Some(&0));
        assert_eq!(test_heap.lookup_table.get(&11), None);
    }
    #[test]
    fn heap_modification_test() {
        eprintln!("Heap modification test");
        let mut test_heap = MinHeap::new();
        test_heap.push((1, 10));
        test_heap.push((2, 20));
        assert_eq!(test_heap.tree[0].0, 1);
        assert_eq!(test_heap.tree[1].0, 2);
        test_heap.decrease_key(1, 21);
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.tree[0].0, 2);
        assert_eq!(test_heap.tree[1].0, 1);
        test_heap.push((3, 25));
        test_heap.push((4, 24));
        test_heap.push((5, 5000));
        test_heap.decrease_key(4, 5);
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.tree[0].0, 4);
    }
    #[test]
    fn heap_pop_test() {
        let mut test_heap = MinHeap::new();
        test_heap.push((1, 50));
        test_heap.push((2, 25));
        test_heap.push((3, 22));
        test_heap.push((4, 23));
        assert_eq!(test_heap.tree[0].0, 3);
        assert_eq!(test_heap.pop().unwrap(), 3);
        assert!(test_heap.is_heap_valid(0));
        assert!(!test_heap.contains(3));
        assert_eq!(test_heap.tree[0].0, 4);
        test_heap.push((3, 22));
        assert_eq!(test_heap.tree[0].0, 3);
        assert_eq!(test_heap.pop().unwrap(), 3);
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.pop().unwrap(), 4);
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.pop().unwrap(), 2);
        assert!(test_heap.is_heap_valid(0));
        assert_eq!(test_heap.tree[0].0, 1);
    }
}
