# Pending Issues & Future Tasks

> **중요:** LLM은 매일 작업 시작 시 이 파일을 확인해야 합니다.

**Last Updated:** 2025-10-21

---

## 🔴 P0 Issues (Blocking)

None currently.

---

## 🟡 P1 Issues (Important, Non-blocking)

None currently.

---

## 🟢 P2 Issues (Optional)

### 1. Let Expression Edge Cases

**Status:** Week 2+  
**Impact:** Low - rare occurrences  
**Workaround:** Python fallback handles these cases

**Problem:**
Occasionally "Undefined variable 'let'" error in specific contexts.

**Action:** Deep parser/type checker debugging when time permits.

---

### 2. Function Argument Record Literals

**Status:** Week 2+  
**Impact:** Low - workaround available  

**Problem:**
```pole-ir
func test(p: Point) -> Int: ...
test({ x: 1, y: 2 })  // Type error
```

**Workaround:**
```pole-ir
let p = { x: 1, y: 2 } in
test(p)  // Works
```

**Action:** Extend expected type hints to function arguments.

---

### 3. Python Type Checker Deprecation

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

### Week 1 Day 5 (2025-10-21)
- ✅ **Rust Type Checker - Variant Constructors**
  - **Problem:** South, North, etc. not recognized
  - **Solution:** Added inline variant parsing + registered as type env values
  - **Commit:** 9d363c8
  
- ✅ **Rust Type Checker - Record Literal Type Inference**
  - **Problem:** `{ health: 100 }` not recognized as `Player` type
  - **Solution:** Expected type hints + recursive inference + structural typing
  - **Commit:** 9d363c8

- ✅ **Rust Type Checker - Builtin Functions**
  - **Problem:** list_get, int_to_float not recognized
  - **Solution:** Added curried builtin function signatures
  - **Commit:** 10cc4a5

- ✅ **IR Parser - Record Literal Syntax**
  - **Problem:** Parser expected `=` instead of `:`
  - **Solution:** Fixed to match IR spec (`:` for field bindings)
  - **Commit:** 10cc4a5

### Week 1 Day 2 (2025-10-21)
- ✅ Python parser limitations (record literals)
  - **Solution:** Migrated to Rust parser with Python fallback
  - **Commit:** d4cab5e

