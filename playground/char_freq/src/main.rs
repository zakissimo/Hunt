use std::collections::HashMap;

fn main() {
    let input =
        "Large cows generosity comes with charts and four blonde hats to defy upper gravity hero";
    let mut freq = HashMap::<char, usize>::new();

    for c in input.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut sorted_vec: Vec<_> = freq.into_iter().collect();

    // Sort the vector by values in ascending order
    sorted_vec.sort_by_key(|&(_, value)| std::cmp::Reverse(value));

    println!("{:?}", sorted_vec);
}
