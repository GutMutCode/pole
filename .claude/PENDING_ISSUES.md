# Pending Issues & Future Tasks

> **중요:** LLM은 매일 작업 시작 시 이 파일을 확인해야 합니다.

**Last Updated:** 2025-10-21

---

## 🔴 P0 Issues (Blocking)

None currently.

---

## 🟡 P1 Issues (Important, Non-blocking)

### 1. Rust Type Checker - Missing Builtin Functions

**Status:** Scheduled for Day 5 (Friday, 2025-10-25)  
**Impact:** Type checking fails for player.pole-ir, zombie.pole-ir  
**Workaround:** Python fallback works, automation not broken

**Problem:**
Rust type checker doesn't recognize builtin functions like `list_get`, `int_to_float`, etc.

**Solution:**
Add builtin function signatures to `compiler/src/type_checker.rs`:

```rust
fn initialize_builtins(&mut self) {
    // List operations
    self.add_builtin("list_get", ...);
    self.add_builtin("list_set", ...);
    self.add_builtin("list_push", ...);
    
    // Type conversions
    self.add_builtin("int_to_float", ...);
    self.add_builtin("float_to_int", ...);
    
    // IO
    self.add_extern("print", ...);
}
```

**Test:**
```bash
pole test games/zomboid/specs/player.pole-ir
pole test games/zomboid/specs/zombie.pole-ir
pole test examples/01-factorial.pole-ir
```

**Expected Result:**
All type checks should pass without errors.

**Related Files:**
- `compiler/src/type_checker.rs` - Add builtins here
- `docs/WEEK1_PLAN.md` - Day 5 schedule
- `CLAUDE.md` - Pending task reference

---

## 🟢 P2 Issues (Optional)

### 2. Python Type Checker Deprecation

**Status:** Consider for Week 2+  
**Impact:** Low - Rust type checker now primary

**Action:** Mark Python type checker as deprecated, add warning message.

---

## 📝 How to Use This File

### For LLM:
1. Check this file when user says "진행해줘" or "continue"
2. If today's date matches a scheduled task, execute it
3. Update status when completed
4. Move completed tasks to archive section

### For Human:
1. Add new issues with priority (P0/P1/P2)
2. Set target date for P1/P2 issues
3. Review weekly

---

## ✅ Completed Issues (Archive)

### Week 1 Day 2 (2025-10-21)
- ✅ Python parser limitations (record literals)
  - **Solution:** Migrated to Rust parser with Python fallback
  - **Commit:** d4cab5e

