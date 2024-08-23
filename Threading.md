```
use std::thread;
use std::time::Duration;

thread::spawn(move || loop {
    println!("Hi from Ted the thread. I print to stdout every 1_000 ms.");
    thread::sleep(Duration::from_millis(1_000));
});
```
