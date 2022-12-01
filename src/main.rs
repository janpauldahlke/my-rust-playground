#![allow(unused)] // silence warnings while dev // comment out later

//mod find_even_index;
//use find_even_index::find_even_index;

/* fn sample() {
    do_test(&[1, 2, 3], 4);
    do_test(&[1234, 5678, 9012], 14690);
    do_test(&[2, 2, 3], 4);
} */

mod two_sum;
use two_sum::two_sum;
fn main() {
    let res = two_sum(&[1, 2, 3], 4);
    println!("{:?}", res);
}
