use std::time::Instant;

pub fn time_function<F, V, I>(func: F, iter: I) -> Vec<(V, u128)> 
    where
        F: Fn(V),
        V: Into<usize> + Copy,
        I: IntoIterator<Item = V> + Clone,
{
    for i in iter.clone() { func(i); }

    let mut times = Vec::new();
    

    for i in iter {
        let start = Instant::now();
        func(i);
        let duration = start.elapsed();
        times.push((i, duration.as_micros()));
    }

    return times
}

