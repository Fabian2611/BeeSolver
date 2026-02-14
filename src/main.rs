use crate::dictionary::Dictionary;

mod dictionary;

fn main() {
    let words = Dictionary::new(
        &std::env::args()
            .nth(1)
            .expect("Usage: beesolver <dictionary> <center> <letters>")[..],
    )
    .get_words()
    .clone();

    let center = std::env::args()
        .nth(2)
        .expect("Usage: beesolver <dictionary> <center> <letters>")[..]
        .chars()
        .next()
        .expect("Center must be a single character");
    let letters = std::env::args()
        .nth(3)
        .expect("Usage: beesolver <dictionary> <center> <letters>")[..]
        .chars()
        .collect::<Vec<char>>();

    let combined = [letters.as_slice(), &[center]].concat();

    words
        .iter()
        .filter(|word| {
            word.len() >= 4 && word.contains(center) && word.chars().all(|c| combined.contains(&c))
        })
        .for_each(|word| println!("{}", word));
}
