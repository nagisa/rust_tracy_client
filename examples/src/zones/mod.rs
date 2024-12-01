use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use std::thread::sleep;
use std::time::Duration;
use tracy_client::{frame_mark, span, Client};

pub fn main() {
    Client::start();
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        simulate_physics(&mut rng);
        simulate_rendering(&mut rng);

        // simulate doing some work
        sleep(Duration::from_millis(rng.gen_range(1..50)));

        // This marks the boundary between two continuous frames
        frame_mark();
    }
}

fn simulate_physics(rng: &mut ThreadRng) {
    // This zone starts immediately, and ends when zone is dropped
    let zone = span!("Physics");
    // Zones can have custom colours!
    zone.emit_color(0xFF0000);

    for name in ["Cow", "Pig", "Player", "Robot"] {
        // Let's imagine these names are dynamic
        // To mark zones for them, we need to use a different method which temporarily allocates a zone location
        let zone = Client::running().unwrap().span_alloc(
            Some(name),
            "perform_physics",
            "physics.rs",
            123,
            0,
        );

        zone.emit_value(rng.next_u64()); // entity ID? Who knows!

        // simulate doing some work
        sleep(Duration::from_millis(rng.gen_range(5..20)));

        if rng.gen_bool(0.15) {
            let zone = span!("Collision");
            // Zones can have arbitrary text!
            zone.emit_text("Collided against a wall");

            // simulate doing some work
            sleep(Duration::from_millis(rng.gen_range(5..20)));
        }
    }

    // simulate doing some work
    sleep(Duration::from_millis(rng.gen_range(1..20)));
}

fn simulate_rendering(rng: &mut ThreadRng) {
    // This zone starts immediately, and ends when zone is dropped
    let zone = span!("Rendering");
    // Zones can have custom colours!
    zone.emit_color(0x00FF00);

    for _ in 0..rng.gen_range(1..10) {
        if rng.gen_bool(0.50) {
            let zone = span!("Mesh");
            zone.emit_color(rng.gen_range(0x000000..0xFFFFFF));
            // simulate doing some work
            sleep(Duration::from_millis(rng.gen_range(1..15)));
        } else {
            // Sometimes let's not mark it, just to show that zones don't have to next to eachother

            // simulate doing some work
            sleep(Duration::from_millis(rng.gen_range(1..15)));
        }
    }
}
