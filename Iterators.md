```
pub fn sum_ints(v: Vec<i128>) -> i128 { 
    let sum:i128 = v.iter().sum();
    sum
}


pub fn sum_ints_addr(v: &[i128]) -> i128 {
    let sum:i128 = v.iter().sum();
    sum
}
```
