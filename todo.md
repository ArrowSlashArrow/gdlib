- add proper benchmarks
- have full api coverage of both savefiles
- macos support for decoding/encdogin

## cclocallevels
- gdlevel
    - implement sha-256 hashing (feature: "hashing")
    - add accessor method for GDLevel that allows you to access a property with its string key (e.g. "k50")
    - checkpoint gamesaves for plat levels
- objects
    - constructors for all of the following:
        - triggers
        - collectibles
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