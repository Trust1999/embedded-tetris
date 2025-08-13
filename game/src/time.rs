pub trait Time {
    fn now_ms(&self) -> u64;
    fn delta_time_ms(&self) -> u64;
}
