#!/bin/bash

set -e
set -x

#git submodule init
#git submodule update --remote

# Volta - 70
# Turing (2080) - 75
# Ampere (3080) - 86
cuda_arch=70

while getopts a: flag
do
    case "${flag}" in
        a) cuda_arch=${OPTARG};;
    esac
done

if [ ! -f "blst/libblst.a" ]; then
    git clone https://github.com/supranational/sppark.git
    cd sppark
    git reset --hard ad3e97274bcba637a03eff0dbdc11f56d5c1ce44
    cd ../
    git clone https://github.com/supranational/blst.git
    cd blst
    git reset --hard c6a3cc00ca0a9e9fb6ad2b2633a031dcea5786f2
    sh build.sh
    cd ..
fi

LIBS="-Lblst -lblst"
INCLUDES="-Iblst/src -Isppark"
FLAGS="-O2 -D__ADX__ -arch=sm_$cuda_arch -Xcompiler -Wno-subobject-linkage -Xcompiler -O3"
OMP="-Xcompiler -fopenmp"

nvcc $LIBS $INCLUDES $FLAGS -c cuda/tree_builder_device.cu

nvcc $LIBS $INCLUDES -Isppark/util $FLAGS -c cuda/pre_commit_phase2.cu $OMP

ar rc libpc2.a pre_commit_phase2.o tree_builder_device.o blst/libblst.a

rm -fr lib
mkdir -p lib

sudo cp blst/libblst.a lib/libblst.a
sudo cp libpc2.a lib/libpc2.a

if [ -L "/usr/lib/libblst.a" ]; then
    sudo rm -rf /usr/lib/libblst.a
fi

if [ -L "/usr/lib/libpc2.a" ]; then
    sudo rm -rf /usr/lib/libpc2.a
fi

sudo ln -s $(pwd)/lib/libblst.a /usr/lib/libblst.a
sudo ln -s $(pwd)/lib/libpc2.a /usr/lib/libpc2.a

sudo ldconfig
