mod thread_learn;

pub fn start() {
    println!("thread learn ............");
    thread_learn::thread_test_spawn();
    thread_learn::thread_test_chan();
}
