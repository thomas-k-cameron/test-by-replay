use std::{
    borrow::Cow, fs::{self, File}, path::Path, sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver},
    }, time::{SystemTime, UNIX_EPOCH}
};

use rdev::{Event, EventType, Key};

pub fn main() {
    let mut check = AtomicBool::new(false);
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut func = || {
            let tx = tx.clone();
            let mut check = false;
            rdev::listen(move |event| {
                let _ = tx.send((event.time, event));
            })
            .unwrap();
        };

        if cfg!(target_os = "macos") {
            loop {
                func();
            }
        } else {
            func();
        }
    });

    let th = std::thread::spawn(move || {
        let mut stack = Vec::with_capacity(1024 * 10);
        let mut stack2 = vec![];
        while let Ok((time, e)) = rx.recv() {
            if e.event_type == EventType::KeyPress(Key::F2) {
                break;
            }
            stack.push((time, e.event_type));
            if stack.len() == stack.capacity() {
                stack.dedup_by_key(|a| a.1);
                stack2.extend_from_slice(&stack);
                stack.clear();
            }
        }

        stack2.extend_from_slice(&stack);
        stack2.dedup();
        stack2
    });

    let mut s = th.join().unwrap();
    s.dedup();
    let slice = s.into_boxed_slice();

    let mut file_path = uuid::Uuid::new_v4().to_string();
    let mut fp = File::create(file_path).unwrap();

    let mut s = "".to_string();
    for i in slice {
        let mut jsonstr = serde_json::to_string(&i).unwrap();
        s.push_str(&jsonstr);
        s.push('\n');
        s.shrink_to_fit();
    }

    zstd::stream::copy_encode(std::io::Cursor::new(jsonstr), fp, 5);
}

pub fn replay(slice: Box<[(SystemTime, EventType)]>) {
    let mut prev = slice.iter().next().map(|i| i.0).unwrap();
    for (time, e_ty) in slice.iter() {
        std::thread::sleep(time.duration_since(prev).unwrap());
        rdev::simulate(e_ty).unwrap();
    }
}

pub fn stream_file(path: impl AsRef<Path>) -> Box<[(SystemTime, EventType)]> {
    let slice = zstd::decode_all(std::fs::File::open(path).unwrap())
        .unwrap()
        .into_boxed_slice();
    serde_json::from_slice(&slice).unwrap()
}
