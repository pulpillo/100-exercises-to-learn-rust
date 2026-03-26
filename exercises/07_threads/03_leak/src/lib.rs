// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let slice = Vec::leak(v);
    let mid = slice.len() / 2;
    let (left,right) = slice.split_at(mid);
    
    let first_handle = thread::spawn(|| { left.iter().sum::<i32>()});
    let second_handle = thread::spawn(|| { right.iter().sum::<i32>()});
    
    let first_sum : i32 = first_handle.join().unwrap();
    let second_sum : i32  = second_handle.join().unwrap();
    
    first_sum + second_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
