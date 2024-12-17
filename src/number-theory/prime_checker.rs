use std::io;

// Time Complexity: O(log(n))
// Space Complexity: O(1)
fn is_prime(num: i32) -> bool {
    let upper_bound = f64::sqrt(num as f64) as i32 + 1;
    if num == 1 {
        return false;
    }
    for i in 2..upper_bound {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
