//! Reads a level list from a .gmdl file and creates a new one.
//!
//! Showcased items:
//! - reading a list with `GDList::from_gmdl`
//! - building a list from scratch, keeping `level_ids` and `levels` in sync
//! - exporting it with `GDList::export_to_gmdl`
//!
//! Run with `cargo run --example lists`.

use gdlib::{
    cclocallevels::{
        gdlevel::{
            GDLevel,
            enums::{GDListDifficulty, GDListType},
        },
        gdlist::GDList,
    },
    core::GDError,
};

fn main() -> Result<(), GDError> {
    // -- Reading --
    let list = GDList::from_gmdl("test_gmds/Summer Vacation.gmdl")?;
    println!("{} by {}", list.name, list.creator);
    println!("{}", list.description);
    println!(
        "difficulty: {:?}, {} levels:",
        list.difficulty,
        list.level_ids.len()
    );
    // `levels` holds short summaries of each level, not the full levels.
    for (id, level) in list.level_ids.iter().zip(&list.levels) {
        println!("  #{id:<10} {}", level.identity.name);
    }

    // -- Creating --
    let picks = [
        (128, "1st level"),
        (4284013, "2nd level"),
        (44062068, "3rd level"),
    ];

    let levels = picks
        .iter()
        .map(|(id, name)| {
            let mut level = GDLevel::default();
            level.identity.id = *id;
            level.identity.name = name.to_string();
            level
        })
        .collect();

    let my_list = GDList {
        name: "My favourites".into(),
        description: "Built with gdlib".into(),
        creator: "gdlib".into(),
        difficulty: GDListDifficulty::Normal,
        list_type: GDListType::Local,
        version: 1,
        level_ids: picks.iter().map(|(id, _)| *id).collect(),
        levels,
        ..Default::default()
    };

    my_list.export_to_gmdl("test_gmds/generated_example_list.gmdl")?;
    println!("\nWrote test_gmds/generated_example_list.gmdl");
    Ok(())
}
