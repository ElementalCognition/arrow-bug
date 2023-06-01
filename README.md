# concat_batches panics with total_len <= bit_len assertion
Reproduces [arrow-rs](https://github.com/apache/arrow-rs) issue [#4324](https://github.com/apache/arrow-rs/issues/4324).
Run `cargo test`
