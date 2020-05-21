use job_scheduler::{Job, JobScheduler};
use notify_rust::Notification;
use std::time::Duration;

fn notify() {
    Notification::new()
        .summary("Olá!")
        .body("Beba Água")
        .appname("Drink Water")
        .icon("information")
        .show()
        .unwrap();
}

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("* 35 * * * *".parse().unwrap(), || {
        notify();
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
