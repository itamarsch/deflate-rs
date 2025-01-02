cargo build --release

export PERF=/usr/lib/linux-tools/6.8.0-51-generic/perf 
/usr/lib/linux-tools/6.8.0-51-generic/perf record --call-graph dwarf target/release/deflate $1

/usr/lib/linux-tools/6.8.0-51-generic/perf script | ~/.cargo/bin/inferno-collapse-perf | ~/.cargo/bin/inferno-flamegraph >perf.svg
