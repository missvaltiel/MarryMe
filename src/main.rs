mod event;
mod loader;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let events = loader::load_events("events.json");
    let start_time = Instant::now();
    let (security_tx, security_rx) = mpsc::channel::<event::Event>(32);
    let (catering_tx, catering_rx) = mpsc::channel::<event::Event>(32);
    let (waiters_tx,  waiters_rx)  = mpsc::channel::<event::Event>(32);


    println!("=== Wedding Sim — Phase 1: Data Layer ===\n");
    println!("Loaded {} valid events:\n", events.len());

    for event in &events {
        let elapsed_before = start_time.elapsed().as_secs_f64();
        let sleep_for = event.timestamp - elapsed_before;

        if sleep_for > 0.0 {
            tokio::time::sleep(Duration::from_secs_f64(sleep_for)).await;
        }

        let elapsed_seconds = start_time.elapsed().as_secs_f64();
        println!("[t={:.1}s] [COORDINATOR] RECEIVED: event #{} ({}) priority={} team={}", elapsed_seconds, event.id, event.event_type, event.priority, event.event_type.team());
    }



    println!("\n=== Simulation complete ===");
}
