use std::vec;
use crate::arguments::Arguments;

fn method_a (n: usize, k: usize, p: f64) -> f64 {
    let mut current_game_streak = vec![0.0; k + 1];
    let mut last_game_streak = vec![0.0; k + 1];
    last_game_streak[0] = 1.0;

    let mut total_prob: f64 = 0.0;

    for _ in 1..=n {
        current_game_streak[0] = 0.;

        for j in 0..k {
            current_game_streak[0] += last_game_streak[j] * (1.0 - p);
        }

        for j in 1..=k {
            current_game_streak[j] = last_game_streak[j - 1] * p;
        }

        total_prob += current_game_streak[k];

        let temp = last_game_streak;
        last_game_streak = current_game_streak;
        current_game_streak = temp;
    }

    return total_prob;
}

fn method_c (n: usize, k: usize, p: f64) -> f64 {
    let mut probs = vec![0.; n + 1];
    probs[k] = p.powi(k as i32);

    for i in k+1..=n {
        probs[i] = probs[i - 1] + p.powi(k as i32) * (1. - p) * (1. - probs[i - k - 1])
    }

    return probs[n];
}

pub fn kattis () {
    let mut arguments = Arguments::new();

    let n: usize = arguments.get_arg("n");
    let k: usize = arguments.get_arg("k");
    let p: f64 = arguments.get_arg("p");

    println!("{}", method_c(n, k, p));
}

pub fn test () {
    println!("{}", method_c(4, 2, 0.7));
}