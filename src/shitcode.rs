fn main() {
    let result = disemvowel("the emprer protects");
    println!("{result}");
}

fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let char_string: Vec<char> = s.chars().collect();
    let removed = char_string.into_iter().map(|letter| {
        let mut inner: Vec<char> = Vec::new();
        for vow in vowels {
            if vow == letter {
                continue;
            } else {
                inner.push(letter)
            }
        }
        inner
    });

    let s: String = removed.flatten().into_iter().collect::<String>();
    s
}

// output is
//ttttthhhhheeee     eeeemmmmmppppprrrrreeeerrrrr     ppppprrrrroooottttteeeeccccctttttsssss
