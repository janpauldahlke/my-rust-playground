#![allow(unused)] // silence warnings while dev // comment out later
fn main() {
    let res = to_camel_case("the_emporer_protects");
    let res1 = to_camel_case("Heresy-is-a-crime-of-thoughts");
    let res2 = to_camel_case("RWUIOBIQsss");
    println!("{:?}", res);
    println!("{:?}", res1);
    println!("{:?}", res2);
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

fn to_camel_case(text: &str) -> String {
    //let mut inner_text = String::from(text);
    let delimiters: Vec<char> = Vec::from(['_', '-']);
    let delim_iter = delimiters.iter();
    let mut res_text = String::new();

    for &entry in delim_iter {
        if text.contains(entry) {
            let res: Vec<&str> = text.split(entry).collect();
            let mut counter = 0;
            for r in res {
                counter += 1;
                if counter < 2 {
                    res_text = r.to_string();
                } else {
                    let foo = uppercase_first(r);
                    res_text = res_text + &foo;
                }
            }
            break; // exit after we found one delimiter
        } else {
            res_text = String::from(text);
        }
    }

    res_text
}

//helper to uper
fn uppercase_first(data: &str) -> String {
    // Uppercase first letter.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
        }
    }
    result
}
