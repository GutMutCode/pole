# Development Checklist

> Complete development workflow with validation steps

**Source:** CLAUDE.md  
**Updated:** 2025-10-21

---

## 🔍 Mandatory TODO Template

When user says "진행해줘" or "개발 진행해", you MUST:

1. **Create TODO list** with TodoWrite tool:
   ```
   - Check dependencies (previous day's work)
   - Read syntax examples (03-user-validation.pole, 08-simple-record.pole-ir)
   - Test pole CLI tools
   - Write .pole specification
   - Validate with pole check
   - Generate IR with pole build (or write manually if LLM fails)
   - Verify with Rust parser
   - Run test cases
   - Commit only if all tests pass
   ```

2. **Mark each TODO as in_progress** before starting
3. **Mark as completed** immediately after finishing
4. **Never skip steps** - complete in order

---

## Before Writing Code

### 1. Check Dependencies
```bash
# If Day N task depends on Day N-1:
ls games/zomboid/specs/player.pole  # Does previous work exist?
```

### 2. Verify Syntax
```bash
# Read example files first
cat examples/03-user-validation.pole  # For .pole syntax
cat examples/08-simple-record.pole-ir  # For .pole-ir syntax
cat specs/syntax-v0.md  # For grammar rules
```

### 3. Test Tools
```bash
pole check examples/01-factorial.pole  # Test basic functionality
```

---

## While Writing Code

### 4. Follow Examples
- `.pole` files: Use `type Name:` with `fields:` (NOT `type Name = {...}`)
- `.pole-ir` files: Use `type Name = {...}` for records
- Enum types: Comment in `.pole`, implement in `.pole-ir`

### 5. Incremental Validation
```bash
pole check file.pole  # After writing .pole
pole build file.pole  # Generate .pole-ir
# If LLM fails: Check examples, improve prompt, or write manually
```

---

## After Writing Code

### 6. Multi-level Testing
```bash
# Level 1: Python parser (quick check)
pole check file.pole-ir

# Level 2: Rust parser (authoritative)
cd compiler && cargo run --release --bin polec -- ../file.pole-ir

# Level 3: Test cases
pole test file.pole-ir

# Level 4: Integration
./test_all_examples.py  # If adding to examples/
```

### 7. Commit Only If
- ✅ Rust parser validates successfully
- ✅ All test cases pass
- ✅ No TODO/FIXME comments without issue tracking

---

## Example Workflow

```
User: "진행해줘"

LLM: 
1. [TodoWrite] Create 9-step checklist
2. [TodoWrite] Mark step 1 as in_progress
3. [Bash] ls games/zomboid/specs/player.pole
4. [TodoWrite] Mark step 1 as completed ✅
5. [TodoWrite] Mark step 2 as in_progress
6. [Read] examples/03-user-validation.pole
7. [TodoWrite] Mark step 2 as completed ✅
... continues through all steps ...
8. [TodoWrite] All steps completed ✅
9. "✅ All checks passed. Safe to commit!"
```

---

## Quick Reference

| Step | Tool | Success Criteria |
|------|------|------------------|
| Dependencies | `ls` | File exists |
| Syntax | `read` | Examples understood |
| Tools | `bash` | Commands work |
| Write spec | `write` | File created |
| Validate | `pole check` | No errors |
| Generate IR | `pole build` | IR created |
| Verify | `cargo run` | Rust parser passes |
| Test | `pole test` | All tests pass |
| Commit | `git` | All criteria met |

---

## Related Documents

- [CLAUDE.md](../CLAUDE.md) - Main guide
- [ERROR_RECOVERY.md](ERROR_RECOVERY.md) - Error handling
- [AUTOMATION_GUIDE.md](../docs/guides/AUTOMATION_GUIDE.md) - Automation
