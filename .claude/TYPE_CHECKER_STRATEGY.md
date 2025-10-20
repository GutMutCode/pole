# Rust 타입 체커 문제 처리 전략

**작성일:** 2025-10-21  
**목적:** LLM 자동화 프로세스를 망가뜨리지 않고 남은 타입 체커 문제 해결

---

## 📊 현재 상태

### ✅ 해결된 문제 (오늘 완료)
1. IR 파서 버그 - Record literal syntax (`:` vs `=`)
2. 함수 타입 시그니처 - Currying 지원
3. 커스텀 타입 해석 - `resolve_type()` 추가
4. 빌트인 함수 추가 - list_get, list_set, list_push, int_to_float, float_to_int

### ⚠️ 남은 문제

#### 1. Variant constructors not in scope (P1)
```pole-ir
type Direction = North | South | East | West

func test() -> Direction:
  South  // ❌ Undefined variable 'South'
```

**영향도:** 높음 - player.pole-ir, zombie.pole-ir에서 사용  
**복잡도:** 중간 - variant 생성자를 function_types에 등록 필요

#### 2. Record literal type checking (P2)
```pole-ir
type Player = { health: Int }

func make() -> Player:
  { health: 100 }  // ❌ Type mismatch: {health: Int} vs Player
```

**영향도:** 높음 - 모든 record 반환 함수에서 발생  
**복잡도:** 높음 - Bidirectional type checking 필요

#### 3. Let expression edge cases (P2)
```pole-ir
func test() -> Int:
  let x = 10 in
  x  // ❌ Undefined variable 'let' (가끔 발생)
```

**영향도:** 낮음 - 드물게 발생  
**복잡도:** 낮음 - 파싱 이슈일 가능성

---

## 🎯 처리 전략

### Phase 1: 즉시 처리 (Day 5 오전, 오늘 오후)
**목표:** LLM 생성 코드가 타입 체크를 통과하도록

**작업 1: Variant Constructors 지원 (1-2시간)**

```rust
// compiler/src/type_checker.rs

impl TypeChecker {
    fn collect_type_definitions(&mut self) {
        for type_def in &self.program.type_defs {
            self.custom_types.insert(type_def.name.clone(), type_def.clone());
            
            // NEW: Variant 생성자를 function_types에 등록
            if let TypeDefKind::Variant(variants) = &type_def.definition {
                for (variant_name, _param_types) in variants {
                    // North -> Direction
                    // South(Int) -> Int -> Direction
                    self.register_variant_constructor(
                        variant_name,
                        &type_def.name,
                        param_types
                    );
                }
            }
        }
    }
}
```

**테스트:**
```bash
pole test games/zomboid/specs/player.pole-ir  # South, North 등 사용
pole test examples/12-simple-variant.pole-ir  # 기존 variant 예제
```

**예상 결과:** player.pole-ir 타입 체크 통과

---

**작업 2: Record Literal Type Inference (2-3시간)**

**옵션 A: 간단한 해결 (권장)**
- Record literal을 만나면 expect type과 비교
- 필드가 모두 일치하면 expect type으로 간주

```rust
fn check_function(&mut self, func_def: &FunctionDef) {
    // 함수 body를 체크할 때 expected_type을 전달
    let body_type = self.infer_type_with_expected(
        &func_def.body,
        Some(&func_def.return_type)
    );
    // ...
}

fn infer_type_with_expected(&mut self, expr: &Expr, expected: Option<&Type>) -> Type {
    match expr {
        Expr::Record(record) => {
            // expected가 있고 Record 타입이면 사용
            if let Some(Type::Basic(basic)) = expected {
                if let Some(type_def) = self.custom_types.get(&basic.name) {
                    if let TypeDefKind::Record(expected_record) = &type_def.definition {
                        // 필드가 일치하는지 확인만 하고 expected type 반환
                        return Type::Basic(basic.clone());
                    }
                }
            }
            // fallback: 기존 로직
            // ...
        }
        _ => self.infer_type(expr)
    }
}
```

**옵션 B: 완전한 Bidirectional Type Checking (미래에)**
- Hindley-Milner 타입 추론
- Unification 알고리즘
- Week 2 이후로 연기

**권장:** 옵션 A 선택 - Week 1에는 실용적 해결이 중요

**테스트:**
```bash
pole test games/zomboid/specs/player.pole-ir
pole test examples/08-simple-record.pole-ir
```

---

### Phase 2: 선택적 처리 (Day 6-7 또는 Week 2)
**작업 3: Let expression edge cases**

**우선순위:** P2 (낮음)  
**이유:** 
- 드물게 발생
- Python fallback으로 회피 가능
- 더 심층적인 파서 디버깅 필요

**처리 시점:** Week 1 데모 완성 후

---

## 🔄 자동화 프로세스와의 통합

### 원칙
1. **LLM 생성 코드 우선** - 타입 체커가 LLM 출력에 맞춰야 함
2. **점진적 개선** - 한 번에 하나씩, 테스트 확인
3. **Python fallback 유지** - Rust가 실패해도 빌드는 계속

### 안전 장치

**Before (현재 상태):**
```python
# src/pole/verifier/type_checker_rust.py

def check_types(program):
    if RUST_AVAILABLE:
        try:
            return check_types_rust(program)
        except:
            return check_types_python(program)  # ✅ Fallback
    else:
        return check_types_python(program)
```

**After (개선 후):**
```python
def check_types(program):
    if RUST_AVAILABLE:
        result = check_types_rust(program)
        if not result.success:
            # Rust 타입 체크 실패 → Python으로 재시도
            python_result = check_types_python(program)
            if python_result.success:
                # Python은 성공 → Rust 버그 가능성
                log_rust_type_checker_issue(result.errors)
                return python_result
        return result
    else:
        return check_types_python(program)
```

---

## 📅 실행 계획

### 오늘 오후 (2025-10-21, 3-4시간)
1. ✅ Variant constructors 지원 구현 (1-2h)
2. ✅ Record literal type inference 구현 (2h)
3. ✅ 전체 테스트 실행
   ```bash
   pole test games/zomboid/specs/player.pole-ir
   pole test games/zomboid/specs/zombie.pole-ir
   pole test examples/01-factorial.pole-ir
   pole test examples/12-simple-variant.pole-ir
   pole test examples/08-simple-record.pole-ir
   ```
4. ✅ 커밋 & PENDING_ISSUES.md 업데이트

### Day 5 나머지 (내일 또는 필요시)
- Pole Engine 리팩토링 (원래 계획대로)
- 문서화

### Day 6-7 (선택)
- Let expression edge cases (시간 있으면)

---

## 🎓 교훈

### LLM 자동화를 위한 타입 체커 설계 원칙

1. **관대한 타입 시스템**
   - Unknown 타입 허용 → 점진적 타입 체크
   - 실패해도 빌드는 계속

2. **LLM 친화적 에러 메시지**
   - "Missing variant constructor 'South'" (구체적)
   - ~~"Type error"~~ (모호함)

3. **Fallback 전략**
   - Rust (빠름, 정확) → Python (느림, 관대)
   - 최악의 경우: 타입 체크 스킵하고 컴파일 시도

4. **점진적 배포**
   - 한 feature씩 Rust로 마이그레이션
   - 각 단계마다 테스트

---

## ✅ Success Criteria

### Day 5 완료 기준
- [ ] `pole test games/zomboid/specs/player.pole-ir` 통과
- [ ] `pole test games/zomboid/specs/zombie.pole-ir` 통과
- [ ] 기존 예제들 여전히 통과 (regression 없음)
- [ ] LLM 자동화 파이프라인 정상 작동

### Week 1 완료 기준
- [ ] 1분 데모 실행 가능
- [ ] YouTube 업로드
- [ ] 타입 체커 80% 커버리지 (완벽하지 않아도 OK)

---

**Last Updated:** 2025-10-21  
**Next Review:** Day 5 완료 후
