/* In this little assignment you are given a string of space separated numbers,
and have to return the highest and lowest number. */

/*
high_and_low("1 2 3 4 5")  // return "5 1"
high_and_low("1 2 -3 4 5") // return "5 -3"
high_and_low("1 9 3 4 -5") // return "9 -5"
 */

pub fn high_and_low(numbers: &str) -> String {
    let mut nums = numbers
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    format!("{} {}", max, min)
}

/*
ideal solution maybe:
fn high_and_low(numbers: &str) -> String {
  let vector_string: Vec<&str> = numbers.split(" ").collect();
    let max: i32 = vector_string.iter().map(|x| x.parse::<i32>().unwrap()).max().unwrap();
    let min: i32 = vector_string.iter().map(|x| x.parse::<i32>().unwrap()).min().unwrap();
    format!("{:?} {:?}", max,min)
}
*/
