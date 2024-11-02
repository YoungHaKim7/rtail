use notify::PollWatcher;

#[test]
fn poll_watcher_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<PollWatcher>();
}
