use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            GDObject,
            ids::{objects::TRIGGER_ADVANCED_RANDOM, properties::RANDOM_PROBABILITIES_LIST},
            structs::Group,
        },
    },
    core::rand::{check_seed_advanced_random, next_seed_mut},
};

fn main() {
    let level = GDLevel::from_gmd("test_gmds/Chompstep.gmd").unwrap();
    let mut objects = level.get_decrypted_data().unwrap().objects;

    // filter out all objects that are not advanced random triggers
    objects.retain(|o| o.id == TRIGGER_ADVANCED_RANDOM && o.config.pos.0 > 0.0);
    objects.sort_by(|a, b| a.config.pos.0.total_cmp(&b.config.pos.0));

    // the group we want spawned for each trigger
    // -1: any group is fine
    let expected = vec![
        2, -1, 2, -1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, -1, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    ];
    let mut seed = 0;

    // then try all of the seeds
    // cap it out at 25 seeds because searching all of them is impractically slow on a CPU
    while !crack_seed(seed, &expected[..25], &objects) {
        seed += 1;
        if seed % 1000000 == 0 {
            println!("checked seeds until {seed}");
        }
    }

    // the seed you get is guaranteed to get you to 79% on Chompstep
    println!("got seed: {seed}");
}

fn crack_seed(seed: u64, expected: &[i16], objects: &Vec<GDObject>) -> bool {
    let mut seed_clone = seed;

    for (idx, obj) in objects.iter().enumerate() {
        if idx == expected.len() {
            return true; // stop when we run out of expected groups to check
        }
        if expected[idx] == -1 {
            continue; // any group is fine (-1 is the sentinel value)
        }

        if !check_object(seed_clone, obj, expected[idx]) {
            return false;
        }
        // update seed (since adv rand triggers do it)
        next_seed_mut(&mut seed_clone);
    }

    true
}

fn check_object(seed: u64, object: &GDObject, expected_group: i16) -> bool {
    let probabilities = object.get_property(RANDOM_PROBABILITIES_LIST).unwrap();

    match check_seed_advanced_random(seed, &probabilities) {
        Some(Group::Regular(g)) => g == expected_group,
        _ => false, // this should never be hit since all of the triggers have set groups
    }
}
