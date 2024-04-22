use std::sync::atomic::{AtomicU64, Ordering}; // Importing AtomicU64 and Ordering types from the atomic module in the Rust standard binary 

static COUNTER: AtomicU64 = AtomicU64::new(0); // Defines a new global variable called counter that has an initial value of 0. Static means the variable will have a fixed memory location 

#[ic_cdk::query] 
fn counter() -> u64 {
    COUNTER.load(Ordering::Relaxed) //  The Ordering::Relaxed argument specifies the memory ordering for this atomic operation. In this case, Relaxed means that there are no synchronization or ordering guarantees beyond atomicity.
}

#[ic_cdk::init] // Canister runs when this function is initialized 
fn init(timer_interval_secs: u64) {
    let interval = std::time::Duration::from_secs(timer_interval_secs); 
    ic_cdk::println!("Starting periodic task with interval {interval:?}"); 
    ic_cdk_timers::set_timer_interval(interval, || { // Creates a new periodic timer with the specified interval and a closure to call 
        COUNTER.fetch_add(1, Ordering::Relaxed); // Increases the global counter everytime the periodic task is triggered 
    }); 
}

#[ic_cdk::post_upgrade] 
fn post_upgrade(timer_interval_secs: u64) {
    init(timer_interval_secs)
}

// Export Candid interface
ic_cdk::export_candid!();