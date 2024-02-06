use std::{io::{self, BufRead}};

pub struct Arguments<'a> {
    lines: io::Lines<io::StdinLock<'a>>
}

impl Arguments<'_> {
    pub fn new() -> Arguments<'static> {
        let stdin = io::stdin();
        let lines = stdin.lock().lines();

        Arguments { lines }
    }

    pub fn get_arg<T>(&mut self, name: &str) -> T 
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug
    {
        self.lines.next().unwrap().unwrap().parse().expect(&format!("Failed to parse '{}'", name))
    }
}