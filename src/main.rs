use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    // initial value to share and mutate
    // in spawed threads
    let shared = 1;
    // use Arc to share data with thread
    // mutex to allow safe mutation
    let s = Arc::new(Mutex::new(shared));

    // thread handles
    let mut handles = vec![];

    for i in 1..11 {
        // get the inner mutex
        let mutex = s.clone();
        // spawn thread and move 
        let handle = thread::spawn(move || {
            // lock for mutation inside thread
            let mut r = mutex.lock().unwrap();
            *r += i;
            println!("shared: ({} + thread #{}) => {}", *r - i, i, *r);
        });
        // save thread handle to join later
        handles.push(handle);
    }

    for handle in handles {
        // join thread
        handle.join().unwrap();
        // what is the current state of the mutex?
        println!("{:?}", s);
    }
}
