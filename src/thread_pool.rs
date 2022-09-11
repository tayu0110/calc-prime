use std::io::Write;
use std::sync::{
    mpsc, Arc, RwLock
};
use threadpool::ThreadPool;
use lazy_static::lazy_static;

lazy_static!(
    static ref PRIMES: Arc<RwLock<Vec<usize>>> = Arc::new(RwLock::new(vec![]));
);

pub fn calc(limit: usize, chunks: usize, jobs: usize) {
    let pool = ThreadPool::new(jobs);
    let (tx, rx) = mpsc::channel();

    for i in (1..=limit).step_by(chunks) {
        let mut file = std::fs::File::create(format!("Thread-Pool-{}-{}", i, i+chunks-1)).unwrap();
        for j in i..std::cmp::min(limit+1, i+chunks) {
            let k = j;
            let tx = tx.clone();
            pool.execute(move || {
                if j == 1 {
                    tx.send(None).expect("channel will be there waiting for the pool");
                    return;
                }

                let primes = PRIMES.read().unwrap();

                for l in primes.iter().take_while(|l| *l * *l <= k) {
                    if k % l == 0 {
                        tx.send(None).expect("channel will be there waiting for the pool");
                        return;
                    }
                }

                let last = match primes.last() {
                    Some(last) => *last,
                    None => 1
                };
                for l in (last+1..=k).take_while(|l| *l * *l <= k) {
                    if k % l == 0 {
                        tx.send(None).expect("channel will be there waiting for the pool");
                        return;
                    }
                }

                tx.send(Some(k)).expect("channel will be there waiting for the pool");
            });
        }

        let buf = rx.iter().take(std::cmp::min(chunks, limit-i+1)).collect::<Vec<Option<usize>>>();
        let mut buf = buf.iter().filter(|v| **v != None).map(|v| v.unwrap()).collect::<Vec<_>>();
        buf.sort();
        for v in buf {
            PRIMES.write().unwrap().push(v);
            writeln!(file, "{}", v).unwrap();
        }

        eprintln!("Chunk {}-{} is completed.", i, i+chunks-1);
    }
}
