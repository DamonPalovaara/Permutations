use math::Permutation;
use std::{collections::BTreeMap, time::Instant};

// The n corresponding to C_n
const N: usize = 8;
// Set to true if you want to see the elements, false otherwise
// WARN: For large N this will overfill your console (use false for large N)
const DISPLAY: bool = true;

fn main() {
    let now = Instant::now();

    let mut nth_rotation = Permutation::<N>::identity();
    let rotation = Permutation::<N>::rotation();
    let mut cycle_count = BTreeMap::new();
    for _ in 0..N {
        if DISPLAY {
            nth_rotation.display();
        }
        // Counts the number of elements with each cycle length
        cycle_count
            .entry(nth_rotation.first_cycle_length())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        nth_rotation = nth_rotation.compose(&rotation);
    }

    cycle_count
        .iter()
        .for_each(|(length, count)| println!("Length {}: \t{}", length, count));

    println!("Finished in {:?}", now.elapsed());
}
