- add proper benchmarks
- have full api coverage of both savefiles
- big massive documentation update
    - cover all enums and struct fields
    - also cover all the known edgecases/anomalies

## cclocallevels
- gdlevel
    - implement sha-256 hashing (feature: "hashing")
    - add accessor method for GDLevel that allows you to access a property with its string key (e.g. "k50")
    - checkpoint gamesaves for plat levels
- objects
    - constructors for all of the following (when we get geode for 2.2091):
        - triggers + percentage trigger
        - checkpoint
        - collectibles
        - monsters
        - particle object
    - cover all gd obj properties
    - implement sha-256 hashing (feature: "hashing")

## ccgamemanager
- everything
- tbh idk what's in there because i have not explored that file


## api-payloads (feature: `api`)
- only for formatting request payloads and for parsing them

## api-client (feature: `api`)
- uses api-payloads feature but also includes functions to help with sending and receiving requests

## tests todo
- structs in `crate::cclocallevels::gdobj::constructors::triggers`
    - `from_object` against raw object strings
- big massive test overhaul
    - tests for parsing all gdvalues
    - tests for creating all gd structs
    - tests for this, that, and the other thing
    - everything in core thoroughly
