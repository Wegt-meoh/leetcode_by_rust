use std::collections::BTreeSet;

pub struct MKAverage {
    m:i32,
    k:i32,
    data:BTreeSet<i32>    
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {        
        MKAverage{
            m,
            k,
        }
    }
    
    fn add_element(&self, num: i32) {
        
    }
    
    fn calculate_mk_average(&self) -> i32 {

    }
}