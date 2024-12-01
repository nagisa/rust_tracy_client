use rand::rngs::ThreadRng;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use tracy_client::{non_continuous_frame, secondary_frame_mark};

pub fn main() {
    tracy_client::Client::start();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        simulate_physics(&mut rng);
        if rng.gen_bool(0.75) {
            simulate_rendering(&mut rng);
        }

        // This marks the boundary between two continuous frames
        secondary_frame_mark!("Update Loop");
    }
}

fn simulate_physics(rng: &mut ThreadRng) {
    // This is a discontinuous frame; it has a defined start and stop
    // In this case, the start is now - and the end is when _frame is dropped
    let _frame = non_continuous_frame!("Physics");

    // simulate doing some work
    sleep(Duration::from_millis(rng.gen_range(10..20)));
}

fn simulate_rendering(rng: &mut ThreadRng) {
    // This is a discontinuous frame; it has a defined start and stop
    // In this case, the start is now - and the end is when _frame is dropped
    let _frame = non_continuous_frame!("Rendering");

    // simulate doing some work
    sleep(Duration::from_millis(rng.gen_range(10..30)));
}
