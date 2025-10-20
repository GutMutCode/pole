# 타입 체커 문제 처리 전략 요약

## 📌 핵심 결정

**오늘 오후 (2-3시간) 완료:**
1. ✅ Variant constructors 지원 (South, North 등)
2. ✅ Record literal type inference (간단한 방법)

**미루기 (Week 2+):**
3. Let expression edge cases
4. Full bidirectional type checking

---

## 🎯 이유

### 왜 지금 해야 하나?
- ✅ player.pole-ir, zombie.pole-ir 타입 체크 차단 중
- ✅ LLM 자동화 프로세스 완성도 향상
- ✅ Week 1 데모 완성에 필수

### 왜 안전한가?
- ✅ Python fallback 유지 (실패해도 빌드 계속)
- ✅ 점진적 커밋 (한 번에 하나씩)
- ✅ 기존 테스트로 regression 확인

### 자동화 프로세스 영향?
**변화 없음 - 오히려 개선:**
```
[Before] LLM → ❌ Rust 에러 → ⚠️ Python fallback → 실행
[After]  LLM → ✅ Rust 통과 → 🚀 네이티브 실행
```

---

## 📋 구현 계획

### Phase 1: Variant Constructors (1-2h)
```rust
// compiler/src/type_checker.rs의 collect_type_definitions()에 추가

if let TypeDefKind::Variant(variants) = &type_def.definition {
    for (variant_name, param_types) in variants {
        if param_types.is_empty() {
            // South -> Direction 형태로 등록
            self.function_types.insert(variant_name, ...);
        }
    }
}
```

### Phase 2: Record Type Inference (1-2h)
```rust
// check_function()에서 expected type 전달
let body_type = self.infer_type_with_hint(&func_def.body, Some(&func_def.return_type));

// Record literal을 만나면 expected type과 비교
Expr::Record(record) => {
    if let Some(expected) = expected_type {
        if self.record_matches(record, expected) {
            return expected;  // ✅
        }
    }
    // fallback
}
```

---

## ✅ 성공 기준

**필수:**
- [ ] `pole test games/zomboid/specs/player.pole-ir` 통과
- [ ] `pole test games/zomboid/specs/zombie.pole-ir` 통과
- [ ] `pole test examples/01-factorial.pole-ir` 여전히 통과

**권장:**
- [ ] 모든 variant/record 예제 통과

---

## 📚 문서

- `.claude/TYPE_CHECKER_STRATEGY.md` - 상세 전략
- `.claude/DECISION.md` - 결정 근거
- `.claude/PENDING_ISSUES.md` - 업데이트 예정

---

**Ready to execute:** 
Human이 "진행해줘" 하면 Phase 1부터 시작
