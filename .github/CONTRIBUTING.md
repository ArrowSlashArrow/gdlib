# Contributing
Thanks for wanting to contribute! All contribution must be made through a pull request. Please see the respective section below for your type of contribution.

## Bug report
- Include a short title, preferably one sentence long, that is informative of the core issue.
- In the description, state exactly what went wrong to the best of your ability.
- If applicable, please provide the program output. 
- Please ensure that your code passes `cargo test` and that `cargo clippy` emits no warnings or errors.

## Feature request
- Title the PR: "Feature Request: \<your feature>"
- Please provide:
    - a brief summary of the feature
    - what you would like to see from it as a result
    - any necessary details of implementation

## Other
- Describe your suggestion briefly and informatively in the title
- Provide as much detail as you want in the description, but please provide enough so that the maintainer(s) understand the core of the request.

# Adding code to the crate
When adding new code to the crate, please follow these guidelines to ensure the robustness of your code:
1. Ensure that your methods are documented with as much documentation as is necessary. 
    - It is vital for anyone reading your code to understand what it does, no matter how convoluted. This doesn't mean that every single line of code must be thoroughly explained, but just that reading the attributed documentation is enough for someone to understand what your code does and in what situations it works.
2. Write tests to cover all foreseeable edge cases.
    - GDLib is meant to be robust and should not buckle when a single struct fails to parse some esoteric value, causing its caller to fail too.
    - Write enough tests to ensure that your new code in its current state will not fail on any input unless it is explicitly designed to do so.
    - Under **NO CIRCUMSTANCES** should you ever `panic!`

## Documentation for new modules
Every module doc starts with one sentence saying what the module is for. After that, add only the sections that have something useful to say:

1. **How to use it:** the usual workflow and its entry points, with links.
2. **Choosing between things:** a table or list when a module has many items or several ways to do something.
3. **Format:** how the data looks on disk, when the module reads or writes GD data. (where applicable)
4. **Common Pitfalls:** behaviour that would surprise someone, such as panics, silent no-ops and lost data.
5. **References:** external sources for the format.

Some common module archetypes:
| Module type | Expected docs | Examples |
|---|---|---|
| **Entry point** | Everything relevant, plus a tested example | `gdlevel`, `gdobj`, `ccgamemanager` |
| **Domain module** | Purpose, format or selection guide, gotchas | `leveldata`, `meta`, `structs`, `io` |
| **Small or lookup module** | A few lines of description, with an example if it's quick | `crypto`, `consts`, `defaults` |

**Conventions:**
- Write examples as doctests. Use `no_run` for anything touching files or savefiles, so tests never read or overwrite a real save.
- Say "module", not "file", since rustdoc shows modules.
- Link to items instead of naming them in plain text, so renames break the docs build instead of leaving stale text.
- Do not hardcode counts of things that can change (for example, the list of nullary functions in `triggers.rs`). 
