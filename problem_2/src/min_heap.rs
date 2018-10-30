/*
Crude implementation of a dynamic Min Heap (u32)
Not all standard heap methods are implemented/needed
Allows modification of any value, and updates heap accordingly
Author: Khalid Akash
*/

use std::collections::HashMap;
use std::vec::Vec;

pub struct MinHeap {
    tree: Vec<(u32, u32)>,             /* node, edge */
    lookup_table: HashMap<u32, usize>, /* node, index in heap*/
}

impl MinHeap {
    pub fn print(&self) {
        let mut level: u32 = 0;
        let mut current_lvl_index = 0;
        for (i, elem) in self.tree.iter().enumerate() {
            eprint!("[{} ({})] ", elem.0, elem.1);
            current_lvl_index = current_lvl_index + 1;
            if i == 0 || u32::pow(2, level) == current_lvl_index {
                current_lvl_index = 0;
                level = level + 1;
                eprintln!("-- {}", level);
            }
        }
        level = level + 1;
        eprintln!("-- {}", level);
    }
    fn get_parent_idx(index: usize) -> usize {
        let mut new_index;
        if index == 0 {
            new_index = 0;
        } else {
            new_index = (index - 1) / 2;
        }
        new_index
    }
    fn get_left_child_idx(max : usize, index: usize) -> usize {
        let mut new_index = 2 * index + 1;
        if new_index > max {
            new_index = index;
        }
        new_index
    }
    fn get_right_child_idx(max : usize, index: usize) -> usize {
        let mut new_index = 2 * index + 2;
        if new_index > max {
            new_index = index;
        }
        new_index
    }
    pub fn new() -> MinHeap {
        MinHeap {
            tree: Vec::new(),
            lookup_table: HashMap::new(),
        }
    }
    fn heapify(&mut self, mut index: usize) {
        let max : usize = self.tree.len() - 1;
        /*Heapify*/
        /*Swap value with parent and child and update lookup table*/
        while &self.tree[MinHeap::get_parent_idx(index)].1 > &self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_parent_idx(index)].0)
            {
                *val0 = index.clone();
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_parent_idx(index).clone();
            }
            self.tree.swap(index, MinHeap::get_parent_idx(index));
            index = MinHeap::get_parent_idx(index);
        }
        /* Compare left child with parent */
        while &self.tree[MinHeap::get_left_child_idx(max, index)].1 < &self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_left_child_idx(max,index)].0)
            {
                *val0 = index.clone();
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_left_child_idx(max, index).clone();
            }
            self.tree.swap(index, MinHeap::get_left_child_idx(max, index));
            index = MinHeap::get_left_child_idx(max, index);
        }
            /* Compare right child with parent */
        while &self.tree[MinHeap::get_right_child_idx(max, index)].1 < &self.tree[index].1 {
            if let Some(val0) = self
                .lookup_table
                .get_mut(&self.tree[MinHeap::get_right_child_idx(max,index)].0)
            {
                *val0 = index.clone();
            }
            if let Some(val1) = self.lookup_table.get_mut(&self.tree[index].0) {
                *val1 = MinHeap::get_right_child_idx(max, index).clone();
            }
            self.tree.swap(index, MinHeap::get_right_child_idx(max, index));
            index = MinHeap::get_right_child_idx(max, index);
        }       
    }
    pub fn modify(&mut self, node: u32, new_edge: u32) {
        let mut index : usize = 0;
        if let Some(val) = self.lookup_table.get(&node) {
            index = (*val).clone() as usize;
        } else {
            return;
        }
        self.tree[index].1 = new_edge;
        self.heapify(index);
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
    pub fn peek(&self) -> &(u32, u32) {
        &self.tree[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_vec_test() {
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
            test_heap.print();
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
            test_heap.print();
            assert_eq!(test_heap.tree[0].0, 69);
        }
    }
    #[test]
    fn push_map_test() {
        let mut test_heap = MinHeap::new();
        test_heap.push((3, 100));
        assert_eq!(test_heap.lookup_table.get(&3), Some(&0));
        test_heap.push((1, 10));
        assert_eq!(test_heap.lookup_table.get(&1), Some(&0));
        test_heap.push((5, 5));
        assert_eq!(test_heap.lookup_table.get(&5), Some(&0));
        assert_eq!(test_heap.lookup_table.get(&11), None);
    }
    #[test]
    fn heap_modification_test(){
        let mut test_heap = MinHeap::new();
        test_heap.push((1, 10));
        test_heap.push((2, 20));
        assert_eq!(test_heap.tree[0].0, 1);
        assert_eq!(test_heap.tree[1].0, 2);
        test_heap.modify(1, 21);
        assert_eq!(test_heap.tree[0].0, 2);
        assert_eq!(test_heap.tree[1].0, 1);
        test_heap.push((3, 25));
        test_heap.push((4, 24));
        test_heap.push((5, 5000));
        test_heap.print();
        test_heap.modify(4, 5);
        test_heap.print();
        assert_eq!(test_heap.tree[0].0, 4);
    }
}
