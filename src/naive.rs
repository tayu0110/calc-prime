use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static PRIMES: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(vec![]));
}

pub fn calc(limit: usize, chunks: usize, _jobs: usize) {
    let primes = &*PRIMES.with(|p| p.clone());
    let mut primes = primes.borrow_mut();

    for i in (1..=limit).step_by(chunks) {
        let mut file = std::fs::File::create(format!("Naive-{}-{}", i, i+chunks-1)).unwrap();
        for j in i..std::cmp::min(limit+1, i+chunks) {
            if j == 1 {
                continue;
            }


            if primes.is_empty() {
                primes.push(j);
                writeln!(file, "{}", j).unwrap();
                continue;
            }

            let mut bad = false;
            for k in primes.iter().take_while(|k| *k * *k <= j) {
                if j % k == 0 {
                    bad = true;
                    break;
                }
            }

            if !bad {
                primes.push(j);
                writeln!(file, "{}", j).unwrap();
            }
        }

        eprintln!("Chunk {}-{} is completed.", i, i+chunks-1);
    }
}
