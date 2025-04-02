/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 将新元素添加到堆的末尾
        self.items.push(value);
        self.count += 1;

        // 执行上滤操作，确保堆的性质
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 比较当前节点与父节点，如果需要交换则进行调整
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 获取左子节点索引
        let left_idx = self.left_child_idx(idx);
        // 获取右子节点索引
        let right_idx = self.right_child_idx(idx);

        // 如果右子节点存在且比左子节点更符合比较条件，则返回右子节点索引
        if right_idx <= self.count && (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
            right_idx
        } else {
            left_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 添加 Clone 约束
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 如果堆为空，返回 None
        }

        // 保存堆顶元素（即第一个有效元素）
        let result = self.items.get(1).cloned();

        // 将堆的最后一个元素移动到堆顶
        if self.count > 0 {
            if let Some(last_item) = self.items.pop() { // 使用 if let 处理 Option 类型
                self.items[1] = last_item;
                self.count -= 1;

                // 执行下滤操作，确保堆的性质
                let mut idx = 1;
                while self.children_present(idx) {
                    let child_idx = self.smallest_child_idx(idx);
                    // 如果当前节点不符合比较条件，则与子节点交换
                    if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                        self.items.swap(idx, child_idx);
                        idx = child_idx;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}