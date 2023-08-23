use std::sync::{self, atomic::AtomicBool, mpsc::RecvError, Arc};

use rdev::{Event, EventType, Key};

fn main() {
    println!("Hello, world!");

    let kill = Key::F2;

    let (tx, rx) = sync::mpsc::channel();
    let th = std::thread::spawn(move || {
        let mut stack = Vec::with_capacity(1024 * 10);
        let mut stack2 = vec![];
        while let Ok(e) = rx.recv() {
            let e: Event = e;
            stack.push(e.event_type);
            if stack.len() == stack.capacity() {
                stack.dedup();
                stack2.extend_from_slice(&stack);
                stack.clear();
            }
        }

        stack2
    });

    let check = Arc::new(AtomicBool::new(false));
    loop {
        if check.load(sync::atomic::Ordering::Relaxed) {
            break;
        }

        let check = check.clone();
        let tx = tx.clone();
        let kill = kill;

        let func = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                if key == kill {
                    check.swap(true, sync::atomic::Ordering::SeqCst);
                }
            };
            tx.send(event).unwrap();
        };
        if let Err(e) = rdev::listen(func) {
            eprintln!("{e:?}");
        }
    }

    th.join().unwrap();
}

fn callback(event: Event) {
    let et = event.event_type;
}

struct RDev {}
