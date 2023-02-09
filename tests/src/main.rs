use binder::ProcessState;

fn main() {
    ProcessState::set_thread_pool_max_thread_count(0);
    ProcessState::start_thread_pool();

    println!("Hello, world!");

    ProcessState::join_thread_pool();
}
