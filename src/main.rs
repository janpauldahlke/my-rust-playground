#![allow(unused)] // silence warnings while dev // comment out later

mod high_low;
use high_low::high_and_low;

fn main() {
    let res = high_and_low("1 2 -3 4 5");
    println!("{}", res);
}

/* mod vowels_count;
use vowels_count::get_count;

fn main() {
    let res = get_count("abracadabra");
    println!("{}", res)
} */

//mod find_even_index;
//use find_even_index::find_even_index;

/* mod two_sum;
use two_sum::two_sum; */

/* fn sample() {
    do_test(&[1, 2, 3], 4);
    do_test(&[1234, 5678, 9012], 14690);
    do_test(&[2, 2, 3], 4);
} */
/* fn main() {
    let res = two_sum(&[1, 2, 3], 4);
    println!("{:?}", res);
} */

/* mod cc_mask;
use cc_mask::maskify;

fn main() {
    let res = maskify("84");
    println!("{}", res)
}
 */

/* mod who_likes_it;
use who_likes_it::likes;

fn main() {
    let res = likes(&["Alex", "Jacob", "Mark", "Max"]);
    println!("{}", res)
}
 */
