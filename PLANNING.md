# PLANNING.md

## Current State (2026-04-30)

Working terminal TUI with three screens: menu → typing → results.

**What works:**
- Language selection: English prose, Rust snippets, Python snippets (10 each)
- Live WPM, accuracy, elapsed time updated every 100 ms tick
- Per-character color feedback (grey/green/red) with cursor highlight
- Progress gauge
- Backspace corrects errors and adjusts error count
- Panic hook restores terminal on crash
- Results screen: WPM, accuracy, time, error count

**Known gaps / rough edges:**
- Snippet pool is small (10 per language) — repeats quickly
- No persistent score history; results vanish on retry
- Snippet length is fixed by pool content; no short/medium/long filter
- No word-by-word mode — only full snippet at once
- No test suite

## Next Obvious Milestone

**Score history + larger snippet pool**

Goals:
1. Persist sessions to `~/.local/share/rhonetyping/scores.json` (or equivalent via `dirs` crate)
2. Add a History screen accessible from the menu showing last N results (WPM, accuracy, mode, date)
3. Expand snippet pools to ≥30 entries per language, or load from bundled text files to keep `snippets.rs` manageable

Acceptance criteria:
- After finishing a session the score is written to disk
- Launching the app with existing history shows it on the History screen
- No two consecutive sessions show the same snippet
