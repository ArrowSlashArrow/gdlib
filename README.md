# GDLib
A library to interface with GD savefiles built in rust.
 
## Capabilities
* Create and edit levels with full control
* Make objects with custom properties
* Read and write to Geometry Dash .dat savefiles
* Fast and lightweight

## Examples
Install this crate through `cargo add gdlib`

Example code:
```rs
use gdlib::gdlevel::{Level, Levels};
use gdlib::gdobj::{misc, triggers, GDObjConfig};

// Fetch levels from your CCLocalLevels.dat file
let mut levels = Levels::from_local().unwrap();

// Create a new Level object
let mut new_level = Level::new("New Level", "You", None, None);

// Add objects to new_level
new_level.add_object(triggers::triggers::move_trigger(
    GDObjConfig::default().pos(45.0, 45.0).groups([1234]), 
    triggers::MoveMode::Default(triggers::DefaultMove {
        dx: 45.0,
        dy: 54.0,
        x_lock: None,
        y_lock: None
    }), 
    0.50, 1, false, true, Some((MoveEasing::ElasticInOut, 1.50))
);
new_level.add_object(misc::default_block(GDObjConfig::default().pos(15.0, 15.0)));

// Add level to master Levels object and export back to savefile
levels.add_level(new_level);
levels.export_to_savefile_with_backup().unwrap();
```
