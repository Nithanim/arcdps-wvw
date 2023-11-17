pub fn a() {
    let workload = Arc::new(Mutex::new(VecDeque::new()));
    workload.lock().unwrap().push_back(0);

    let thread_1_queue = workload.clone();
    let thread_1 = thread::spawn(move || {
        let mut counter1: i32 = 0;
        let some_time = time::Duration::from_millis(50);

        loop {
            counter1 += 1;
            thread_1_queue.lock().unwrap().push_back(counter1);

            println!("Thread #1: {:?}", thread_1_queue.lock().unwrap());

            if counter1 == 10 {
                break;
            }

            thread::sleep(some_time);
        }
    });

    let thread_2_queue = workload.clone();
    let thread_2 = thread::spawn(move || {
        let mut counter2: i32 = 10;
        let some_time = time::Duration::from_millis(50);

        loop {
            counter2 += 1;
            thread_2_queue.lock().unwrap().push_back(counter2);

            println!("Thread #2: {:?}", thread_2_queue.lock().unwrap());

            if counter2 == 20 {
                break;
            }

            thread::sleep(some_time);
        }
    });

    let some_time = time::Duration::from_millis(50);

    loop {
        if workload.lock().unwrap().capacity() == 10 {
            break;
        }

        println!("MainQueue: {:?}", workload.lock().unwrap());

        thread::sleep(some_time);
    }

    thread_1.join();
    thread_2.join();
}