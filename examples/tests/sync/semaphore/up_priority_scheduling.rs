#![no_main]
#![no_std]

extern crate alloc;
use hopter::{boot::main, config, debug::semihosting, hprintln, sync::Semaphore, task};

static SEMAPHORE: Semaphore = Semaphore::new(1, 1);

#[main]
fn main(_: cortex_m::Peripherals) {
    // Create test tasks.
    task::build()
        .set_entry(low_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY + 1)
        .spawn()
        .unwrap();
    task::build()
        .set_entry(high_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY - 1)
        .spawn()
        .unwrap();
    task::build()
        .set_entry(middle_task)
        .set_priority(config::DEFAULT_TASK_PRIORITY)
        .spawn()
        .unwrap();

    // Let the test tasks run. But they will be blocked by the semaphore.
    task::change_current_priority(config::DEFAULT_TASK_PRIORITY + 2).unwrap();

    // Notify through the semaphore and the test tasks should be woken up based
    // on their respective priority.
    for _ in 0..3 {
        SEMAPHORE.down();
    }

    semihosting::terminate(true);
}

fn high_task() {
    SEMAPHORE.up();
    hprintln!("High priority task executed");
}

fn middle_task() {
    SEMAPHORE.up();
    hprintln!("Middle priority task executed");
}

fn low_task() {
    SEMAPHORE.up();
    hprintln!("Low priority task executed");
}
