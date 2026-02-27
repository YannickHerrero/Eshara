//! One-shot utility to export the hardcoded story tree to data/story.json.
//!
//! Run with: cargo run --bin export-story

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use eshara::story::nodes::build_story_tree;
use eshara::story::{EndingInfo, EndingType, StoryData};

fn build_endings() -> HashMap<String, EndingInfo> {
    let mut endings = HashMap::new();

    let all = vec![
        EndingType::NewDawn,
        EndingType::TheSignal,
        EndingType::Static,
        EndingType::GoneDark,
        EndingType::TheEsharaWins,
    ];

    for ending in all {
        let key = format!("{:?}", ending); // e.g. "NewDawn", "TheSignal"
        let title = eshara::story::endings::ending_title(&ending);
        let desc = eshara::story::endings::ending_description(&ending);
        endings.insert(
            key,
            EndingInfo {
                title,
                description: desc,
            },
        );
    }

    endings
}

fn main() {
    let nodes = build_story_tree();
    let endings = build_endings();

    let story_data = StoryData { nodes, endings };

    let json = serde_json::to_string_pretty(&story_data).expect("Failed to serialize story data");

    let out_path = Path::new("data/story.json");
    fs::create_dir_all(out_path.parent().unwrap()).expect("Failed to create data directory");
    fs::write(out_path, &json).expect("Failed to write story.json");

    println!("Exported story to {}", out_path.display());
    println!(
        "  {} nodes, {} endings, {} bytes",
        story_data.nodes.len(),
        story_data.endings.len(),
        json.len()
    );
}
