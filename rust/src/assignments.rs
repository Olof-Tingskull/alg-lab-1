use std::{error::Error};
use crate::{coins, plot, track_time::track_time, winning_streak};
use coins::{GreadyAlorithm};



pub fn assignment_1a() -> Result<(), Box<dyn Error>> {
    let a = 5;
    let b = 6;
    let c = 7;
    let currencies = vec![1, a, b, c];
    let mut method = coins::MethodA::new(currencies);

    let coins = method.run(15);

    if let Some(coins) = coins {
        println!("Method A: {}", coins);
    } else {
        println!("Method A: No solution found");
    }

    Ok(())
}

pub fn assignment_1b() -> Result<(), Box<dyn Error>> {
    let a = 5;
    let b = 6;
    let c = 7;
    let currencies = vec![1, a, b, c];

    let mut method = coins::MethodA::new(currencies);

    plot::plot("1b-time", track_time(1..50, |i| { method.run(i); }))?;

    Ok(())
}

pub fn assignment_1c() -> Result<(), Box<dyn Error>> {
    let mut method = coins::MethodC::new(vec![1, 5, 10, 25, 50]);
    let coins = method.run(63);

    if let Some(coins) = coins {
        println!("Method A: {}", coins);
    } else {
        println!("Method A: No solution found");
    }

    Ok(())  
}

pub fn assignment_1d() -> Result<(), Box<dyn Error>> {
    let a = 5;
    let b = 6;
    let c = 7;
    let currencies = vec![1, a, b, c];

    let mut method = coins::MethodC::new(currencies);

    plot::plot("1d-time", track_time(1..5000, |i| { method.run(i); }))?;

    Ok(())
}

pub fn assignment_1e() -> Result<(), Box<dyn Error>> {
    let a = 5;
    let b = 6;
    let c = 7;
    let currencies = vec![1, a, b, c];

    let mut method = coins::MethodE::new(currencies);

    let coins = method.run(63);

    if let Some(coins) = coins {
        println!("Method A: {}", coins);
    } else {
        println!("Method A: No solution found");
    }

    Ok(())
}

pub fn assignment_2a() -> Result<(), Box<dyn Error>> {
    winning_streak::method_a(4, 2, 0.9);
    Ok(())
}

pub fn assignment_2b() -> Result<(), Box<dyn Error>> {    
    plot::plot("2b-time", track_time(1..1000, |i| { winning_streak::method_a(i, i/2, 0.99); }))?;

    Ok(())
}

pub fn assignment_2c() -> Result<(), Box<dyn Error>> {
    winning_streak::method_c(4, 2, 0.9);
    Ok(())
}

pub fn assignment_2d() -> Result<(), Box<dyn Error>> {
    plot::plot("2d-time", track_time(1..1000, |i| { winning_streak::method_c(i, i/2, 0.99); }))?;

    Ok(())
}