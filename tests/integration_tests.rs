// Some tests for the library code in http_server/src/lib.rs:

use http_server_example::ThreadPool;

#[test]
fn test_new() {
    let pool = ThreadPool::new(4);
    assert_eq!(pool.len(), 4);
}
