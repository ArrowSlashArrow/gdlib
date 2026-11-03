# Unresolved issues

Codebase audit, 2026-10-19 (branch `dev`, HEAD `512c41f` plus uncommitted work in progress).

Line numbers are for the working tree when the audit was written. Some files were being edited during the audit, so the numbers may drift.

**Legend**
- **[verified]**: reproduced by running code against a clean checkout of HEAD.
- **[read]**: found by reading the code. Very likely real, but not executed.
- **[suspect]**: probably wrong, but depends on GD format details that I couldn't confirm.

## fixed
1. boolean flags are now encoded properly
2. pink groups are serialized
3. unknown binary versions not written as 0
4. all setter methods for `GDObjConfig` now work properly
5. `CounterLabel::from_object` is implemented correctly
6. `GDOrb::Blue` now has the correct ID
7. Pulse trigger is properly encoded for group pulse mode
8. HSV string checkboxes are parsed in the correct order
9. Duplicate properties are no longer possible 
10. Item edit trigger `eval_result` works properly
11. Guideline strings are read and encoded correctly
12. All object constructors use the correct types when serialising objects.
13. Methods do not panic where they shouldn't
14. Property 155 and 156 have been discovered
15. The linux file path for GD files has been fixed
16. Atomic file writes are used to prevent savefile corruption
17. Plist aho-corasick automaton is built only once
18. Keys others than LLM_01/2/3 are preserved in CCLocalLevels

---

## 5. Low: docs, naming and inconsistencies

- `GDStatistics.achievements` says "0 - 100" but values go up to 10,000.
- `.ok().unwrap()` / `try_from(..).ok().unwrap()` on infallible conversions. In total, `cargo clippy --lib` reported 46 warnings at HEAD.
- `GDLevelIntegrity.replay_data` is documented as gzip bytes but holds the raw string bytes.

## 6. Repo, CI and tests

- The CI workflow is named "Test dev branch" but only runs on `master`. `cargo clippy` runs without `-D warnings`, so lints never fail the build. The release workflow runs `cargo publish` without `--locked`, and `Cargo.lock` is gitignored.

## 7. other things
Types of properties need to be properly resolved - i.e. whether they hold an i16 or i32
test: read sample objects from level and check that a bunch of objects created in gd and in gdlib match