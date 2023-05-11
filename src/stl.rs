use std::collections::VecDeque;

pub mod collections {
    use super::*;
    pub struct OrderedQueue<T> {
        queue: VecDeque<T>,
    }

    impl<T> OrderedQueue<T>
    where
        T: Ord + PartialOrd + Copy,
    {
        pub fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }

        pub fn push(&mut self, item: T) {
            let idx = self.queue.partition_point(|&x| x < item);
            self.queue.insert(idx, item);
        }

        pub fn front(&self) -> Option<&T> {
            self.queue.front()
        }

        pub fn back(&self) -> Option<&T> {
            self.queue.back()
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.queue.pop_front()
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.queue.pop_back()
        }

        pub fn len(&self) -> usize {
            self.queue.len()
        }

        pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
            self.queue.iter()
        }

        pub fn remove(&mut self, item: &T) -> Option<T> {
            match self.queue.binary_search(item) {
                Ok(idx) => self.queue.remove(idx),
                Err(_) => None,
            }
        }
    }
}

pub mod transform{
    pub fn tuple_to_vec(){
        
    }
}