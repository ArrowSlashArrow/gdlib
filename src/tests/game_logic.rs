//! Tests for code that reproduces in-game behaviour: item arithmetic, comparisons, RNG and leaderboard seeds.
use std::f32::consts::PI;

use crate::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            GDObject, ObjectProperties,
            structs::*,
            triggers::{AdvancedRandomTrigger, ItemCompareTrigger, ItemEditTrigger},
        },
    },
    core::rand::*,
};

// ---------------- operators ----------------

#[test]
fn op_eval() {
    assert_eq!(Op::Set.eval(5.0, 3.0), 3.0);
    assert_eq!(Op::Add.eval(5.0, 3.0), 8.0);
    assert_eq!(Op::Sub.eval(5.0, 3.0), 2.0);
    assert_eq!(Op::Mul.eval(5.0, 3.0), 15.0);
    assert_eq!(Op::Div.eval(6.0, 3.0), 2.0);
    // documented: division by zero yields zero
    assert_eq!(Op::Div.eval(6.0, 0.0), 0.0);
    assert_eq!(Op::Div.eval(6.0, -0.0), 0.0);
}

#[test]
fn round_mode_eval() {
    assert_eq!(RoundMode::None.eval(2.5), 2.5);
    assert_eq!(RoundMode::Nearest.eval(2.4), 2.0);
    assert_eq!(RoundMode::Nearest.eval(2.5), 3.0);
    assert_eq!(RoundMode::Nearest.eval(-2.5), -3.0);
    assert_eq!(RoundMode::Floor.eval(-2.5), -3.0);
    assert_eq!(RoundMode::Ceiling.eval(-2.5), -2.0);
}

#[test]
fn sign_mode_eval() {
    assert_eq!(SignMode::None.eval(-2.0), -2.0);
    assert_eq!(SignMode::Absolute.eval(-2.0), 2.0);
    assert_eq!(SignMode::Absolute.eval(2.0), 2.0);
    assert_eq!(SignMode::Negative.eval(2.0), -2.0);
    assert_eq!(SignMode::Negative.eval(-2.0), -2.0);
}

#[test]
fn round_3dp() {
    use crate::cclocallevels::gdobj::structs::round_3dp;
    assert_eq!(round_3dp(1.23456), 1.234);
    assert_eq!(round_3dp(-1.23456), -1.234);
    assert_eq!(round_3dp(2.0), 2.0);
    assert_eq!(round_3dp(0.0009), 0.0);
    // documented: NaN returns 0
    assert_eq!(round_3dp(f64::NAN), 0.0);
}

#[test]
fn compare_operand_eval() {
    let op = CompareOperand {
        operand_item: Item::Counter(1),
        modifier: 2.0,
        mod_op: Op::Mul,
        rounding: RoundMode::None,
        sign: SignMode::None,
    };
    assert_eq!(op.eval_operand(5.0), 10.0);

    // `Op::Set` is treated as multiplication
    let set = CompareOperand {
        mod_op: Op::Set,
        ..op
    };
    assert_eq!(set.eval_operand(5.0), 10.0);

    let add = CompareOperand {
        mod_op: Op::Add,
        ..op
    };
    assert_eq!(add.eval_operand(5.0), 7.0);

    // the modifier is truncated to 3 decimal places
    let precise = CompareOperand {
        modifier: 1.23456,
        ..op
    };
    assert!((precise.eval_operand(1.0) - 1.234).abs() < 1e-9);

    assert_eq!(CompareOperand::number_literal(7.5).eval_operand(1.0), 7.5);
    assert_eq!(CompareOperand::from(Item::Timer(2)).eval_operand(3.0), 3.0);
}

// ---------------- item compare ----------------

fn compare_trigger(compare_op: CompareOp, tolerance: f32) -> ItemCompareTrigger {
    ItemCompareTrigger {
        true_id: 1,
        false_id: 2,
        lhs: CompareOperand::from(Item::Counter(1)),
        rhs: CompareOperand::from(Item::Counter(2)),
        compare_op,
        tolerance,
    }
}

#[test]
fn item_compare_operators() {
    let cases = [
        (CompareOp::Equals, 3.0, 3.0, 1),
        (CompareOp::Equals, 3.0, 4.0, 2),
        (CompareOp::NotEquals, 3.0, 4.0, 1),
        (CompareOp::NotEquals, 3.0, 3.0, 2),
        (CompareOp::Greater, 4.0, 3.0, 1),
        (CompareOp::Greater, 3.0, 3.0, 2),
        (CompareOp::Greater, 2.0, 3.0, 2),
        (CompareOp::GreaterOrEquals, 3.0, 3.0, 1),
        (CompareOp::GreaterOrEquals, 2.0, 3.0, 2),
        (CompareOp::Less, 2.0, 3.0, 1),
        (CompareOp::Less, 3.0, 3.0, 2),
        (CompareOp::Less, 4.0, 3.0, 2),
        (CompareOp::LessOrEquals, 3.0, 3.0, 1),
        (CompareOp::LessOrEquals, 4.0, 3.0, 2),
    ];
    for (op, lhs, rhs, expected) in cases {
        assert_eq!(
            compare_trigger(op, 0.0).eval_self(lhs, rhs),
            expected,
            "{lhs} {op:?} {rhs}"
        );
    }
}

#[test]
fn item_compare_tolerance() {
    assert_eq!(
        compare_trigger(CompareOp::Equals, 0.5).eval_self(3.0, 3.4),
        1
    );
    assert_eq!(
        compare_trigger(CompareOp::Equals, 0.5).eval_self(3.0, 3.6),
        2
    );
    assert_eq!(
        compare_trigger(CompareOp::NotEquals, 0.5).eval_self(3.0, 3.4),
        2
    );
    assert_eq!(
        compare_trigger(CompareOp::NotEquals, 0.5).eval_self(3.0, 3.6),
        1
    );
}

#[test]
fn item_compare_with_modifiers() {
    let t = ItemCompareTrigger {
        true_id: 11,
        false_id: 22,
        lhs: CompareOperand {
            operand_item: Item::Counter(1),
            modifier: 1.0,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        },
        rhs: CompareOperand {
            operand_item: Item::Counter(0),
            modifier: 2.0,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        },
        compare_op: CompareOp::LessOrEquals,
        tolerance: 0.0,
    };
    // lhs <= rhs * 2
    assert_eq!(t.eval_self(4.0, 2.0), 11);
    assert_eq!(t.eval_self(5.0, 2.0), 22);
}

// ---------------- item edit ----------------

fn item_edit() -> ItemEditTrigger {
    ItemEditTrigger {
        operand1: None,
        operand2: None,
        target: Item::Counter(1),
        modifier: 1.0,
        assign_op: Op::Set,
        multiply_mod: true,
        id_op: Op::Add,
        id_rounding: RoundMode::None,
        result_rounding: RoundMode::None,
        id_sign: SignMode::None,
        result_sign: SignMode::None,
    }
}

#[test]
fn item_edit_without_operands_uses_modifier() {
    let t = ItemEditTrigger {
        modifier: 7.0,
        ..item_edit()
    };
    assert_eq!(t.eval_result(100.0, 200.0, 300.0), 7.0);
}

#[test]
fn item_edit_single_operand() {
    let t = ItemEditTrigger {
        operand1: Some(Item::Counter(2)),
        modifier: 2.0,
        ..item_edit()
    };
    assert_eq!(t.eval_result(5.0, 999.0, 0.0), 10.0);

    let t = ItemEditTrigger {
        operand2: Some(Item::Counter(2)),
        modifier: 2.0,
        multiply_mod: false,
        ..item_edit()
    };
    assert_eq!(t.eval_result(999.0, 5.0, 0.0), 2.5);
}

#[test]
fn item_edit_two_operands() {
    for (id_op, expected) in [
        (Op::Add, 8.0),
        (Op::Sub, 2.0),
        (Op::Mul, 15.0),
        (Op::Div, 5.0 / 3.0),
    ] {
        let t = ItemEditTrigger {
            operand1: Some(Item::Counter(2)),
            operand2: Some(Item::Counter(3)),
            id_op,
            ..item_edit()
        };
        assert_eq!(t.eval_result(5.0, 3.0, 0.0), expected, "{id_op:?}");
    }
}

#[test]
fn item_edit_assign_operator_order() {
    // documented step 4: `current target item value <op> operand result`
    for (assign_op, expected) in [
        (Op::Set, 10.0),
        (Op::Add, 110.0),
        (Op::Sub, 90.0),
        (Op::Mul, 1000.0),
        (Op::Div, 10.0),
    ] {
        let t = ItemEditTrigger {
            operand1: Some(Item::Counter(2)),
            assign_op,
            ..item_edit()
        };
        assert_eq!(t.eval_result(10.0, 0.0, 100.0), expected, "{assign_op:?}");
    }
}

#[test]
fn item_edit_rounding_and_sign() {
    // step 3: id rounding then id sign
    let t = ItemEditTrigger {
        operand1: Some(Item::Counter(2)),
        id_rounding: RoundMode::Floor,
        id_sign: SignMode::Negative,
        ..item_edit()
    };
    assert_eq!(t.eval_result(2.7, 0.0, 0.0), -2.0);
}

#[test]
fn item_edit_result_rounding_applies_with_set() {
    // documented step 5 happens regardless of the assignment operator
    let t = ItemEditTrigger {
        operand1: Some(Item::Counter(2)),
        result_rounding: RoundMode::Floor,
        result_sign: SignMode::Absolute,
        ..item_edit()
    };
    assert_eq!(t.eval_result(-2.5, 0.0, 0.0), 3.0);
}

#[test]
fn item_edit_result_rounding_with_assign_operator() {
    let t = ItemEditTrigger {
        operand1: Some(Item::Counter(2)),
        assign_op: Op::Add,
        result_rounding: RoundMode::Ceiling,
        ..item_edit()
    };
    assert_eq!(t.eval_result(0.25, 0.0, 1.0), 2.0);
}

// ---------------- RNG ----------------

#[test]
fn lcg() {
    assert_eq!(next_seed(0), 2_531_011);
    assert_eq!(next_seed(1), 214_013 + 2_531_011);
    let mut seed = 0;
    next_seed_mut(&mut seed);
    assert_eq!(seed, next_seed(0));
    next_seed_mut(&mut seed);
    assert_eq!(seed, next_seed(next_seed(0)));
    // must wrap instead of overflowing
    let _ = next_seed(u64::MAX);
}

#[test]
fn fast_rand() {
    assert_eq!(fast_rand_bits(0), (2_531_011 >> 16) & 0x7fff);
    for seed in [0, 1, 12345, u64::MAX] {
        assert!(fast_rand_bits(seed) <= 0x7fff);
        let n = fast_rand_bits_norm(seed);
        assert!((0.0..=1.0).contains(&n), "{n}");
    }
}

#[test]
fn random_trigger_chance() {
    for seed in 0..1000 {
        assert!(check_seed_random(seed, 1.01));
        assert!(!check_seed_random(seed, 0.0));
    }
}

#[test]
fn advanced_random_empty_and_single() {
    assert_eq!(check_seed_advanced_random(5, &vec![]), None);
    for seed in 0..100 {
        assert_eq!(check_seed_advanced_random(seed, &vec![(7, 1)]), Some(7));
    }
}

#[test]
fn advanced_random_only_picks_listed_groups() {
    let probabilities = vec![(1, 10), (2, 20), (3, 30)];
    for seed in 0..10_000 {
        let g = check_seed_advanced_random(seed, &probabilities).unwrap();
        assert!([1, 2, 3].contains(&g));
    }
}

#[test]
fn advanced_random_zero_chances_do_not_panic() {
    for seed in 0..100 {
        let _ = check_seed_advanced_random(seed, &vec![(1, 0), (2, 0)]);
    }
}

#[test]
fn advanced_random_negative_chances_do_not_panic() {
    for seed in 0..100 {
        let _ = check_seed_advanced_random(seed, &vec![(1, -10), (2, 5)]);
    }
}

#[test]
fn advanced_random_huge_chances_do_not_panic() {
    for seed in 0..100 {
        let _ = check_seed_advanced_random(seed, &vec![(1, i32::MAX), (2, i32::MAX)]);
    }
}

#[test]
fn advanced_random_predict() {
    // tuple: (object, seed, expected group)
    let tests: &[(&str, u64, i16)] = &[
        (
            "1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,2.50.3.50;",
            5006,
            2,
        ),
        ("1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,2.50.3.50;", 1, 2),
        (
            "1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,1.10.2.10.3.10.4.10.5.10;",
            32321,
            2,
        ),
        (
            "1,2068,2,75,3,75,155,1,11,1,87,1,36,1,152,1.10.2.20.3.15.4.5.5.25.6.50;",
            58392,
            6,
        ),
        ("1,2068,2,285,3,285,155,1,36,1,152,2.10.3.10;", 190824, 2),
    ];

    for &(obj_str, seed, expected) in tests {
        assert_eq!(
            AdvancedRandomTrigger::from_object(&GDObject::parse_str(obj_str))
                .unwrap()
                .determine_spawn_from_seed(seed)
                .unwrap(),
            expected
        );
    }
}

// ---------------- leaderboard seeds ----------------

#[test]
fn level_seed_classic() {
    let mut level = GDLevel::default();
    level.player_stats.jumps = 10;
    level.player_stats.normal_percentage = 50;
    let seconds = 30;
    let expected =
        1482 + (10 + 3991) * (50 * 8354) + (seconds + 4085) * (seconds + 4085) - 50028039;
    assert_eq!(level.generate_level_seed_classic(true, seconds), expected);
    assert_eq!(
        level.generate_level_seed_classic(false, seconds),
        expected - 1482
    );
}

#[test]
fn level_seed_classic_large_values_do_not_overflow() {
    // a completed level with a lot of jumps overflows an i32. GD uses wrapping 32-bit arithmetic.
    let mut level = GDLevel::default();
    level.player_stats.jumps = 50_000;
    level.player_stats.normal_percentage = 100;
    let seconds = 100_000;
    let expected = 1482i32
        .wrapping_add((50_000i32 + 3991).wrapping_mul(100 * 8354))
        .wrapping_add((seconds + 4085i32).wrapping_mul(seconds + 4085))
        .wrapping_sub(50028039);
    assert_eq!(level.generate_level_seed_classic(true, seconds), expected);
}

#[test]
fn level_seed_platformer_is_in_range() {
    let mut level = GDLevel::default();
    for (time, points) in [(0, None), (12_345, Some(17)), (i32::MAX / 2, Some(-50))] {
        level.player_stats.best_attempt_time = time;
        level.player_stats.best_points = points;
        let seed = level.generate_level_seed_platformer();
        assert!((0..77849).contains(&seed), "{seed}");
    }
}

#[test]
fn level_seed_platformer_negative_time_does_not_panic() {
    let mut level = GDLevel::default();
    level.player_stats.best_attempt_time = i32::MIN;
    level.player_stats.best_points = Some(i32::MIN);
    let _ = level.generate_level_seed_platformer();
}

// ---------------- rotation ----------------

#[test]
fn rotation_from_degrees() {
    assert_eq!(
        RotationNormal::from_degrees(450.0),
        RotationNormal {
            degrees: 90.0,
            x360: 1
        }
    );
    assert_eq!(
        RotationNormal::from_degrees(-450.0),
        RotationNormal {
            degrees: -90.0,
            x360: -1
        }
    );
    assert_eq!(
        RotationNormal::from_degrees(45.0),
        RotationNormal {
            degrees: 45.0,
            x360: 0
        }
    );
    assert_eq!(
        RotationNormal::from_degrees(720.0),
        RotationNormal {
            degrees: 0.0,
            x360: 2
        }
    );
}

#[test]
fn rotation_from_radians() {
    let r = RotationNormal::from_radians(PI * 3.0);
    assert_eq!(r.x360, 1);
    assert!((r.degrees - 180.0).abs() < 1e-3, "{r:?}");
}

#[test]
fn collider_config() {
    let c = ColliderConfig::two_colliders(1, 2);
    assert_eq!((c.collider1, c.collider2), (1, 2));
    assert!(!c.collide_player1 && !c.collide_player2 && !c.collide_both_players);
}
