use std::io::Write;
use std::thread;
use std::sync::{ Mutex, Arc, RwLock };
use lazy_static::lazy_static;

lazy_static!(
    static ref PRIMES: Arc<RwLock<Vec<usize>>> = Arc::new(RwLock::new(vec![]));
);

pub fn calc(limit: usize, chunks: usize, jobs: usize) {
    for i in (1..=limit).step_by(chunks) {
        let mut file = std::fs::File::create(format!("Fork-Join-{}-{}", i, std::cmp::min(limit, i+chunks-1)).as_str()).unwrap();
        let mut j = i;
        while j < std::cmp::min(limit+1, i+chunks) {
            let buf_arc = Arc::new(Mutex::new(vec![]));
            let mut threads = vec![];
            while j < std::cmp::min(limit+1, i+chunks) && threads.len() < jobs {
                if j == 1 {
                    j += 1;
                    continue;
                }

                if PRIMES.read().unwrap().is_empty() {
                    let mut w = PRIMES.write().unwrap();
                    w.push(j);
                    let mut buf = buf_arc.lock().unwrap();
                    buf.push(j);
                    j += 1;
                    continue;
                }

                let mut bad = false;
                for l in PRIMES.read().unwrap().iter().take(1000).take_while(|l| *l * *l <= j) {
                    if j % l == 0 {
                        bad = true;
                        break;
                    }
                }
                if bad {
                    j += 1;
                    continue;
                }

                let k = j;
                let buf_mutex = buf_arc.clone();
                let th = thread::spawn(move || {
                    let primes = PRIMES.read().unwrap();
                    for l in primes.iter().skip(1000).take_while(|l| *l * *l <= k) {
                        if k % l == 0 {
                            return;
                        }
                    }
                    let last = *primes.last().unwrap();
                    for l in (last+1..=k).take_while(|l| l * l <= k) {
                        if k % l == 0 {
                            return;
                        }
                    }

                    loop {
                        if let Ok(mut buf) = buf_mutex.lock() {
                            buf.push(k);
                            return;
                        }
                    }
                });

                threads.push(th);
                j += 1;
            }

            for th in threads {
                th.join().unwrap();
            }
            if let Ok(mut buf) = buf_arc.lock() {
                buf.sort();
                let mut primes = PRIMES.write().unwrap();
                for v in &*buf {
                    writeln!(file, "{}", v).unwrap();
                    primes.push(*v);
                }
            };
        }

        eprintln!("Chunk {}-{} is completed.", i, i+chunks-1);
    }
}
