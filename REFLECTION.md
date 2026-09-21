# REFLECTION — CA1

**1.** `src/scanner.rs:134`: `if self.peek() == '.' && self.peek_next().is_ascii_digit() {` — the fractional part exists only when a digit follows the dot. `5.` stops after the 5; the bare dot hits the catch-all at `src/scanner.rs:101`: `Character is not part of any token.` `.5` never reaches `number()`. 

**2.** The counter moves at `src/scanner.rs:100` and `:114` (newlines outside and inside strings). `run()` resets it at `:39` to the last real token's line before EOF, so trailing blank lines don't move it (section 6.1); no tokens: line 1.

**3.** Failed `tests/phase-1/valid/strings.kobo`. I thought a token's line was decided where its reading finished, so I incremented the counter after the closing quote — commit `636d864dea7f0aee196ea172ee5efc0cccb02e28`, `src/scanner.rs:57`: `self.line += 1;` after `self.advance();` — counting the quote as a line end, so every STRING reported one line low. Fixed in `FIX_COMMIT_HASH_HERE`, `src/scanner.rs:114`: increment moved into the newline loop, post-quote one deleted. 0/14 → 14/14.
