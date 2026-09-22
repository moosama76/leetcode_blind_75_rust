use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }
    
    // 1 2 3 4 10       5 6 7 8   ----------> 1 2 3 4 5 6 7 8 10
    // 1 2 3 4 5       5 6 7 8 10 ----------> 1 2 3 4 5 6 7 8 10
    // 1 5             3 

    fn add_num(&mut self, num: i32) {
        if self.left.is_empty() || num <= *self.left.peek().unwrap()
        {
            self.left.push(num);
        } else {
            self.right.push(Reverse(num));
        }
        if self.left.len() > self.right.len() + 1 {
            let x = self.left.pop().unwrap();
            self.right.push(Reverse(x));
        } else if self.right.len() > self.left.len() {
            let x = self.right.pop().unwrap().0;
            self.left.push(x);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.right.is_empty() {
            return *self.left.peek().unwrap() as f64;
        }

        if self.left.len() > self.right.len() {
            return *self.left.peek().unwrap() as f64;
        }

        let left = *self.left.peek().unwrap();
        let right = self.right.peek().unwrap().0;

        (left + right) as f64 / 2.0
            
    }  
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
