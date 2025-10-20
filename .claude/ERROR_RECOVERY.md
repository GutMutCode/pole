# Autonomous Error Recovery

> LLM must autonomously resolve errors without user intervention

**Source:** CLAUDE.md  
**Updated:** 2025-10-21

---

## 🔧 Error Recovery Protocols

### Error Type 1: Dependency Missing

**Detection:**
```bash
ls games/zomboid/specs/player.pole
# Error: file not found
```

**Autonomous Resolution:**
1. **Analyze:** Day N depends on Day N-1
2. **Decide:** Switch to Day N-1 task first
3. **Update TODO:** Add Day N-1 tasks to beginning of list
4. **Execute:** Complete Day N-1
5. **Resume:** Return to Day N tasks

**Example:**
```
[TodoWrite] Add urgent tasks:
- Write player.pole (Day 1 - URGENT)
- Generate player.pole-ir (Day 1 - URGENT)
- Continue with zombie.pole (Day 2 - P0)
```

---

### Error Type 2: Syntax Error

**Detection:**
```bash
pole check file.pole
# Error: Invalid type definition at line 10
```

**Autonomous Resolution:**
1. **Read error message** - Identify exact problem
2. **Re-read examples** - Study correct syntax
3. **Compare** - Find difference between example and your code
4. **Fix** - Edit file with correct syntax
5. **Retry** - Run pole check again (max 3 attempts)
6. **Escalate** - If 3 failures, ask user for guidance

**Example:**
```
Error: type Position = {...}  (wrong)
Example: type Position: fields: ... (correct)

[Edit] Fix syntax error
[Bash] pole check file.pole
✅ Success - continue
```

---

### Error Type 3: IR Generation Failed

**Detection:**
```bash
pole build file.pole
# Error: LLM generated invalid IR / Type check failed
```

**Autonomous Resolution:**
1. **Attempt 1:** Improve LLM prompt with more examples
2. **Attempt 2:** Use --mock flag for template
3. **Attempt 3:** Write IR manually following examples/08-simple-record.pole-ir
4. **Verify:** Run pole check on generated IR
5. **Success:** Continue to next step

**Example:**
```
Attempt 1: pole build file.pole
❌ Failed

Attempt 2: [Read] examples/08-simple-record.pole-ir
[Write] file.pole-ir (manual)
[Bash] pole check file.pole-ir
✅ Success - continue
```

---

### Error Type 4: Rust Parser Error

**Detection:**
```bash
cargo run --release --bin polec -- ../file.pole-ir
# Error: Invalid match expression syntax
```

**Autonomous Resolution:**
1. **Read error line number**
2. **Re-read IR examples** - Study correct match syntax
3. **Fix syntax** - Edit file.pole-ir
4. **Retry verification** (max 3 attempts)

---

### Error Type 5: Test Failures

**Detection:**
```bash
pole test file.pole-ir
# ✗ test_create_player failed
```

**Autonomous Resolution:**
1. **Analyze test failure** - Read expected vs actual
2. **Fix logic** - Edit function implementation
3. **Retry test**
4. **If persistent:** Review spec for correctness

---

## Priority Re-evaluation Rules

### When to change priority:

**1. Blocking Dependency Missing (P0 → URGENT)**
```
Current: Day 2 zombie.pole (P0)
Blocker: Day 1 player.pole not found
Action: Switch to Day 1 player.pole (URGENT)
```

**2. Critical Bug in Previous Work (P1 → P0)**
```
Current: Day 3 rendering (P1)
Issue: Day 2 zombie.pole has type error
Action: Fix Day 2 first, then resume Day 3
```

**3. Tool/Compiler Broken (Any → URGENT)**
```
Current: Any task
Issue: pole CLI not working
Action: Fix tool first, then resume
```

---

## Autonomous Decision Framework

```
FOR each error:
  1. Classify error type (1-5 above)
  2. Select resolution strategy
  3. Execute fix attempts (max 3)
  4. IF successful: Continue workflow
  5. ELSE IF max attempts: 
     - Update TODO with "blocked by X"
     - Switch to alternative task OR ask user
  6. Log resolution in TODO notes
```

---

## Complete Example

```
Step 5: pole check zombie.pole
❌ Error: Invalid type definition at line 15

LLM Autonomous Action:
1. [Read] Error message - "type ZombieState ="
2. [Read] examples/03-user-validation.pole - Study syntax
3. [Compare] My code vs example
4. [Edit] Change "type ZombieState =" to comment
5. [Bash] pole check zombie.pole
6. ✅ Success - continue to Step 6

Total time: 30 seconds
User intervention: 0
```

---

## Related Documents

- [CLAUDE.md](../CLAUDE.md) - Main guide
- [PRIORITY_ADJUSTMENT.md](../docs/guides/PRIORITY_ADJUSTMENT.md) - Full guide
- [DEVELOPMENT_CHECKLIST.md](DEVELOPMENT_CHECKLIST.md) - Checklist
