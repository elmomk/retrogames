---
name: check-rust
description: Run cargo check on all Miyoo Rust ports and fix any compile errors
user_invocable: true
---

Run `cargo check` on all Miyoo Rust game ports and report results.

```bash
cd /home/mo/data/Documents/git/retrogames && ./scripts/check-rust.sh
```

If any game fails, read the error output, identify the issue, and fix it using Edit on `miyoo/<game>/src/main.rs`. Then re-run the script to verify.

Common error patterns:
- **E0499 (borrow checker)**: Use index-based loops instead of `iter_mut()` when calling `self.method()` in loop body
- **E0689 (float ambiguity)**: Add `: f32` type annotation to `rand::gen_range()` results
- **E0004 (non-exhaustive match)**: Add new enum variants to all `match` blocks
