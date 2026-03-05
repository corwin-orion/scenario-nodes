use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ================================== TYPES ===================================

#[derive(Serialize, Deserialize)]
struct Scenario {
    name: String,
    description: String,

    scenes: HashMap<SceneId, Scene>,
    clues: HashMap<ClueId, Clue>,

    next_scene_id: u64,
    next_clue_id: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SceneId(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClueId(u64);

#[derive(Serialize, Deserialize)]
struct Scene {
    name: String,
    description: String,
    completed: bool,

    outgoing: HashSet<ClueId>, // Clues found within the Scene
    incoming: HashSet<ClueId>, // Clues that lead to the Scene
}

#[derive(Serialize, Deserialize)]
struct Clue {
    source: Option<SceneId>,
    target: Option<SceneId>,
    revealed: bool,
}

// ============================= IMPLEMENTATIONS ==============================

impl Scenario {
    pub fn generate_scene_id(&mut self) -> SceneId {
        let id = SceneId(self.next_scene_id);
        self.next_scene_id += 1;
        id
    }

    pub fn generate_clue_id(&mut self) -> ClueId {
        let id = ClueId(self.next_clue_id);
        self.next_clue_id += 1;
        id
    }
}

// =================================== RUN ====================================

fn run() {}
