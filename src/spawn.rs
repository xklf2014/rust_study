use std::thread;
use std::time::Duration;

pub fn spawn_function(){
    for i in 0..5 {
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }
}