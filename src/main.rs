mod event;
mod loader;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Semaphore;


#[tokio::main]
async fn main() {
    // ── Load and validate events from JSON ────────────────────────────────────
    // loader::load_events reads events.json, skips unknown event types,
    // and returns a Vec<Event> sorted by timestamp.
    let events = loader::load_events("events.json");
    let start_time = Instant::now();

    println!("=== Wedding Sim ===\n");
    println!("Loaded {} valid events\n", events.len());

    // ── Create one channel per team ───────────────────────────────────────────
    // mpsc::channel returns a (Sender, Receiver) pair.
    // The coordinator holds the tx (sender) side and sends events into it.
    // Each team task owns its rx (receiver) side and reads events out of it.
    // The buffer size (32) is how many events can queue up before the sender waits.
    let (security_tx, mut security_rx) = mpsc::channel::<event::Event>(32);
    let (catering_tx, mut catering_rx) = mpsc::channel::<event::Event>(32);
    let (waiters_tx,  mut waiters_rx)  = mpsc::channel::<event::Event>(32);

    // ── Spawn one task per team ───────────────────────────────────────────────
    // tokio::spawn launches each task concurrently alongside the coordinator loop.
    // `async move` transfers ownership of the rx into the task so it can run
    // independently. When all tx senders are dropped, recv() returns None
    // and the while loop exits cleanly.
    let security_handle = tokio::spawn(async move {
        while let Some(event) = security_rx.recv().await {
            println!("[SECURITY] DISPATCHED event #{}", event.id);
        }
    });

    let catering_handle = tokio::spawn(async move {
        while let Some(event) = catering_rx.recv().await {
            println!("[CATERING] DISPATCHED event #{}", event.id);
        }
    });

    let waiters_handle = tokio::spawn(async move {
        while let Some(event) = waiters_rx.recv().await {
            println!("[WAITERS] DISPATCHED event #{}", event.id);
        }
    });

    // ── Coordinator loop ──────────────────────────────────────────────────────
    // Iterates through events in timestamp order. For each event:
    //   1. Sleeps until the event's scheduled arrival time
    //   2. Prints RECEIVED
    //   3. Routes the event to the correct team channel and prints DISPATCHED
    for event in &events {
        // Compute how much longer to wait before this event is due.
        // We subtract already-elapsed time so events stay on schedule
        // even if previous iterations took a small amount of processing time.
        let elapsed_before = start_time.elapsed().as_secs_f64();
        let sleep_for = event.timestamp - elapsed_before;

        if sleep_for > 0.0 {
            tokio::time::sleep(Duration::from_secs_f64(sleep_for)).await;
        }

        // Capture elapsed AFTER the sleep so the printed timestamp is accurate.
        let elapsed_seconds = start_time.elapsed().as_secs_f64();
        println!(
            "[t={:.1}s] [COORDINATOR] RECEIVED: event #{} ({}) priority={} team={}",
            elapsed_seconds, event.id, event.event_type, event.priority, event.event_type.team()
        );

        // Route to the correct team based on event_type.
        // We borrow the matching tx and send a clone of the event into it.
        // .clone() is needed because we only have a reference (&event) from the loop.
        let tx = match event.event_type.team() {
            "Security" => &security_tx,
            "Catering" => &catering_tx,
            _          => &waiters_tx,
        };
        tx.send(event.clone()).await.unwrap();
        println!(
            "[COORDINATOR] DISPATCHED event #{} -> {}",
            event.id, event.event_type.team()
        );
    }

    // ── Shut down channels ────────────────────────────────────────────────────
    // Explicitly dropping the senders signals to each team task that no more
    // events are coming. This causes their recv() to return None and their
    // while loops to exit. Without this, the tasks would wait forever.
    drop(security_tx);
    drop(catering_tx);
    drop(waiters_tx);

    // ── Wait for all team tasks to finish ─────────────────────────────────────
    // Without these awaits, main() could reach "Simulation complete" and exit
    // before the team tasks have finished printing their DISPATCHED lines.
    let _ = security_handle.await;
    let _ = catering_handle.await;
    let _ = waiters_handle.await;

    println!("\n=== Simulation complete ===");
}
