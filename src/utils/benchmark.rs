use std::time::Instant;

pub fn bench<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("{} => {} ms", label, elapsed.as_millis());
    result
}
