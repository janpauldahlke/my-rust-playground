pub fn find_even_index(arr: &[i32]) -> Option<usize> {
    let reversed = Vec::from(arr).reverse();
    let orignal = Vec::from(arr);

    let mut index = 0;
    let outcome_index = 0;
    let mut sum_original = 0;
    let mut sum_reversed = 0;
    for entry in orignal {
        sum_original += sum_original + entry;
        index += 1;

        //sum_reversed = sum_reversed + reversed.[`index`]
    }

    //println!("{:?},", arr);
    //remove at the end
    Some(1)
}
