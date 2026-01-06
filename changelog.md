# Changelog

## Update 0.2.7
 * Added triggers: rotate, scale, follow y-pos, middleground config
 * Minor doc fixes
 * formatter patches + utility functions in GDObjConfig

## Update 0.2.6
 * Added error type
 * Removed all\* magic numbers from trigger constructor functions in triggers.rs
 * Added follow, animate, count, advanced random, ui config, pulse triggers
 * Optimised GD plist serialiser

## Update 0.2.5
 * Improved performance of the GDObject deserialiser
 * Added support for all universal object properties
 * Added object IDS

## Update 0.2.4
 * Minor patches/removals of unnecessary utils
 * miscellaneous other minor things

## Update 0.2.3
 * Added contrsuctors of triggers:
    * player control
    * gravity
    * end
    * full move trigger constructor
    * timewarp
    * camera zoom
    * camera guide
    * persistent item
    * item edit
    * spawn particle

## Update 0.2.2
 * Added constructors for miscellaneous triggers:
    * group reset trigger
    * random trigger
    * shake trigger
    * collision blocks
    * time triggers
    * show/hide player + player trail
    * bg/mg speed config
    * bg effect on/off
    * on death trigger
 * Patch
    * Fixed `.scale()` in `GDObject` changing position instead of scale
    * Implemented `Display` trait of `GDLevel`
    * Added functions for indexing unused or used groups + groups used as arguments

## Update 0.2.1
 * Added constructors for objects:
    * Transition objects
    * Reverse gameplay trigger
    * Link visible trigger
    * Counter object
    * Spawn trigger
    * Item edit trigger
 * Bugfixes:
    * Trailing char of object string no longer gets chopped off if it isn't a semicolon

## Update 0.2.0
 * Added constructors for some of the basic triggers:
    * Start pos
    * Colour trigger
    * Alpha trigger
    * Stop trigger
    * Toggle trigger

## Update 0.1.x
 * Ported over most of the GD IO from tasm-lang
 * Set up the module system