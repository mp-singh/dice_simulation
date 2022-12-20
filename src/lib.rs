use std::collections::HashMap;

use rand::{rngs::ThreadRng, seq::SliceRandom};

pub fn simulate(iterations: usize, num_of_dice: usize) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for _ in 0..iterations {
        let mut num_of_dice = num_of_dice;
        let mut answer = 0;
        while num_of_dice > 0 {
            let rng = rand::thread_rng();
            let mut roll = roll_dice(num_of_dice, rng);
            answer += score(&mut roll);
            map.entry(answer).and_modify(|e| *e += 1).or_insert(1);
            num_of_dice = roll.len();
        }
    }
    map
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

fn roll_dice(n: usize, mut rng: ThreadRng) -> Vec<i32> {
    let roll = [1, 2, 3, 4, 5, 6]
        .choose_multiple(&mut rng, n)
        .cloned()
        .collect::<Vec<_>>();
    roll
}

fn score(roll: &mut Vec<i32>) -> i32 {
    if roll.contains(&3) {
        roll.retain(|&x| x != 3);
        return 0;
    }
    roll.sort();
    roll.remove(0)
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
        let mut roll = vec![1, 2, 3, 4, 5];
        assert_eq!(score(&mut roll), 0);
        assert_eq!(roll, vec![1, 2, 4, 5]);
    }
    #[test]
    fn test_score_more_than_one_three() {
        let mut roll = vec![1, 2, 3, 3, 4];
        assert_eq!(score(&mut roll), 0);
        assert_eq!(roll, vec![1, 2, 4]);
    }
    #[test]
    fn test_score_example() {
        let mut roll = vec![3, 1, 3, 6, 6];
        assert_eq!(score(&mut roll), 0);
    }
}
