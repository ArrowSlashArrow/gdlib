//! Builds a counter-based mechanic and checks its logic in Rust before ever opening GD.
//!
//! The level counts how many checkpoints the player passes. Each checkpoint adds 1 to counter 1,
//! and once the counter reaches 3 a door (group 5) opens.
//!
//! Shows how to:
//! - create item edit and item compare triggers
//! - display an item in-game with a counter label
//! - simulate `ItemEditTrigger::eval_result` and `ItemCompareTrigger::eval_self` to test logic offline
//!
//! Run with `cargo run --example item_logic`.

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            CounterLabel, GDObject, ItemCompareTrigger, ItemEditTrigger, MoveTrigger, SpawnTrigger,
            TextObject,
            ids::objects,
            meta::GDObjConfig,
            structs::{
                CompareOp, CompareOperand, DefaultMove, Easing, Item, ItemAlign, MoveMode, Op,
                RoundMode, SignMode,
            },
        },
    },
    core::GDError,
};

const HITS: Item = Item::Counter(1);
const CHECK_GROUP: i16 = 3;
const OPEN_DOOR_GROUP: i16 = 4;
const DOOR: i16 = 5;

fn add_one() -> ItemEditTrigger {
    ItemEditTrigger {
        operand1: None,
        operand2: None,
        target: HITS,
        modifier: 1.0,
        assign_op: Op::Add,
        multiply_mod: true,
        id_op: Op::Add,
        id_rounding: RoundMode::None,
        result_rounding: RoundMode::None,
        id_sign: SignMode::None,
        result_sign: SignMode::None,
    }
}

fn check_hits() -> ItemCompareTrigger {
    ItemCompareTrigger {
        true_id: OPEN_DOOR_GROUP,
        false_id: 0, // do nothing when false
        lhs: CompareOperand::from(HITS),
        rhs: CompareOperand::number_literal(3.0),
        compare_op: CompareOp::GreaterOrEquals,
        tolerance: 0.0,
    }
}

fn main() -> Result<(), GDError> {
    // -- Simulate the logic first --
    let (edit, compare) = (add_one(), check_hits());
    let mut hits = 0.0;
    for checkpoint in 1..=4 {
        // Both operands are `None`, so only the current value of the target matters.
        hits = edit.eval_result(0.0, 0.0, hits);
        // The right-hand side is a number literal: item 0 times the modifier. Item 0 reads as 1,
        // so pass 1.0 for its raw value and the operand evaluates to the literal itself.
        let spawned = compare.eval_self(hits, 1.0);
        println!("checkpoint {checkpoint}: counter = {hits}, compare spawns group {spawned}");
    }
    assert_eq!(
        compare.eval_self(2.0, 1.0),
        0,
        "door must stay shut at 2 hits"
    );
    assert_eq!(
        compare.eval_self(3.0, 1.0),
        OPEN_DOOR_GROUP,
        "door must open at 3 hits"
    );

    // -- Then build it --
    let mut level = GDLevel::new_latest_version();
    level.identity.name = "gdlib item logic example".into();
    let mut objs = Vec::new();

    // A label in the corner that shows the counter.
    objs.push(GDObject::from_config(
        &GDObjConfig::new().with_pos(75.0, 255.0),
        CounterLabel {
            item: HITS,
            align: ItemAlign::Left,
            seconds_only: false,
        },
    ));

    // Three checkpoints. Each one adds one to the counter and then runs the check.
    for i in 0..3 {
        let x = 300.0 + i as f32 * 150.0;
        objs.push(GDObject::from_config(
            &GDObjConfig::new().with_pos(x, 75.0),
            TextObject {
                text: format!("checkpoint {}", i + 1),
                kerning: 0,
            },
        ));
        // Triggers placed in the level fire when the player passes their x position.
        objs.push(GDObject::from_config(
            &GDObjConfig::new().with_pos(x, 345.0),
            add_one(),
        ));
        objs.push(GDObject::from_config(
            &GDObjConfig::new().with_pos(x, 375.0),
            SpawnTrigger {
                spawn_id: CHECK_GROUP,
                ..Default::default()
            },
        ));
    }

    // The compare trigger only runs when spawned.
    let spawned = |group: i16| {
        GDObjConfig::new()
            .with_pos(-100.0, 300.0)
            .with_groups([group])
            .with_spawnable(true)
            .with_multitrigger(true)
    };
    objs.push(GDObject::from_config(&spawned(CHECK_GROUP), check_hits()));

    // Opening the door moves it up out of the way.
    objs.push(GDObject::from_config(
        &spawned(OPEN_DOOR_GROUP).translate(30.0, 0.0),
        MoveTrigger {
            move_config: MoveMode::Default(DefaultMove {
                dx: 0.0,
                dy: 150.0,
                x_lock: None,
                y_lock: None,
            }),
            time: 1.0,
            target_group: DOOR,
            silent: false,
            dynamic: false,
            easing: Easing::none(),
        },
    ));

    // The door: a column of blocks in group 5.
    for row in 0..4 {
        objs.push(GDObject::new(
            objects::BLACK_GRADIENT_SQUARE,
            &GDObjConfig::new()
                .with_pos(915.0, 15.0 + row as f32 * 30.0)
                .with_groups([DOOR]),
            vec![],
        ));
    }

    level.add_objects(objs);
    level.export_to_gmd("test_gmds/generated_example_item_logic.gmd")?;
    println!("Wrote test_gmds/generated_example_item_logic.gmd");
    Ok(())
}
