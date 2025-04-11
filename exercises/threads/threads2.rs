use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        // 打印当前完成的作业数
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
    
    // 最终完成的作业数
    println!("total jobs completed: {}", status.lock().unwrap().jobs_completed);
}