use make_singleton::MakeSingletonThreadSafe;
use std::thread;

#[derive(Default, MakeSingletonThreadSafe)]
pub struct Counter {
    num: i32,
}

impl Counter {
    pub fn count_number(&mut self) {
        self.num += 1;
        println!(
            "thread is {:?} and num is {}",
            thread::current().id(),
            self.num
        );
    }
}

fn main() {
    println!("Hello, world! Let's start counting...");

    let thr = thread::spawn(move || {
        for _ in 0..100 {
            Counter::instance_thread_safe()
                .lock()
                .unwrap()
                .count_number();
            thread::sleep_ms(10);
        }
    });

    for _ in 0..100 {
        Counter::instance_thread_safe()
            .lock()
            .unwrap()
            .count_number();
        thread::sleep_ms(10);
    }

    thr.join();
    println!("count over~! bye~");
}
