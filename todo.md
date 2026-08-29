- add proper benchmarks
- have full api coverage of both savefiles
- big massive documentation update
    - cover all enums and struct fields
    - also cover all the known edgecases/anomalies

fns todo
- `ItemEditTrigger::eval_result(item1_value, item2_value, attempts, points, maintime)`
- `CompareOperand::eval_operand(item_value)`
- `ItemCompareTrigger::eval_comparison(item1_value, item2_value)`

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
