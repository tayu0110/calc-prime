use clap::Parser;

pub mod naive;
pub mod fork_join;
pub mod thread_pool;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help = "Upper limit of calculation. Default is 2^64-1.", value_name = "LIMIT")]
    limit: Option<usize>,
    #[clap(short, long, help = "Results are stored in a file for each number of chunks. Default is 1000000.", value_name = "CHUNKS")]
    chunks: Option<usize>,
    #[clap(short, long, help = "Number of the thread. Ignore if specified -s.", value_name = "JOBS")]
    jobs: Option<usize>,
    #[clap(short, long, help = "Use the single-thread.")]
    single_thread: bool,
    #[clap(short, long, help = "Use the multi-thread with fork/join.")]
    fork_join: bool,
    #[clap(short, long, help = "Use the multi-thread with thread-pool.")]
    thread_pool: bool
}

fn main() {
    let arg = Args::parse();

    let limit = match arg.limit {
        Some(limit) => limit,
        None => std::usize::MAX
    };

    let chunks = match arg.chunks {
        Some(chunks) => chunks,
        None => 1000_000
    };

    let jobs = match arg.jobs {
        Some(jobs) => jobs,
        None => 1
    };

    if arg.single_thread {
        naive::calc(limit, chunks, jobs);
    } else if arg.fork_join {
        fork_join::calc(limit, chunks, jobs);
    } else if arg.thread_pool {
        thread_pool::calc(limit, chunks, jobs);
    } else {
        naive::calc(limit, chunks, jobs);
    }
}
