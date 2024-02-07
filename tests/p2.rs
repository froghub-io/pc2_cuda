// Copyright Supranational LLC

use pc2_cuda::treecr;
use std::path::PathBuf;
use std::env;

#[test]
fn run_seal() {
    let  cache_path = PathBuf::from(env::var("CACHE_PATH").expect("export CACHE_PATH=your cache path"));
    let  replica_path = PathBuf::from(env::var("REPLICA_PATH").expect("export REPLICA_PATH=your replica path"));
    let  sector_size = env::var("SECTOR_SIZE").expect("export SECTOR_SIZE=your sector sizer");
    let treecr_start = std::time::Instant::now();
    treecr(cache_path, replica_path, &sector_size);
    println!("treecr time: {:?}", treecr_start.elapsed());
}
