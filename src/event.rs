use serde::Deserialize;
use std::fmt;

// ── Priority ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    /// How many seconds from arrival until this event expires.
    pub fn deadline_secs(&self) -> f64 {
        match self {
            Priority::High   =>  5.0,
            Priority::Medium => 10.0,
            Priority::Low    => 15.0,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Priority::High   => "high",
            Priority::Medium => "medium",
            Priority::Low    => "low",
        };
        write!(f, "{}", s)
    }
}

// ── EventType ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Security
    Brawl,
    NotOnList,
    // Catering
    BadFood,
    FeelingIll,
    // Waiters
    DirtyTable,
    BrokenItem,
}

impl EventType {
    pub fn team(&self) -> &'static str {
        match self {
            EventType::Brawl | EventType::NotOnList         => "Security",
            EventType::BadFood | EventType::FeelingIll      => "Catering",
            EventType::DirtyTable | EventType::BrokenItem   => "Waiters",
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EventType::Brawl       => "brawl",
            EventType::NotOnList   => "not_on_list",
            EventType::BadFood     => "bad_food",
            EventType::FeelingIll  => "feeling_ill",
            EventType::DirtyTable  => "dirty_table",
            EventType::BrokenItem  => "broken_item",
        };
        write!(f, "{}", s)
    }
}

// ── Event ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    pub id: u32,
    pub event_type: EventType,
    pub priority: Priority,
    pub description: String,
    pub timestamp: f64, // seconds since simulation start
}

impl Event {
    /// Absolute deadline = arrival timestamp + priority window
    pub fn deadline(&self) -> f64 {
        self.timestamp + self.priority.deadline_secs()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Event #{}] type={} priority={} team={} deadline={:.1}s | \"{}\"",
            self.id,
            self.event_type,
            self.priority,
            self.event_type.team(),
            self.deadline(),
            self.description
        )
    }
}
