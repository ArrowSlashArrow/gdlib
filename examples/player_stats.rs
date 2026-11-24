//! Reads the player's profile from CCGameManager.dat and prints a summary.
//!
//! Showcased items:
//! - load the game manager savefile with `CCGameManager::from_local`
//! - read player info, settings and statistics
//! - check achievement progress with `GDAchievement`
//!
//! This example deliberately never prints `save.account`, which can contain passwords.
//!
//! Run with `cargo run --example player_stats`.

use gdlib::{
    ccgamemanager::{CCGameManager, achievements::GDAchievement},
    core::{GDError, get_ccgamemanager_path},
};

fn main() -> Result<(), GDError> {
    if get_ccgamemanager_path().is_none() {
        println!("couldn't find CCGameManager.dat on this machine");
        return Ok(());
    }

    let save = CCGameManager::from_local()?;

    let player = &save.player_info;
    println!("player:      {}", player.username);
    println!(
        "icons:       cube {}, ship {}, ball {}, wave {}",
        player.icon_cube, player.icon_ship, player.icon_ball, player.icon_wave
    );
    println!("game opened: {} times", save.stats.bootups);
    println!("demon keys:  {}", save.stats.demon_keys);
    println!(
        "music/sfx:   {:.0}% / {:.0}%",
        save.config.bgm_volume * 100.0,
        save.config.sfx_volume * 100.0
    );

    println!(
        "\nofficial levels with progress: {}",
        save.stats.official_level_progresses.len()
    );
    for level in &save.stats.official_level_progresses {
        println!(
            "  {:<24} {:>3}%",
            level.identity.name, level.player_stats.normal_percentage
        );
    }

    // Achievement progress is a number: for most achievements, 100 means unlocked.
    let unlocked = GDAchievement::ALL
        .iter()
        .filter(|a| save.stats.get_vanilla_achievement(**a) >= 100)
        .count();
    println!(
        "\nachievements: {unlocked} / {} unlocked",
        GDAchievement::ALL.len()
    );
    for a in [
        GDAchievement::Attempt01,
        GDAchievement::Jump01,
        GDAchievement::Demon01,
    ] {
        println!(
            "  {:<28} progress {}",
            a.to_ident(),
            save.stats.get_vanilla_achievement(a)
        );
    }

    Ok(())
}
