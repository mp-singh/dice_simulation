use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};

pub fn simulate(iterations: usize, num_of_dice: usize) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for _ in 0..iterations {
        let mut num_of_dice = num_of_dice;
        let mut answer = 0;
        while num_of_dice > 0 {
            let roll = roll_dice(num_of_dice, rand::thread_rng());
            let (ans, len) = score(roll);
            answer += ans;
            map.entry(answer).and_modify(|e| *e += 1).or_insert(1);
            num_of_dice = len
        }
    }
    map
}

fn roll_dice(n: usize, mut rng: ThreadRng) -> Vec<i32> {
    let mut roll = Vec::with_capacity(n);
    for _ in 0..n {
        roll.push(rng.gen_range(1..=6));
    }
    roll
}

fn score(mut roll: Vec<i32>) -> (i32, usize) {
    if roll.contains(&3) {
        return (0, roll.iter().filter(|&x| x != &3).count());
    }
    roll.sort();
    (*roll.first().unwrap(), roll.len() - 1)
}

pub fn pretty_print(map: &HashMap<i32, i32>, iterations: usize) {
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();
    for k in keys {
        println!(
            "Total {} occurs {:.3} occured {} times",
            k,
            (map[k] as f64 / iterations as f64),
            map[k]
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_roll_dice() {
        let rng = rand::thread_rng();
        let roll = roll_dice(5, rng);
        assert_eq!(roll.len(), 5);
    }
    #[test]
    fn test_score() {
        let roll = vec![1, 2, 3, 4, 5];
        assert_eq!(score(roll), (0, 4));
    }
    #[test]
    fn test_score_more_than_one_three() {
        let roll = vec![1, 2, 3, 3, 4];
        assert_eq!(score(roll), (0, 3));
    }
    #[test]
    fn test_score_example() {
        let roll = vec![3, 1, 3, 6, 6];
        assert_eq!(score(roll), (0, 3));
    }
    #[test]
    fn test_score_example_all_3() {
        let roll = vec![3, 3, 3, 3, 3];
        assert_eq!(score(roll), (0, 0));
    }
}
