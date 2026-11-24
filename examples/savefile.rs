//! Reads your own GD savefile (CCLocalLevels.dat) and, if you ask it to, imports a level into it.
//!
//! Showcased items:
//! - load every local level and list with `CCLocalLevels::from_local`
//! - read levels without decompressing their object data
//! - add a level to the savefile and write it back with a backup
//!
//! Run with `cargo run --example savefile` to only list your levels.
//! Run with `cargo run --example savefile -- --import path/to/level.gmd` to add a level.
//! **Close GD before importing**, or it may overwrite the savefile when it next saves.

use gdlib::{
    cclocallevels::gdlevel::{CCLocalLevels, GDLevel},
    core::{GDError, get_cclocallevels_path},
};

fn main() -> Result<(), GDError> {
    match get_cclocallevels_path() {
        Some(path) => println!("savefile: {}", path.display()),
        None => {
            println!("couldn't find a GD savefile on this machine");
            return Ok(());
        }
    }

    let mut save = CCLocalLevels::from_local()?;
    println!(
        "{} levels and {} lists (game version {:?})\n",
        save.levels.len(),
        save.lists.len(),
        save.binary_version
    );

    // Levels are stored newest first. Level data stays compressed, so this is fast even for big saves.
    for level in save.levels.iter().take(15) {
        println!(
            "  {:<30} attempts: {:>5}  edit time: {:>5} min",
            level.identity.name,
            level.player_stats.attempts,
            level.meta.seconds_editing / 60,
        );
    }
    if save.levels.len() > 15 {
        println!("  ... and {} more", save.levels.len() - 15);
    }

    for list in &save.lists {
        println!("  [list] {} ({} levels)", list.name, list.level_ids.len());
    }

    // Only write when explicitly asked to.
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--import") {
        let Some(path) = args.get(i + 1) else {
            println!("usage: --import <path to .gmd>");
            return Ok(());
        };
        let level = GDLevel::from_gmd(path)?;
        println!("\nimporting '{}'", level.identity.name);

        save.add_level(level);
        // Keeps a copy of the old savefile next to the new one.
        save.export_to_savefile_with_backup()?;
        println!("done. the level is at the top of your created levels.");
    }

    Ok(())
}
