use std::time::Instant;
use dice_simulation::simulate;

fn main() {
    let num_of_dice = 5;
    let iterations = 1_000_000;
    let start = Instant::now();
    simulate(iterations, num_of_dice);
    let duration = start.elapsed();
    println!("Time elapsed in simulate function() is: {:?}", duration);
}
