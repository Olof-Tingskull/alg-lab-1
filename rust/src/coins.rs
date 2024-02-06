use std::{io::{self, BufRead}, vec};
use std::time::Instant;

use crate::arguments;

pub fn gready_min_coins_recursive(total_value: usize, currencies: &Vec<usize>, memory: &mut Option<Vec<usize>>) -> usize {
    if let Some(memory) = memory {
        if let Some(&value) = memory.get(total_value) {
            if value != usize::MAX {
                return value;
            }
        }
    }

    let mut min_coins = total_value;

    for &currency in currencies {
        if total_value >= currency {
            min_coins = min_coins.min(gready_min_coins_recursive(total_value - currency, currencies, memory) + 1);
        }
    }

    if let Some(memory) = memory {
        (*memory)[total_value] = min_coins;
    }
    return min_coins;
}

pub fn gready_min_coins(total_value: usize, currencies: &Vec<usize>) -> usize {
    let mut min_coins_for_value = vec![0; total_value + 1];

    for current_value in 1..=total_value {
        let mut current_min_coins = current_value;
        
        for &currency in currencies {
            if current_value >= currency {
                current_min_coins = current_min_coins.min(min_coins_for_value[current_value - currency] + 1);
            }
        }

        min_coins_for_value[current_value] = current_min_coins;
    }

    return min_coins_for_value[total_value];
}

pub fn kattis () {
    let mut arguments = arguments::Arguments::new();

    let n: usize = arguments.get_arg("n");
    let a: usize = arguments.get_arg("a");
    let b: usize = arguments.get_arg("b");
    let c: usize = arguments.get_arg("c");

    println!("{}", gready_min_coins(n, &vec![a, b, c]));
}