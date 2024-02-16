use std::{cell::RefCell, time::Instant, vec};
use crate::arguments;

pub trait GreadyAlorithm {
    fn new (currencies: Vec<usize>) -> Self;
    fn run (&mut self, n: usize) -> Option<usize> {
        return self.reiterate(n);
    }
    fn reiterate(&mut self, n: usize) -> Option<usize> {
        self.gready_min_coins(n)
    }
    fn get_currencies(&self) -> Vec<usize>;
    fn gready_min_coins(&mut self, n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }

        return self.get_currencies().into_iter().filter_map(|coin| {
            if coin <= n {
                if let Some(sub_coins) = self.reiterate(n - coin) {
                    return Some(1 + sub_coins);
                }
            }

            return None;
        }).min()
    }
}

pub struct MethodA {
    currencies: Vec<usize>
}
impl GreadyAlorithm for MethodA {
    fn new (currencies: Vec<usize>) -> Self {
        MethodA { currencies }
    }   

    fn get_currencies(&self) -> Vec<usize> {
        self.currencies.clone()
    }
}

pub struct MethodC {
    currencies: Vec<usize>,
    memory: Vec<Option<usize>>,
}

impl GreadyAlorithm for MethodC {
    fn new (currencies: Vec<usize>) -> Self {
        MethodC { currencies, memory: Vec::new() }
    }

    fn run (&mut self, n: usize) -> Option<usize> {
        self.memory = vec![None; n + 1];
        self.reiterate(n)
    }

    fn reiterate (&mut self, value: usize) -> Option<usize> {
        if let Some(memory_coins) = self.memory[value] {
            return Some(memory_coins);
        }

        let coins = self.gready_min_coins(value);
        self.memory[value] = coins;
        return coins   
    }

    fn get_currencies(&self) -> Vec<usize> {
        self.currencies.clone()
    }
}

pub struct MethodE {
    currencies: Vec<usize>,
    memory: Vec<Option<usize>>,
}

impl GreadyAlorithm for MethodE {
    fn new (currencies: Vec<usize>) -> Self {
        MethodE { currencies, memory: Vec::new() }
    }

    fn run (&mut self, n: usize) -> Option<usize> {
        self.memory = vec![None; n + 1];
        self.memory[0] = Some(0);

        for i in 0..n + 1 {
            self.memory[i] = self.gready_min_coins(i);
        }

        self.reiterate(n)
    }

    fn reiterate(&mut self, n: usize) -> Option<usize> {
        self.memory[n]
    }

    fn get_currencies(&self) -> Vec<usize> {
        self.currencies.clone()
    }
}

pub fn kattis () {
    let mut arguments = arguments::Arguments::new();

    let n: usize = arguments.get_arg("n");
    let a: usize = arguments.get_arg("a");
    let b: usize = arguments.get_arg("b");
    let c: usize = arguments.get_arg("c");

    let mut method = MethodE {
        currencies: vec![1, a, b, c],
        memory: Vec::new()
    };

    if let Some(n) = method.run(n) {
        println!("{}", n);
    } else {
        println!("no solution");
    }
}

pub fn test () {
    let mut method = MethodC {
        currencies: vec![1, 3, 4],
        memory: Vec::new()
    };

    if let Some(n) = method.run(10) {
        println!("{}", n);
    } else {
        println!("no solution");
    }
}