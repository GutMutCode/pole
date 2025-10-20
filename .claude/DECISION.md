# 타입 체커 문제 처리 결정

**Date:** 2025-10-21  
**Decision:** 남은 타입 체커 문제를 **오늘 오후 완료**

---

## TL;DR

```
✅ DO NOW (오늘 오후, 2-3시간):
  1. Variant constructors 지원
  2. Record literal type inference (간단한 방법)
  
❌ DEFER (Week 2+):
  3. Let expression edge cases
  4. Full bidirectional type checking
```

---

## 근거

### 1. Week 1 목표와의 정렬
- **목표:** 1분 플레이 가능한 데모
- **현재 차단 요소:** player.pole-ir, zombie.pole-ir 타입 체크 실패
- **해결 시:** LLM 생성 코드가 즉시 실행 가능

### 2. 자동화 프로세스 영향
**Before 수정:**
```
LLM 생성 → ❌ 타입 에러 → Python fallback → ⚠️ 경고 → 실행
```

**After 수정:**
```
LLM 생성 → ✅ 타입 통과 → Rust 컴파일 → 실행
```

**영향:** 긍정적 - 더 빠르고 안정적

### 3. 기술적 복잡도
| 작업 | 복잡도 | 시간 | 가치 |
|------|--------|------|------|
| Variant constructors | 중간 | 1-2h | 높음 ⭐⭐⭐ |
| Record type inference (simple) | 중간 | 1-2h | 높음 ⭐⭐⭐ |
| Let expression edge cases | 낮음 | 1h | 낮음 ⭐ |
| Full bidirectional checking | 높음 | 1-2주 | 중간 ⭐⭐ |

**선택:** 높은 가치, 중간 복잡도 작업 우선

---

## 실행 계획

### Phase 1: Variant Constructors (1-2시간)

**목표:** `South`, `North` 등을 함수처럼 인식

**구현:**
```rust
// compiler/src/type_checker.rs

impl TypeChecker {
    fn collect_type_definitions(&mut self) {
        for type_def in &self.program.type_defs {
            self.custom_types.insert(type_def.name.clone(), type_def.clone());
            
            // Variant 생성자 등록
            if let TypeDefKind::Variant(variants) = &type_def.definition {
                for (variant_name, param_types) in variants {
                    if param_types.is_empty() {
                        // Nullary constructor: South -> Direction
                        self.function_types.insert(
                            variant_name.clone(),
                            FunctionType {
                                param_type: Box::new(Type::Basic(BasicType { 
                                    name: "Unit".to_string() 
                                })),
                                return_type: Box::new(Type::Basic(BasicType { 
                                    name: type_def.name.clone() 
                                })),
                                effect: None,
                            }
                        );
                    } else {
                        // Constructor with args: Some(Int) -> Int -> Option<Int>
                        // TODO: Implement if needed
                    }
                }
            }
        }
    }
}
```

**테스트:**
```bash
pole test games/zomboid/specs/player.pole-ir  # uses South, North
pole test examples/12-simple-variant.pole-ir
```

---

### Phase 2: Record Literal Type Inference (1-2시간)

**목표:** `{ health: 100 }` 를 `Player` 타입으로 인식

**구현 (간단한 방법):**
```rust
// compiler/src/type_checker.rs

fn check_function(&mut self, func_def: &FunctionDef) {
    let mut local_env = self.type_env.clone();
    
    for (param_name, param_type) in &func_def.params {
        local_env.insert(param_name.clone(), param_type.clone());
    }
    
    let old_env = std::mem::replace(&mut self.type_env, local_env);
    
    // NEW: expected type 전달
    let body_type = self.infer_type_with_hint(&func_def.body, Some(&func_def.return_type));
    
    if !self.types_compatible(&body_type, &func_def.return_type) {
        self.errors.push(TypeError::with_location(...));
    }
    
    self.type_env = old_env;
}

fn infer_type_with_hint(&mut self, expr: &Expr, expected: Option<&Type>) -> Type {
    match expr {
        Expr::Record(record) => {
            // expected가 custom type이고 record이면 타입 맞춰보기
            if let Some(expected_type) = expected {
                if let Type::Basic(basic) = expected_type {
                    if let Some(type_def) = self.custom_types.get(&basic.name) {
                        if let TypeDefKind::Record(expected_fields) = &type_def.definition {
                            // 필드 이름과 타입이 일치하는지 확인
                            if self.record_matches(record, expected_fields) {
                                return Type::Basic(basic.clone());
                            }
                        }
                    }
                }
            }
            // Fallback: 기존 로직 (anonymous record type)
            self.infer_type(expr)
        }
        _ => self.infer_type(expr)
    }
}
```

**테스트:**
```bash
pole test games/zomboid/specs/player.pole-ir
pole test examples/08-simple-record.pole-ir
```

---

## 안전 장치

### 1. Regression Prevention
```bash
# 기존 예제가 깨지지 않았는지 확인
make verify-ir
pole test examples/01-factorial.pole-ir
pole test examples/03-user-validation.pole
```

### 2. Fallback 유지
```python
# Python fallback은 그대로 유지
# Rust 실패해도 프로세스는 계속
```

### 3. Incremental Commit
```bash
git commit -m "Add variant constructor support"
# 테스트
git commit -m "Add record literal type inference"
# 테스트
```

---

## Success Criteria

### 필수 (Must Have)
- [ ] `pole test games/zomboid/specs/player.pole-ir` 통과
- [ ] `pole test games/zomboid/specs/zombie.pole-ir` 통과
- [ ] 기존 factorial 예제 여전히 통과

### 권장 (Should Have)
- [ ] 모든 variant 예제 통과
- [ ] 모든 record 예제 통과

### 선택 (Nice to Have)
- [ ] Let expression edge cases 해결

---

## Timeline

**시작:** 지금 (2025-10-21 오후)  
**완료 목표:** 오늘 저녁  
**대안 계획:** 내일 오전 (Day 5 계획대로)

---

## Approval

**Human:** (대기 중)  
**LLM:** 전략 문서 작성 완료, 실행 대기 중

---

**Next Step:** 
Human이 "진행해줘" 또는 "OK" 하면 Phase 1부터 시작
