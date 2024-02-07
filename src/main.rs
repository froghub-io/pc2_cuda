use std::path::PathBuf;
use clap::Parser;
use pc2_cuda::treecr;

#[derive(Debug, Parser)]
#[clap(name = "pc2-cuda")]
pub struct Options {
    #[clap(long, short)]
    pub cache_path: String,
    #[clap(long, short)]
    pub replica_path: String,
    #[clap(long, short)]
    pub sector_size: String,
}

impl Options {
    pub fn get_cache_path(&self) -> PathBuf {
        PathBuf::from(&self.cache_path)
    }
    pub fn get_replica_path(&self) -> PathBuf {
        PathBuf::from(&self.replica_path)
    }
    pub fn get_sector_size(&self) -> String {
        self.sector_size.clone()
    }
}

fn main() {
    let options = Options::parse();
    treecr(options.get_cache_path(), options.get_replica_path(), &options.get_sector_size());
}