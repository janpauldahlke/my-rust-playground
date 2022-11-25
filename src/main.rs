#![allow(unused)] // silence warnings while dev // comment out later
fn main() {
    //let result = disemvowel("the emporer protects! So must we. I also");
    // let result = reverse_seq(5);
    let result = is_valid_ip("12.255.56.1");
    println!("{:?}", result);
}

/* fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut local_string = String::from(s).to_lowercase();

    for vow in vowels {
        let local_vow = vow.to_string();
        local_string = local_string.replace(&local_vow, "");
    }
    local_string
} */

fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|&char| !"aAeEiIoOuU".contains(char))
        .collect()
}

fn min_max(lst: &[i32]) -> (i32, i32) {
    let mut min_max: (i32, i32) = (0, 0);

    min_max.0 = *lst.iter().min().unwrap();
    min_max.1 = *lst.iter().max().unwrap() as i32;
    min_max
}

fn reverse_seq(n: u32) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();

    //the rev is already here, but not in the end
    for num in (1..n + 1).rev() {
        res.push(num)
    }
    res

    // onliner is
    //(1..=n)
    //.rev()
    //.collect()
}

//https://math.stackexchange.com/a/3813030
//Set 4x2+80=y2. Then we have 4x2−y2=−80. Factor it as a difference of two squares and go through all the pairs of divisors of −80, whose product is also −80.

enum Value {
    Int(isize),
    Float(f64),
}
//perfect square
fn find_next_square(sq: u64) -> Option<u64> {
    //check if sqrt to string contains .
    //complicated hack?

    if f64::sqrt(sq as f64).to_string().contains(".") {
        None
    } else {
        let rounded = f64::sqrt(sq as f64).ceil() + 1.0;
        Some((rounded * rounded) as u64)
    }
}

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::from(numbers);
    if res.len() >= 1 {
        let min = res.iter().min().unwrap();
        let index = res.iter().position(|num| *num == *min).unwrap();
        res.remove(index);
    }
    res
}

/* fn main() {
    println!("{:?}", binary_slice_to_number(&[0, 0, 1, 0]))
} */

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let joined: String = slice.iter().map(|&id| id.to_string() + "").collect();
    let num_from_bin = u32::from_str_radix(&joined, 2).unwrap();
    num_from_bin
}

fn is_valid_ip(ip: &str) -> bool {
    let octets = ip.split(".").collect::<Vec<&str>>();
    if octets.len() == 0 || octets.len() != 4 {
        return false;
    }
    for octet in octets {
        let oct: Vec<char> = octet.chars().collect();
        let num = octet.parse::<u8>();
        match num {
            Ok(num) => {
                if oct.len() > 1 && oct[0] == '0' {
                    return false;
                }
            }
            Err(_num) => {
                return false;
            }
        }
    }
    true
}
