use dice_simulation::{pretty_print, simulate};
use std::time::Instant;

fn main() {
    let num_of_dice = 5;
    let iterations = 10_000_000;
    let start = Instant::now();
    let map1 = simulate(iterations, num_of_dice);
    let duration = start.elapsed();
    println!(
        "Time elapsed in simulate function() is for {} iterations: {:?}",
        iterations, duration
    );

    pretty_print(&map1, iterations);
}
