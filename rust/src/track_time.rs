use std::{cell::RefCell, time::Instant};

pub fn track_time<M, I>(input_iterator: I, mut method: M) -> Vec<(usize, f64)> 
    where M: FnMut(usize), I: IntoIterator<Item = usize>
{
    let track_time = |i| {
        let start = Instant::now();
        method(i);
        let duration = start.elapsed();
        (i, duration.as_nanos())
    };
    
    let steps_over_one_second = 3;
    let input_for_one_second = RefCell::new(usize::MAX / 2);

    let time_data = input_iterator
        .into_iter()
        .filter_map(|i| if i <= *input_for_one_second.borrow() + steps_over_one_second { Some(i) } else { None })
        .map(track_time)
        .map(|(i, time)| {
            if time > 1_000_000_000 && *input_for_one_second.borrow() > i {
                *input_for_one_second.borrow_mut() = i;
            } 

            if i == *input_for_one_second.borrow() {
                println!("Input for on second: {}", i)
            } else {
                println!("input: {}, time: {}", i, time);
            }

            return (i, time)
        })
        .map(|(i, time)| (i, time as f64 / 1_000_000_000.0))
        .collect::<Vec<_>>();

    time_data
}