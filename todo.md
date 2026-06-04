- improve tests
- fix doc examples
- make a level header string parser
- add features: 
    - `shorthand`: shorthand constructors
        - on by default
- add proper benchmarks
- compartmentalization of crate via feature toggles
- all trigger constructors
- add all gd obj property types
    - then remove unknown property type
- have full api coverage of both savefiles

## cclocallevels
- llm03 parser for lists
    - list objects themselves
- gdlevel
    - optimize level struct
        - better property storage
        - its basically a hashmap so it shouldnt be slow\
    - implement sha-256 hashing (feature: "hashing")
- objects
    - constructors for all of the following:
        - triggers
        - gameplay objects
        - saws
        - other objects which have intrinsic properties
    - cover all gd obj properties
    - implement sha-256 hashing (feature: "hashing")

## ccgamemanager
- everything
- tbh idk what's in there because i have not explored that file

## server (feature: "api")
- all known request formats
- utils stuff todo with formatting request inputs (e.g. GJP)
- [useful thing](https://wyliemaster.github.io/gddocs/#/)