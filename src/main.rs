use rocksdb::{
    BlockBasedOptions, BottommostLevelCompaction, Cache, CompactOptions, DBCompactionStyle, Env,
    Error, FifoCompactOptions, IteratorMode, Options, PerfContext, PerfMetric, ReadOptions,
    SliceTransform, Snapshot, UniversalCompactOptions, UniversalCompactionStopStyle, WriteBatch,
    DB,
};

fn main() {
    println!("Hello, world!");
    let path = std::path::Path::new("/tmp/disable_thread");

    // create new options
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_max_background_jobs(0);
    opts.set_stats_dump_period_sec(0);
    opts.set_stats_persist_period_sec(0);

    // test Env::Default()->SetBackgroundThreads(0, Env::Priority::BOTTOM);
    let mut env = Env::default().unwrap();
    env.set_background_threads(0);
    env.set_bottom_priority_background_threads(0);
    env.set_high_priority_background_threads(0);
    env.set_low_priority_background_threads(0);
    opts.set_env(&env);

    // open db
    println!("disable all threads");
    let db = DB::open(&opts, &path).unwrap();
    env.set_bottom_priority_background_threads(0);
    env.set_high_priority_background_threads(0);
    env.set_low_priority_background_threads(0);
    // write a lot
    let mut batch = WriteBatch::default();
    for i in 0..1_00000000 {
        batch.put(format!("{:0>4}", i).as_bytes(), b"v");
    }
    assert!(db.write(batch).is_ok());
    println!("done writing");
    // try to get key
    let iter = db.iterator(IteratorMode::Start);
    for (expected, (k, _)) in iter.enumerate() {
        assert_eq!(k.as_ref(), format!("{:0>4}", expected).as_bytes());
    }
}
