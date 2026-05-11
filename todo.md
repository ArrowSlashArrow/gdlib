- improve tests
- fix doc examples
- optimize gdlevel
    - better property storage
    - its basically a hashmap so it shouldnt be slow
- make a level header string parser
- add features: 
    - `shorthand`: shorthand constructors
        - on by default
- add proper benchmarks
- put object map inside of lookup.rs
- hashing of gd objects and levels
- compartmentalization of crate via feature toggles
    - and also proper modules
- all trigger constructors
- add all gd obj property types
    - the remove unknown property type
- interface with server (feature)
- ccgamemanger
    - end goal: have full api coverage of both savefiles

## cclocallevels
- llm03 parser for lists
    - list objects themselves
- levels
    - level header string parser
        - within that, the colour string parser
- objects
    - constructors for all of the following:
        - triggers
        - gameplay objects
        - saws
        - other objects which have intrinsic properties
    - cover all gd obj properties

## ccgamemanager
- everything
- tbh idk what's in there because i have not explored that file

## server
- all known request formats
- utils stuff todo with formatting request inputs (e.g. GJP)
- [useful thing](https://wyliemaster.github.io/gddocs/#/)