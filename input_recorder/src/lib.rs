use std::{
    fs,
    rc::Rc,
    sync::{
        self,
        atomic::AtomicBool,
        mpsc::{self, Receiver, RecvError},
        Arc, Mutex,
    },
    time::SystemTime,
};

use rdev::{Event, EventType, Key};

pub fn main() {
    let rx = listen2input();

    let th = std::thread::spawn(move || {
        let mut stack = Vec::with_capacity(1024 * 10);
        let mut stack2 = vec![];
        while let Ok((_time, e)) = rx.recv() {
            if e.event_type == EventType::KeyPress(Key::F2) {
                break;
            }
            stack.push(e.event_type);
            if stack.len() == stack.capacity() {
                stack.dedup();
                stack2.extend_from_slice(&stack);
                stack.clear();
            }
        }
        stack.dedup();
        stack2.extend_from_slice(&stack);
        stack2
    });

    let mut s = th.join().unwrap();
    s.dedup();
    let slice = s.into_boxed_slice();
    let s = serde_json::to_string(&slice).unwrap();
    fs::write("./asdf.json", &s).unwrap();
}

fn listen2input() -> Receiver<(SystemTime, Event)> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || loop {
        let tx = tx.clone();
        rdev::listen(move |event| {
            let _ = tx.send((event.time, event));
        })
        .unwrap();
    });

    rx
}

pub fn replay(slice: Box<[EventType]>) {
    for i in slice.iter() {
        rdev::simulate(i).unwrap();
    }
}

struct RDev {}
