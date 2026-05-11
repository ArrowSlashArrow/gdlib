use gdlib::{
    cclocallevels::{gdlevel::Level, gdobj::structs::Group},
    core::GDError,
};

fn main() -> Result<(), GDError> {
    // Load level from .gmd file
    let mut level = Level::from_gmd("test_gmds/level.gmd")?;

    // Get level data, which is None only if it hasn't been initialized.
    if let Some(data) = level.get_decrypted_data_ref() {
        // Add group 42 to all objects
        for obj in data.objects.iter_mut() {
            obj.config.add_group(Group::Regular(42));
        }
    }

    // Export level
    level.export_to_gmd("test_gmds/generated_group_42.gmd")?;
    Ok(())
}
