use crate::event::Event;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

/// Raw deserialization target — event_type comes in as a plain string
/// so we can detect and log unknown values before building an Event.
#[derive(Debug, Deserialize)]
struct RawEvent {
    id: u32,
    event_type: String,
    priority: String,
    description: String,
    timestamp: f64,
}

/// Load events from a JSON file.
/// Unknown event_type values are logged and skipped (not fatal).
pub fn load_events(path: &str) -> Vec<Event> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Could not read {}: {}", path, e));

    let raw_values: Vec<Value> = serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("Invalid JSON in {}: {}", path, e));

    let mut events = Vec::new();

    for raw_value in raw_values {
        // Try full deserialisation (validates event_type and priority enums)
        match serde_json::from_value::<Event>(raw_value.clone()) {
            Ok(event) => {
                events.push(event);
            }
            Err(_) => {
                // Try to at least extract id and event_type for a useful log line
                let id = raw_value.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let et = raw_value
                    .get("event_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("<unknown>");
                eprintln!(
                    "[COORDINATOR] SKIP  event #{} — unknown or invalid event_type \"{}\"",
                    id, et
                );
            }
        }
    }

    // Sort by arrival time so the coordinator can process them in order
    events.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
    events
}
