/* Usually when you buy something, you're asked whether your
credit card number, phone number or answer to your most secret question is still
correct. However, since someone could look over your shoulder, you don't want that shown
on your screen. Instead, we mask it.
Your task is to write a function maskify, which changes all but the last four characters into '#'.
*/

/*
"4556364607935616" --> "############5616"
"64607935616" -->      "#######5616"
          "1" -->                "1"
           "" -->                 ""
           */

/// Return a String with all characters masked as '#' except the last 4.
pub fn maskify(cc: &str) -> String {
    if cc.len() > 4 {
        let mut masked = String::new();
        for _ in 0..cc.len() - 4 {
            masked.push('#');
        }
        masked.push_str(&cc[cc.len() - 4..]);
        masked
    } else {
        cc.to_string()
    }
}

/*
best practice:
fn maskify(cc: &str) -> String {
    let mask_length = cc.len().saturating_sub(4);
    "#".repeat(mask_length) + &cc[mask_length..]
}
*/
