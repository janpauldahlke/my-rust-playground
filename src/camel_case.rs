/* Complete the method/function so that it converts dash/underscore delimited words into camel casing.
The first word within the output should be capitalized only if the original word was capitalized
(known as Upper Camel Case, also often referred to as Pascal case). The next words should be always capitalized.

Examples
"the-stealth-warrior" gets converted to "theStealthWarrior"
"The_Stealth_Warrior" gets converted to "TheStealthWarrior" */

fn main() {
    let res = to_camel_case("the_emporer_protects");
    let res1 = to_camel_case("Heresy-is-a-crime-of-thoughts");
    let res2 = to_camel_case("RWUIOBIQsss");
    println!("{:?}", res);
    println!("{:?}", res1);
    println!("{:?}", res2);
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
                break;
            }
        } else {
            return String::from(text);
        }
    }

    res_text
}

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

/*
//shortest solution for comparision
fn to_camel_case(text: &str) -> String {
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => w.to_string(),
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect()
}
*/
