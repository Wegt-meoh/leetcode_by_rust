use std::collections::VecDeque;

use crate::stl::collections::OrderedQueue;

pub struct MKAverage {
    m: usize,
    k: usize,
    sum: i64,
    queue: VecDeque<i32>,
    small_queue: OrderedQueue<i32>,
    mid_queue: OrderedQueue<i32>,
    large_queue: OrderedQueue<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    pub fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m: m as usize,
            k: k as usize,
            sum: 0,
            queue: VecDeque::new(),
            small_queue: OrderedQueue::new(),
            mid_queue: OrderedQueue::new(),
            large_queue: OrderedQueue::new(),
        }
    }

    pub fn add_element(&mut self, num: i32) {
        self.queue.push_back(num);

        if self.queue.len() == self.m {
            for i in 0..self.m {
                self.mid_queue.push(*self.queue.get(i).unwrap());
            }

            for _ in 0..self.k {
                self.small_queue.push(self.mid_queue.pop_front().unwrap());
                self.large_queue.push(self.mid_queue.pop_back().unwrap());
            }

            for i in self.mid_queue.iter() {
                self.sum += *i as i64;
            }
        } else if self.queue.len() > self.m {
            let old = self.queue.pop_front().unwrap();
            if old == num {
                return;
            }

            if num<*self.small_queue.back().unwrap(){
                self.small_queue.push(num);
                self.sum+=*self.small_queue.back().unwrap() as i64;
                self.mid_queue.push(self.small_queue.pop_back().unwrap());                
            }else if num>*self.large_queue.front().unwrap(){
                self.large_queue.push(num);
                self.sum+=*self.large_queue.front().unwrap() as i64;
                self.mid_queue.push(self.large_queue.pop_front().unwrap());
            }else{
                self.sum+=num as i64;
                self.mid_queue.push(num);
            }

            if self.small_queue.remove(&old)!=None{
                self.sum-=*self.mid_queue.front().unwrap() as i64;
                self.small_queue.push(self.mid_queue.pop_front().unwrap());
            }else if self.large_queue.remove(&old)!=None{
                self.sum-=*self.mid_queue.back().unwrap() as i64;
                self.large_queue.push(self.mid_queue.pop_back().unwrap());
            }else{
                self.sum-=old as i64;
                self.mid_queue.remove(&old);
            }           
        }
    }

    pub fn calculate_mk_average(&mut self) -> i32 {
        if self.queue.len() < self.m {
            return -1;
        }

        (self.sum / self.mid_queue.len() as i64) as i32
    }
}

pub mod method2 {
    use std::collections::{BTreeMap, VecDeque};

    struct OrderedSet<T> {
        tree: BTreeMap<T, i32>,
    }

    impl<T> OrderedSet<T>
    where
        T: PartialOrd + Ord + Copy,
    {
        fn new() -> Self {
            Self {
                tree: BTreeMap::new(),
            }
        }

        fn insert(&mut self, x: T) {
            *self.tree.entry(x).or_insert(0) += 1;
        }

        fn remove(&mut self, x: &T) -> bool {
            let need_remove = {
                if let Some(t) = self.tree.get_mut(x) {
                    *t -= 1;
                    *t == 0
                } else {
                    return false;
                }
            };
            if need_remove {
                self.tree.remove(x);
            }
            true
        }

        fn peek_first(&mut self) -> &T {
            self.tree.iter().next().unwrap().0
        }

        fn peek_last(&mut self) -> &T {
            self.tree.iter().rev().next().unwrap().0
        }

        fn pop_first(&mut self) -> T {
            let t = *self.peek_first();
            self.remove(&t);
            t
        }

        fn pop_last(&mut self) -> T {
            let t = *self.peek_last();
            self.remove(&t);
            t
        }
    }

    struct MKAverage {
        m: i32,
        k: i32,
        fifo: VecDeque<i32>,
        s_min: OrderedSet<i32>,
        s_mid: OrderedSet<i32>,
        s_max: OrderedSet<i32>,
        sum: i64,
    }

    impl MKAverage {
        fn new(m: i32, k: i32) -> Self {
            Self {
                m,
                k,
                fifo: VecDeque::with_capacity(m as usize),
                s_min: OrderedSet::new(),
                s_mid: OrderedSet::new(),
                s_max: OrderedSet::new(),
                sum: 0,
            }
        }

        fn add_element(&mut self, num: i32) {
            if self.fifo.len() < self.m as usize {
                self.sum += num as i64;
                self.s_mid.insert(num);
                self.fifo.push_back(num);
                if self.fifo.len() == self.m as usize {
                    for _ in 0..self.k {
                        let t = self.s_mid.pop_first();
                        self.sum -= t as i64;
                        self.s_min.insert(t);
                    }
                    for _ in 0..self.k {
                        let t = self.s_mid.pop_last();
                        self.sum -= t as i64;
                        self.s_max.insert(t);
                    }
                }
                return;
            }
            let out = self.fifo.pop_front().unwrap();
            self.fifo.push_back(num);
            if num < *self.s_min.peek_last() {
                let t = self.s_min.pop_last();
                self.s_min.insert(num);
                self.s_mid.insert(t);
                self.sum += t as i64;
            } else if num > *self.s_max.peek_first() {
                let t = self.s_max.pop_first();
                self.s_max.insert(num);
                self.s_mid.insert(t);
                self.sum += t as i64;
            } else {
                self.s_mid.insert(num);
                self.sum += num as i64;
            }
            if self.s_mid.remove(&out) {
                self.sum -= out as i64;
            } else if self.s_min.remove(&out) {
                let t = self.s_mid.pop_first();
                self.sum -= t as i64;
                self.s_min.insert(t);
            } else {
                let t = self.s_mid.pop_last();
                self.sum -= t as i64;
                self.s_max.insert(t);
            }
        }

        fn calculate_mk_average(&self) -> i32 {
            if self.fifo.len() < self.m as usize {
                -1
            } else {
                (self.sum / (self.m - self.k * 2) as i64) as i32
            }
        }
    }
}
