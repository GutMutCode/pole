# IR Parser Multi-arg 함수 호출 버그 수정

**작성일:** 2025-10-20
**소요 시간:** 2시간
**우선순위:** P0 Critical

## 요약

IR Parser가 multi-argument 함수 호출을 잘못 파싱하던 버그를 수정했습니다.

**Before:** `f(a, b+1)` → `Application(func=f, arg=Variable("a, b+1"))`  
**After:** `f(a, b+1)` → `Application(func=f, arg=TupleExpr([Variable("a"), BinaryOp("+", Variable("b"), Literal(1))]))`

## 발견 배경

LLM 변환 시스템 테스트 중 다음 문제 발견:

1. LLM이 `inventory_add_item(inv, max_slots, item_id, quantity)` 같은 multi-arg 함수 호출 생성
2. IR Parser가 `"inv, max_slots, item_id, quantity"`를 단일 변수명으로 파싱
3. Type Checker 에러: "Undefined variable 'inv, max_slots, item_id, quantity'"

## 근본 원인

`src/pole/runtime/ir_parser.py:393`의 `_parse_simple_expr()`:

```python
# 기존 코드 (버그)
elif func_name:
    arg = self._parse_simple_expr(args_str)  # 전체 문자열을 단일 표현식으로 파싱
    return Application(func=Variable(name=func_name), arg=arg)
```

**문제:**
- Constructor는 `args_str.split(",")`로 각 인자를 개별 파싱 (line 386-391)
- Function application은 전체를 하나의 표현식으로 파싱
- 불일치로 인해 multi-arg 함수 호출 불가능

## 해결 방법

### 1. `_split_args()` 헬퍼 함수 추가

괄호 depth를 고려하여 콤마로 인자 분리:

```python
def _split_args(self, args_str: str) -> list[str]:
    """Split function arguments by comma, respecting parentheses"""
    args = []
    current = []
    paren_depth = 0
    
    for char in args_str:
        if char == '(':
            paren_depth += 1
            current.append(char)
        elif char == ')':
            paren_depth -= 1
            current.append(char)
        elif char == ',' and paren_depth == 0:
            args.append(''.join(current).strip())
            current = []
        else:
            current.append(char)
    
    if current:
        args.append(''.join(current).strip())
    
    return args
```

### 2. Function Application 파싱 수정

```python
elif func_name:
    # Multi-argument function call: parse as tuple
    if ',' in args_str:
        args = [self._parse_simple_expr(a.strip()) for a in self._split_args(args_str)]
        arg = TupleExpr(elements=args)
    else:
        arg = self._parse_simple_expr(args_str) if args_str else Literal(value=None, type_name="Unit")
    return Application(func=Variable(name=func_name), arg=arg)
```

**핵심:**
- 콤마가 있으면 → `TupleExpr`로 변환
- 단일 인자 → 기존 방식 유지
- 빈 인자 `()` → `Literal(Unit)`

### 3. Type Checker 통합

Type Checker는 이미 `TupleExpr`와 `TupleType` 매칭을 지원:

```python
# src/pole/verifier/type_checker.py:266-268
elif isinstance(expr, TupleExpr):
    element_types = [self._infer_type(elem) for elem in expr.elements]
    return TupleType(element_types=element_types)
```

**함수 정의:**
```pole
func add(a: Int, b: Int) -> Int
```
→ `params = [('a', Int), ('b', Int)]` (이미 tuple 형태)

**함수 호출:**
```pole
add(1, 2)
```
→ `Application(func=add, arg=TupleExpr([Literal(1), Literal(2)]))`

Type checker가 `TupleExpr` → `TupleType([Int, Int])`로 추론 → params와 매칭 ✓

## 검증 결과

### Parser 테스트

```python
# Test 1: Simple 2-arg
func add(a: Int, b: Int) -> Int:
  a + b
func main() -> Int:
  add(5, 3)
✓ PASSED

# Test 2: 3-arg with expression
func sum3(a: Int, b: Int, c: Int) -> Int:
  a + b + c
func main() -> Int:
  sum3(1, 2 + 3, 4 * 5)
✓ PASSED

# Test 3: Nested calls
func add(a: Int, b: Int) -> Int:
  a + b
func main() -> Int:
  add(add(1, 2), add(3, 4))
✓ PASSED

# Test 4: SDL-style 6-arg
func create_window(title: String, x: Int, y: Int, w: Int, h: Int, flags: Int) -> Int:
  42
func main() -> Int:
  create_window("Test", 100, 100, 800, 600, 8)
✓ PASSED
```

### LLM 생성 IR 테스트

**입력:** `games/zomboid/specs/inventory-simple.pole`

**출력:** `/tmp/inventory-test-final.pole-ir` (2,645 bytes)

**함수 예시:**
```pole
func inventory_add_item(inv: Ptr<Unit>, max_slots: Int, item_id: Int, quantity: Int) -> Int
  requires item_id > 0 && quantity > 0 && max_slots > 0
  ensures result == 1 || result == 0
:
  if item_id <= 0 || quantity <= 0 then 0 
  else find_or_create_slot(inv, max_slots, item_id, quantity, 0)
```

**호출 예시:**
```pole
func test_inventory_add_item_new() -> Bool:
  let result = inventory_add_item(0 : Ptr<Unit>, 10, 1, 5) in
  result == 1
```

**결과:**
- ✅ 파싱 성공
- ✅ 타입 체크 통과 (multi-arg 함수 호출 인식)
- ⚠️ 일부 LLM 생성 문법 오류 (`null`, `0 : Ptr<Unit>`) - 별도 이슈

## 부수적 수정

### LLM System Prompt 개선

`src/pole/transformer/llm_transformer.py`의 시스템 프롬프트에 `else if` 포맷 가이드라인 추가:

```python
Example 3 - Nested if (MUST be single line):
func classify(n: Int) -> String:
  if n == 0 then "zero" else if n > 0 then "positive" else "negative"

Example 4 - Multi-line if (with indentation):
func validate_range(x: Int) -> String:
  if x < 0 then
    "negative"
  else if x > 100 then
    "too large"
  else
    "valid"

CRITICAL: For else-if chains:
- EITHER write entire expression on ONE line
- OR use multi-line with indented values
- NEVER mix formats
```

**이유:** LLM이 혼합 포맷 생성 (`if x then y` 같은 줄, `else if` 다음 줄)

## 영향 범위

### 긍정적 영향

1. **Multi-arg 함수 완전 지원**
   - SDL2, OpenGL 등 C FFI 함수 호출 가능
   - 일반적인 Pole 코드 작성 가능

2. **LLM 변환 파이프라인 복원**
   - .pole 명세 → LLM → .pole-ir 전체 파이프라인 작동
   - Pole 핵심 원칙 달성: 사람=명세, LLM=구현

3. **타입 안전성 유지**
   - Multi-arg 호출도 타입 체크 통과
   - 컴파일 타임 에러 감지

### 기존 코드 영향

- **72개 .pole-ir 파일:** 일부 파싱 에러 존재 (record syntax, standalone annotations)
- **결정:** 기존 파일 그대로 유지, 새 개발은 .pole 명세 기반

## 남은 작업

### P0 (필수)
- ✅ Parser 수정 완료
- ✅ Type Checker 통합 완료
- ✅ LLM prompt 개선 완료

### P1 (중요)
- [ ] LLM 생성 코드 품질 개선
  - `null` → Pole IR에 맞는 표현으로 변경
  - `0 : Ptr<Unit>` → 타입 캐스팅 문법 지원 또는 대안
- [ ] System prompt 지속 개선
  - 더 많은 예제 추가
  - 금지 패턴 명시

### P2 (선택)
- [ ] 기존 72개 .pole-ir 파일을 .pole 명세로 역변환
  - 교육 자료로 활용
  - LLM fine-tuning 데이터

## 결론

**핵심 성과:**
- ✅ Multi-arg 함수 호출 완전 지원
- ✅ LLM 변환 파이프라인 정상 작동
- ✅ Pole 핵심 원칙 복원

**다음 단계:**
- PZ 게임 개발을 .pole 명세 기반으로 재시작
- LLM system prompt 지속 개선
- 표준 라이브러리 명세 작성 시작

**기술적 교훈:**
1. Parser와 Type Checker의 일관성 중요
2. `TupleExpr` 활용으로 multi-arg 문제 우아하게 해결
3. LLM 프롬프트는 지속적 개선 필요
