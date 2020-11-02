# Tests failing

The tests fail with `there is no reactor running, must be called from the context of Tokio runtime`. Can you help me?

```
tveronezi:help_sample_runtime_block_on$ cargo build
   Compiling help_sample_runtime_block_on v0.1.0 (/home/tveronezi/Documents/sources/personal/help_sample_runtime_block_on)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
tveronezi:help_sample_runtime_block_on$ cargo test
   Compiling help_sample_runtime_block_on v0.1.0 (/home/tveronezi/Documents/sources/personal/help_sample_runtime_block_on)
    Finished test [unoptimized + debuginfo] target(s) in 7.29s
     Running target/debug/deps/help_sample_runtime_block_on-3182e40652415bae

running 1 test
test tests::test_get_containers ... FAILED

failures:

---- tests::test_get_containers stdout ----
thread 'tests::test_get_containers' panicked at 'there is no reactor running, must be called from the context of Tokio runtime', /home/tveronezi/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.22/src/io/driver/mod.rs:202:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_get_containers

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
tveronezi:help_sample_runtime_block_on$ 
```

This is the code that panics:

```
Runtime::new().unwrap().block_on(async {
    let result = super::docker::get_containers().await;
    assert_eq!(format!("{}", container_id), result.unwrap());
});
```

Using the following didn't do the trick either:

```
let mut builder = Builder::new_multi_thread();
builder.enable_all().thread_name("my-test");
let runtime = builder.build().expect("Unable to build tokio runtime");
runtime.block_on(async {
    let result = super::docker::get_containers().await;
    assert_eq!(format!("{}", container_id), result.unwrap());
});
``` 
