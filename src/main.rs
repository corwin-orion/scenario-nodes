use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

struct Scenario {
    scenes: HashMap<Instant, String>,
    clues: Vec<Clue>
}

struct Clue {
    id: Instant,
    source_id: Instant,
    target_id: Instant, 
}

impl Clue {
    pub fn get_source(self) -> Instant {
        self.source_id
    }

    pub fn set_source(mut self, source_id: Instant) {
        self.source_id = source_id
    }
    
    pub fn get_target(self) -> Instant {
        self.target_id
    }

    pub fn set_target(mut self, target_id: Instant) {
        self.target_id = target_id
    }
}

fn main() {
    // Open file browser to create or open .txt
    // Parse JSON to Scenario - Error -> prompt again
    // CLI options:
        // View Clue List
        // View Revelation List
        // Add/Remove Scene
        // Add/Remove Clue

    println!("Hello, world!");
}
