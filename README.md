
# Optimized Filecoin PC2
Standalone C++ GPU implementation of Filecoin's PreCommit2 function

**This repository is for demonstration purposes only. Feel free to modify as needed to support your specific setup.**

## Build and Running
```
./build.sh
export P2_POSEIDON_CONSTANTS=your poseidon constants
export CACHE_PATH=your cache path
export REPLICA_PATH=your replica path
export SECTOR_SIZE=your sector size
cargo test --release --test p2
```

## Various Notes
- The pc2 performance for 32GB sectors is ~150 seconds (2.5 min) on a 3090 GPU.
- Currently only CC sectors are supported. In order to work with non-CC sectors, the code needs to be updated to add the piece_file with the final layer prior to building tree r.
- The test includes timed pinned memory allocation and deallocation. If used in an application, the pinning only needs to performed once then reused across many different sectors.
- The test will check the tree c and tree r output data against the files in the input path. If run on non-CC sectors then tree c should match although tree r will fail.
