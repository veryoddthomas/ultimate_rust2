// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    // Pretend our fancy little filter-map-sum is expensive and takes 500ms
    sleep_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 1. Spawn a child thread and have it call `expensive_sum(my_vector)`. Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the output from the child thread's expensive sum in the middle of the main
    // thread's processing of letters.
    //
    let handle = thread::spawn(move || {
        let sum = expensive_sum(my_vector);
        println!("Child thread: The sum is {}", sum);
        sum
    });

    // Main thread: Processing the letter 'a'
    // Main thread: Processing the letter 'b'
    // Main thread: Processing the letter 'c'
    // Child thread: just about finished
    // Child thread: The sum is 20
    // Main thread: Processing the letter 'd'
    // Main thread: Processing the letter 'e'
    // Main thread: Processing the letter 'f'
    // Main thread: Exiting.

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    // 2. Let's retrieve the value returned by the child thread once it has exited.
    // - Uncomment and complete the code below.
    // - Call the .join() method on `handle` from #1 and assign the `Result<i32, Err>` it returns
    // to a variable named `result`
    // - Get the i32 out of `result` and store it in a `sum` variable.

    let result = match handle.join() {
        Ok(sum) => sum,
        Err(_) => panic!("Child thread panicked"),
    };

    let sum = result;

    println!("The child thread's expensive sum is {}", sum);

    // 3. Time for some fun with channels!
    // - Uncomment the block comment below (Find and remove the `/*` and `*/`).
    // - Create variables `tx` and `rx` and assign them to the sending and receiving ends of an
    // unbounded channel. Hint: An unbounded channel can be created with `channel::unbounded()`

    let (tx, rx) = channel::unbounded();

    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread. We want another variable that can be used for sending...
    let tx2 = tx.clone();

    // 4. Examine the flow of execution of "Thread A" and "Thread B" below. Do you see how their
    // output will mix with each other?
    // - Run this code. Notice the order of output from Thread A and Thread B.
    // - Increase the value passed to the first `sleep_ms()` call in Thread A so that both the
    // Thread B outputs occur *before* Thread A outputs anything.
    // - Run the code again and make sure the output comes in a different order.

    // Thread A
    let handle_a = thread::spawn(move || {
        sleep_ms(400);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    sleep_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    // Thread B
    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed. A Receiver channel is automatically closed once all Sender channels have been
    // closed. Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // 5. Oops, we forgot to join "Thread A" and "Thread B". That's bad hygiene!
    // - Use the thread handles to join both threads without getting any compiler warnings.

    if let Err(e) = handle_a.join() {
        println!("Thread A panicked: {:?}", e);
    }
    if let Err(e) = handle_b.join() {
        println!("Thread B panicked: {:?}", e);
    }
    // Challenge: Make two child threads and give them each a receiving end to a channel. From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`). Join
    // the child threads.

    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();
    let rx2 = rx.clone();

    // Thread C
    let handle_c = thread::spawn(move || loop {
        match rx.recv_timeout(Duration::from_millis(1000)) {
            Ok(msg) => println!("Thread C: Received {}", msg),
            Err(channel::RecvTimeoutError::Disconnected) => break,
            Err(channel::RecvTimeoutError::Timeout) => break,
        }
        sleep_ms(100);
    });

    // Thread D
    let handle_d = thread::spawn(move || loop {
        match rx2.recv_timeout(Duration::from_millis(1000)) {
            Ok(msg) => println!("Thread D: Received {}", msg),
            Err(channel::RecvTimeoutError::Disconnected) => break,
            Err(channel::RecvTimeoutError::Timeout) => break,
        }
        sleep_ms(100);
    });

    for letter in vec!["A", "B", "C", "D", "E", "F"] {
        println!("Main thread: Sending the letter '{}'", letter);
        tx.send(letter).unwrap();
        // sleep_ms(200);
    }
    drop(tx);

    if let Err(e) = handle_c.join() {
        println!("Thread C panicked: {:?}", e);
    }
    if let Err(e) = handle_d.join() {
        println!("Thread D panicked: {:?}", e);
    }

    println!("Main thread: Exiting.")
}
