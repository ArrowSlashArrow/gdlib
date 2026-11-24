//! Loads a level and prints a report about it, without changing anything.
//!
//! Showcased items:
//! - read level metadata (name, creator, song, stats)
//! - count objects by name with `OBJECT_NAMES`
//! - find which groups are used, unused, or referenced by triggers
//! - turn raw objects back into typed structs with `ObjectProperties::from_object`
//!
//! Run with `cargo run --example inspect_level -- [path/to/level.gmd]`.
//! Defaults to `test_gmds/Chompstep.gmd`.

use std::collections::HashMap;

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{MoveTrigger, ObjectProperties, SpawnTrigger},
        properties::OBJECT_NAMES,
    },
    core::GDError,
};

fn main() -> Result<(), GDError> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "test_gmds/Chompstep.gmd".into());

    let level = if path.ends_with(".gdlvl") {
        GDLevel::from_gdlvl(&path)?
    } else {
        GDLevel::from_gmd(&path)?
    };

    println!("== {} ==", level.identity.name);
    println!("creator:      {}", level.identity.creator);
    if let Some(desc) = &level.identity.description {
        println!("description:  {desc}");
    }
    match (level.content.custom_song_id, level.content.official_song_id) {
        (Some(id), _) => println!("song:         custom song #{id}"),
        (None, Some(song)) => println!("song:         {song:?}"),
        _ => println!("song:         none"),
    }
    println!("length:       {:?}", level.content.length);
    println!(
        "attempts:     {} (best {}%)",
        level.player_stats.attempts, level.player_stats.normal_percentage
    );

    // `get_decrypted_data` returns a decompressed copy and leaves `level` as it is.
    let Some(data) = level.get_decrypted_data() else {
        println!("this level has no level data");
        return Ok(());
    };
    println!("objects:      {}", data.objects.len());

    // Count objects by ID, then show the ten most common ones by name.
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for obj in &data.objects {
        *counts.entry(obj.id).or_default() += 1;
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by_key(|(_, n)| std::cmp::Reverse(*n));

    println!("\nmost common objects:");
    for (id, n) in counts.iter().take(10) {
        let name = OBJECT_NAMES.get(id).copied().unwrap_or("unknown");
        println!("  {n:>6} x {name} (id {id})");
    }

    // Group usage. Handy before adding generated content to an existing level, so you don't
    // collide with groups the creator is already using.
    let used = data.get_used_groups();
    let args = data.get_argument_groups();
    let unused = data.get_unused_groups();
    println!("\ngroups on objects:        {}", used.len());
    println!("groups used by triggers:  {}", args.len());
    println!(
        "first free groups:        {:?}",
        &unused[..unused.len().min(5)]
    );

    // Convert some triggers back into typed structs to read their settings.
    let moves: Vec<MoveTrigger> = data
        .objects
        .iter()
        .filter_map(MoveTrigger::from_object)
        .collect();
    let spawns: Vec<SpawnTrigger> = data
        .objects
        .iter()
        .filter_map(SpawnTrigger::from_object)
        .collect();

    println!("\nmove triggers:  {}", moves.len());
    if let Some(longest) = moves.iter().max_by(|a, b| a.time.total_cmp(&b.time)) {
        println!(
            "  longest move: group {} over {}s",
            longest.target_group, longest.time
        );
    }
    println!("spawn triggers: {}", spawns.len());
    let delayed = spawns.iter().filter(|s| s.delay > 0.0).count();
    println!("  with a delay: {delayed}");

    Ok(())
}
