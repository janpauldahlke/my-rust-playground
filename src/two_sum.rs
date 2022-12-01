/* Write a function that takes an array of numbers
(integers for the tests) and a target number.
It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple / list
(depending on your language) like so: (index1, index2).
For the purposes of this kata, some tests may have multiple answers; any valid solutions will be accepted.
The input will always be valid (numbers will be an array of length 2 or greater, and all of the items will be numbers; target will always be the sum of two different items from that array).
Based on: http://oj.leetcode.com/problems/two-sum/

```rust
two_sum(&[1, 2, 3], 4) // return (0, 2) or (2, 0)
```
*/

/* fn main() {
    let res = two_sum(&[1, 2, 3], 4);
    println!("{:?}", res);
} */

/* pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let inner_num_iter = numbers.iter();
    let num_iter = numbers.iter();
    let mut outer_index = 0;
    let mut result: (usize, usize) = (0, 0);

    'outer: for outer_num in num_iter {
        let mut inner_index = 0;

        for inner_num in inner_num_iter.clone() {
            if outer_num + inner_num == target {
                result = (outer_index, inner_index);
                break 'outer;
            }
            inner_index += 1;
        }
        outer_index += 1;
    }
    result
} */

/*

best practice solution would be
fn two_sum(n: &[i32], t: i32) -> (usize, usize) {
    for i in 0..n.len(){
        for j in 0..n.len(){
            if n[i] + n[j] == t && i != j{
                return (i,j)
            }
        }
    }
    return (0,0)
}
*/

// credit 2  TheRoboMan at discord
pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for (outer_index, outer_num) in numbers.iter().enumerate() {
        for (inner_index, inner_num) in numbers.iter().enumerate() {
            if outer_num + inner_num == target {
                return (outer_index, inner_index);
            }
        }
    }
    return (0, 0);
}
