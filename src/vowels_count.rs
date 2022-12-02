/* Return the number (count) of vowels in the given string.
We will consider a, e, i, o, u as vowels for this Kata (but not y).
The input string will only consist of lower case letters and/or spaces. */

pub fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    const MATCH: &str = "aeiouAEIOU";

    for c in string.chars() {
        if MATCH.contains(c) {
            vowels_count += 1;
        }
    }

    vowels_count
}
