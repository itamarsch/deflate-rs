hyperfine --warmup 3 --runs 10 './target/release/deflate gcc-14.2.0.tar.gz' 'gunzip -fk ./gcc-14.2.0.tar.gz'
rm gcc-14.2.0.tar