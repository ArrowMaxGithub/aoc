use std::{
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::channel,
    },
    thread::JoinHandle,
    time::Duration,
};

static LUT: [u32; 16777216 / size_of::<u32>()] = unsafe {
    std::mem::transmute(*include_bytes!(concat!(
        env!("OUT_DIR"),
        "/LUT_2015d2p1.bin"
    )))
};

pub fn main() {
    let min_bits = 11;
    let mut num_bits = 24;
    while num_bits >= min_bits {
        eprintln!("Starting with {num_bits} max bits");
        let magic = find_magic(num_bits, &LUT);
        eprintln!("Found magic: {magic} with {num_bits} bits");

        num_bits -= 1;
    }
    pause();
}

pub fn find_magic(num_bits: usize, data: &'static [u32]) -> u32 {
    let iter = Arc::new(AtomicUsize::new(0));
    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel();

    let handles: Vec<JoinHandle<()>> = (0..32)
        .map(|_| {
            let tx = tx.clone();
            let iter = iter.clone();
            let cancel = cancel.clone();
            std::thread::spawn(move || {
                while !cancel.load(Ordering::Acquire) {
                    iter.fetch_add(1, Ordering::Relaxed);
                    let mut seen = HashSet::new();
                    let magic = rand::random();
                    let mut collision = false;

                    for hash in data {
                        let reduced = hash.wrapping_mul(magic) >> (64 - num_bits);
                        if !seen.insert(reduced) {
                            collision = true;
                            break;
                        }
                    }

                    if !collision {
                        tx.send(magic).unwrap();
                        break;
                    }
                }
            })
        })
        .collect();

    let mut last_iter = iter.load(Ordering::Relaxed);
    let magic = loop {
        let iter = iter.load(Ordering::Relaxed);
        let iter_s = (iter - last_iter) as f64 / 10.0;
        last_iter = iter;
        eprintln!("Iterations: {} | iter/s: {:.2}", iter, iter_s);
        if let Ok(magic) = rx.recv_timeout(Duration::from_secs(10)) {
            break magic;
        }
    };

    cancel.store(true, Ordering::Release);
    for h in handles {
        h.join().unwrap();
    }

    magic
}

fn pause() {
    use std::io::prelude::*;
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
