use std::time::{Duration, Instant};
use sys_info::{mem_info, cpu_num};


// function to calculate the nth Fibonacci number
fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    let start_time: Instant = Instant::now();
    let result: u64 = fib(35);
    let end_time: Instant = Instant::now();
    let memory_info: sys_info::MemInfo = mem_info().unwrap();

    println!("35th Fibonacci number: {}", result);
    println!(
        "Memory Usage: {:.2}%",
        (memory_info.total - memory_info.avail) as f32 / memory_info.total as f32 * 100.0
    );
    
    // measure CPU usage
    let num_cores = cpu_num().unwrap();
    let elapsed_time = end_time.duration_since(start_time);
    let cpu_usage = elapsed_time.as_secs_f64() * num_cores as f64 * 100.0 / Duration::from_secs(1).as_secs_f64();

    println!("CPU used: {:.2}%", cpu_usage);
    println!("Time taken: {:.6} seconds", end_time.duration_since(start_time).as_secs_f64());

}
