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
- big massive documentation update
    - cover all enums and struct fields
    - also cover all the known edgecases/anomalies

## cclocallevels
- gdlevel
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


## api-payloads
- only for formatting request payloads and for parsing them

## api-client
- uses api-payloads feature but also includes functions to help with sending and receiving requests