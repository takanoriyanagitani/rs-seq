#!/bin/sh

maxint=167772160

bench_seq(){
	\time -l seq $maxint |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_native(){
	\time -l ./rs-seq $maxint |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_wazero(){
	\time -l wazero run ./rs-seq.wasm $maxint |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

#bench_seq
bench_native
#bench_wazero
