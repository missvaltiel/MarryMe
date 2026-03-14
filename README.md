# Wedding Sim 🎪

An event-driven wedding incident coordinator built in Rust with Tokio.

## Build & Run

```bash
# Prerequisites: Rust installed (https://rustup.rs)
cargo build
cargo run
```

The binary expects `events.json` in the working directory (project root).

## Project Structure

```
src/
  main.rs      — entry point
  event.rs     — Event, EventType, Priority types
  loader.rs    — JSON parsing with validation/skip logic
events.json    — sample input
```

## Implementation Phases

- [x] **Phase 1** — Data layer: Event model, JSON loading, validation
- [x] **Phase 2** — Simulation clock: sleep until each event's timestamp, print RECEIVED
- [x] **Phase 3** — Routing: 3 team channels, coordinator dispatches by event_type
- [ ] **Phase 4** — Worker pool: 2 fixed workers per team, queue backpressure
- [ ] **Phase 5** — Expiration: deadline check when worker pulls from queue
- [ ] **Phase 6** — Metrics: Arc<Mutex<Metrics>> for received/handled/expired/stress
- [ ] **Phase 7** — Simulation boundary: 60s limit, graceful shutdown, final summary

## Team → Event Type Mapping

| Team     | Handles                    |
|----------|----------------------------|
| Security | brawl, not_on_list         |
| Catering | bad_food, feeling_ill      |
| Waiters  | dirty_table, broken_item   |

## Priority Deadlines

| Priority | Deadline |
|----------|----------|
| High     | 5 sec    |
| Medium   | 10 sec   |
| Low      | 15 sec   |
