// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        // 创建一个新线程，计算其运行时间并返回
        handles.push(thread::spawn(move || {
            let start = Instant::now(); // 记录线程开始时间
            thread::sleep(Duration::from_millis(250)); // 使线程休眠250毫秒
            println!("thread {} is complete", i); // 打印线程完成信息
            start.elapsed().as_millis() // 返回线程运行时间（毫秒）
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // 使用 join 方法等待线程完成并收集返回值
        results.push(handle.join().unwrap()); // 收集线程返回的运行时间
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!"); // 检查所有线程是否完成
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        // 打印每个线程的运行时间
        println!("thread {} took {}ms", i, result);
    }
}