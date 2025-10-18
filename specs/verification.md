# Pole 검증 시스템 요구사항 v0.1

> Pole IR 코드의 정확성, 안전성, 명세 준수를 보장하는 검증 시스템

## 개요

Pole 검증 시스템은 LLM이 생성한 IR 코드가:
1. **타입 안전**한지
2. **명세를 준수**하는지
3. **의미적으로 정확**한지

자동으로 검증합니다.

**설계 원칙**:
- 자동화 우선 (수동 검증 최소화)
- 실패 시 명확한 피드백
- 점진적 검증 (빠른 검사부터 수행)
- 재현 가능한 검증

---

## 1. 타입 체커 (Type Checker)

### 1.1 목적

IR 코드가 타입 시스템 규칙을 준수하는지 정적으로 검증

### 1.2 검증 항목

#### 1.2.1 기본 타입 검사

```
검증: 모든 표현식이 올바른 타입을 가지는가?

예시:
✓ let x: Int = 42
✗ let x: Int = "hello"  // 타입 불일치

✓ if true then 1 else 2
✗ if 42 then 1 else 2   // 조건식은 Bool이어야 함
```

#### 1.2.2 함수 타입 검사

```
검증: 함수 적용 시 타입이 일치하는가?

예시:
func add (x: Int, y: Int) -> Int : x + y

✓ add 1 2              // Int -> Int -> Int
✗ add "a" "b"          // String은 Int가 아님
✗ add 1                // 인자 부족 (부분 적용 의도면 OK)
```

#### 1.2.3 패턴 매칭 완전성

```
검증: 모든 경우를 다루는가?

예시:
type Option<T> = Some(T) | None

✓ match opt with
  | Some(x) -> x
  | None -> 0

✗ match opt with       // None 케이스 누락
  | Some(x) -> x
```

#### 1.2.4 효과 시스템 검증

```
검증: 함수가 선언한 효과만 사용하는가?

예시:
func pure_func (x: Int) -> Int : x + 1   // 순수 함수

✓ pure_func 42
✗ func impure (x: Int) -> Int :          // IO 효과 사용하지만 타입에 명시 안 함
    print "hello"
    x + 1

✓ func with_io (x: Int) ->IO Int :       // IO 효과 명시
    print "hello"
    x + 1
```

#### 1.2.5 의존 타입 검증 (제한적)

```
검증: 타입 수준 제약이 만족되는가?

예시:
type Vec<T, n: Nat> = ...

✓ let v: Vec<Int, 3> = [1, 2, 3]
✗ let v: Vec<Int, 3> = [1, 2]           // 길이 불일치
```

### 1.3 타입 추론

```
타입이 명시되지 않은 경우 Hindley-Milner 타입 추론 사용

예시:
let identity = \x -> x
// 추론된 타입: forall a. a -> a

let map_inc = \list -> List.map (\x -> x + 1) list
// 추론된 타입: List<Int> -> List<Int>
```

### 1.4 에러 메시지 형식

```
[Type Error] <파일>:<라인>:<컬럼>

Expected: <기대 타입>
Found:    <실제 타입>

Expression: <문제의 표현식>

Hint: <해결 방법 제안>
```

**예시**:
```
[Type Error] examples/01-factorial.pole-ir:12:15

Expected: Int
Found:    String

Expression: factorial "5"

Hint: factorial requires a Nat argument. Did you mean: factorial 5?
```

### 1.5 구현 요구사항

- [ ] 타입 환경 (Type Environment) 관리
- [ ] 타입 추론 알고리즘 (Algorithm W 또는 유사)
- [ ] 패턴 매칭 완전성 검사
- [ ] 효과 추적 및 검증
- [ ] 명확한 에러 메시지 생성
- [ ] 소스 위치 추적

---

## 2. 명세 준수 검증 (Specification Compliance)

### 2.1 목적

생성된 IR 코드가 원본 명세(.pole)의 요구사항을 만족하는지 검증

### 2.2 검증 항목

#### 2.2.1 계약 프로그래밍 (Contract Programming)

**사전조건 (requires)**:
```
func factorial (n: Nat) -> Nat
  requires n >= 0
:
  ...

검증:
- 함수 시작 시 requires 조건 체크 코드 존재
- 조건 위반 시 에러 발생
```

**사후조건 (ensures)**:
```
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  ...

검증:
- 함수 종료 시 ensures 조건 체크 코드 존재
- 모든 반환 경로에서 조건 만족 확인
```

**불변 조건 (invariant)**:
```
검증: 루프/재귀에서 불변 조건 유지

예시:
func sum_list (list: List<Int>, acc: Int) -> Int
  ensures result >= acc
:
  match list with
  | Nil -> acc
  | Cons(x, xs) -> sum_list xs (acc + x)
  
검증: 재귀 호출 시에도 ensures 조건 만족
```

#### 2.2.2 예제 기반 검증

```
명세:
examples:
  - 0 → 1
  - 5 → 120

검증 절차:
1. 각 예제를 IR 인터프리터로 실행
2. 출력이 예상값과 일치하는지 확인
3. 모든 예제 통과해야 검증 성공

실패 시:
[Example Test Failed] factorial(5)
Expected: 120
Got:      125
```

#### 2.2.3 제약 조건 검증

```
명세:
constraints:
  - name must be between 1 and 50 characters
  - age must be between 0 and 150

검증:
1. IR 코드에 제약 조건 체크 로직 존재하는가?
2. 제약 위반 시 적절한 에러 반환하는가?

테스트:
- 경계값 테스트 (0, 1, 49, 50, 51)
- 에러 케이스 테스트 (음수, 151)
```

### 2.3 속성 기반 테스트 (Property-Based Testing)

```
명세에서 자동으로 속성 추출:

예시 (factorial):
속성 1: factorial(0) == 1
속성 2: factorial(n) == n * factorial(n-1)  // n > 0
속성 3: factorial(n) >= 1                   // 모든 n >= 0

검증:
- 랜덤 입력으로 속성 테스트 (QuickCheck 스타일)
- 100~1000회 반복 테스트
- 실패 시 최소 반례 찾기
```

### 2.4 구현 요구사항

- [ ] 계약 조건 파싱 및 검증
- [ ] 예제 실행 엔진 (IR 인터프리터)
- [ ] 속성 추출 및 테스트 생성
- [ ] 랜덤 입력 생성기
- [ ] 반례 최소화 (shrinking)

---

## 3. 테스트 생성 전략

### 3.1 명세에서 테스트 자동 생성

#### 3.1.1 예제 → 단위 테스트

```
명세:
examples:
  - input: 0 → output: 1
  - input: 5 → output: 120

생성되는 테스트:
@test
func test_factorial_0 : Bool :
  factorial 0 == 1

@test
func test_factorial_5 : Bool :
  factorial 5 == 120
```

#### 3.1.2 제약 조건 → 경계 테스트

```
명세:
constraints:
  - n >= 0

생성되는 테스트:
@test
func test_factorial_boundary : Bool :
  match factorial (-1) with
  | Err(_) -> true     // 에러 발생 기대
  | Ok(_) -> false

@test
func test_factorial_zero : Bool :
  factorial 0 == 1     // 경계값 0
```

#### 3.1.3 타입 → 속성 테스트

```
함수 시그니처:
func reverse<T> (list: List<T>) -> List<T>

생성되는 속성:
@property
func prop_reverse_involutive<T> (list: List<T>) : Bool :
  reverse (reverse list) == list

@property
func prop_reverse_length<T> (list: List<T>) : Bool :
  List.length (reverse list) == List.length list
```

### 3.2 커버리지 목표

```
최소 달성 목표:
- 명세 예제 커버리지: 100%
- 브랜치 커버리지: 80% 이상
- 경계값 테스트: 모든 제약 조건에 대해

권장 목표:
- 변이 테스트 점수: 70% 이상
- 속성 기반 테스트: 주요 함수에 대해
```

### 3.3 테스트 실행 전략

```
1. 빠른 테스트 먼저:
   - 단위 테스트 (예제 기반)
   - 타입 체크
   
2. 느린 테스트 나중:
   - 속성 기반 테스트 (랜덤 생성)
   - 성능 테스트

3. 실패 시 즉시 중단 옵션
```

### 3.4 테스트 출력 형식

```
[Test Run] examples/01-factorial.pole-ir
========================================

Unit Tests:
  ✓ test_factorial_0         (0.1ms)
  ✓ test_factorial_5         (0.2ms)
  ✓ test_factorial_boundary  (0.1ms)
  
Property Tests:
  ✓ prop_factorial_positive  (100 cases, 15ms)
  
Coverage:
  Examples: 3/3 (100%)
  Branches: 5/6 (83%)
  
Result: PASSED (3 tests, 0 failures)
```

### 3.5 구현 요구사항

- [ ] 테스트 케이스 생성기
  - [ ] 예제 기반 테스트 생성
  - [ ] 경계값 테스트 생성
  - [ ] 속성 기반 테스트 생성
- [ ] 테스트 실행 엔진
- [ ] 커버리지 측정 도구
- [ ] 테스트 리포트 생성

---

## 4. 정적 분석 (Static Analysis)

### 4.1 안전성 검증

#### 4.1.1 메모리 안전성

```
검증 항목:
- 널 포인터 접근 방지 (Option 타입 강제)
- 버퍼 오버플로우 방지 (경계 검사)
- Use-after-free 방지 (선형 타입)

예시:
✓ match opt with
  | Some(x) -> use x
  | None -> default

✗ let x = opt.unwrap()  // 안전하지 않은 접근
```

#### 4.1.2 정수 오버플로우

```
검증:
- Int 타입: 임의 정밀도 (오버플로우 없음)
- 명시적 변환 시 검사

예시:
let x: Int = factorial 1000  // OK, 임의 정밀도

func to_int32 (x: Int) -> Result<Int32, OverflowError> :
  if x < -2147483648 or x > 2147483647 then
    Err(Overflow)
  else
    Ok(x)
```

#### 4.1.3 데이터 레이스 방지

```
검증:
- 불변성 기본 (race 없음)
- 가변 상태는 State 효과로 격리
- 병렬 접근 시 소유권 검사
```

### 4.2 종료성 분석 (Termination Analysis)

```
검증:
- 재귀 함수의 종료 보장
- 구조적 재귀 체크
- 측정 함수 (measure) 감소 증명

예시:
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)  // n-1 < n이므로 종료 보장

경고:
func loop : Unit :
  loop  // 무한 재귀 경고
```

### 4.3 부작용 추적

```
검증:
- 순수 함수가 IO/State 효과 사용하지 않는지
- 효과 타입 일관성

예시:
func pure (x: Int) -> Int :
  x + 1  // OK, 순수

func impure (x: Int) -> Int :
  print x  // ERROR: IO 효과 사용하지만 타입에 없음
  x + 1
```

### 4.4 구현 요구사항

- [ ] 소유권 분석기
- [ ] 종료성 분석기
- [ ] 효과 추적기
- [ ] 데이터 흐름 분석

---

## 5. 형식 검증 (Formal Verification) - Phase 3

### 5.1 정리 증명기 통합

```
목표: requires/ensures 조건의 수학적 증명

도구 후보:
- Z3 (SMT Solver)
- Coq (증명 보조기)
- Lean (의존 타입 증명)

예시:
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
  ensures n == 0 => result == 1
:
  ...

검증: SMT 솔버로 조건 증명
```

### 5.2 모델 체킹

```
목표: 상태 공간 탐색으로 안전성 증명

예시:
- 데드락 부재 증명
- 활성성 증명 (liveness)
- 시간 제약 만족
```

### 5.3 동등성 검증

```
목표: 명세와 구현의 의미적 동등성 증명

기법:
- 기호 실행 (Symbolic Execution)
- 동등성 체크 (Equivalence Checking)
```

---

## 6. 성능 검증

### 6.1 복잡도 분석

```
목표: 시간/공간 복잡도 자동 분석

예시:
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)

분석 결과:
- 시간 복잡도: O(n)
- 공간 복잡도: O(n) (재귀 스택)

명세 요구사항과 비교:
constraints:
  - compute efficiently

검증: O(n)이 "efficient"한지 판단 (휴리스틱)
```

### 6.2 벤치마크 자동 실행

```
생성되는 벤치마크:
@benchmark
func bench_factorial_10 : Unit :
  factorial 10

@benchmark
func bench_factorial_100 : Unit :
  factorial 100

출력:
factorial(10):   0.05ms
factorial(100):  0.8ms
```

### 6.3 성능 회귀 탐지

```
검증:
- 이전 버전과 성능 비교
- 10% 이상 느려지면 경고

출력:
[Performance Regression]
factorial(100): 0.8ms → 1.2ms (+50%)
```

---

## 7. 검증 파이프라인

### 7.1 검증 순서 (빠른 것부터)

```
1. 타입 체크              (1초 이내)
   ↓
2. 예제 기반 테스트       (수 초)
   ↓
3. 계약 조건 검증         (수 초)
   ↓
4. 속성 기반 테스트       (수십 초)
   ↓
5. 정적 분석              (수 분)
   ↓
6. 형식 검증 (선택)       (수 분~수 시간)

실패 시 즉시 중단하여 빠른 피드백 제공
```

### 7.2 검증 수준 선택

```
--verify-level=basic      # 타입 체크 + 예제 테스트
--verify-level=standard   # + 계약 검증 + 속성 테스트 (기본)
--verify-level=strict     # + 정적 분석
--verify-level=formal     # + 형식 검증 (느림)
```

### 7.3 CI/CD 통합

```
Git Hook:
  pre-commit: 타입 체크 + 예제 테스트
  pre-push:   standard 수준 검증

CI Pipeline:
  Pull Request: standard 수준
  Main Branch:  strict 수준
  Release:      formal 수준 (선택)
```

---

## 8. 에러 리포팅

### 8.1 검증 실패 보고서

```
[Verification Report] examples/01-factorial.pole-ir
====================================================

✗ Type Check: FAILED
  - Line 12: Type mismatch (Expected Int, Found String)

✓ Example Tests: PASSED (3/3)

✗ Contract Verification: FAILED
  - ensures result >= 1: Violated for input -1

✓ Property Tests: PASSED (100 cases)

Summary: 2 errors, 0 warnings
Result: FAILED
```

### 8.2 경고 vs 에러

```
에러 (Error): 검증 실패, 빌드 중단
- 타입 에러
- 예제 테스트 실패
- 계약 위반

경고 (Warning): 잠재적 문제, 빌드 계속
- 종료성 불확실
- 성능 우려
- 커버리지 부족
```

---

## 9. 구현 우선순위

### Phase 1 (P0 - 필수)
- [ ] 타입 체커
  - [ ] 기본 타입 검사
  - [ ] 함수 타입 검사
  - [ ] 패턴 매칭 완전성
- [ ] 예제 기반 테스트
  - [ ] IR 인터프리터
  - [ ] 테스트 실행기
- [ ] 계약 조건 검증
  - [ ] requires/ensures 체크

### Phase 2 (P1 - 중요)
- [ ] 속성 기반 테스트
  - [ ] 랜덤 입력 생성
  - [ ] 속성 추출 및 테스트
- [ ] 정적 분석
  - [ ] 효과 추적
  - [ ] 종료성 분석
- [ ] 테스트 자동 생성
  - [ ] 경계값 테스트
  - [ ] 커버리지 측정

### Phase 3 (P2 - 선택)
- [ ] 형식 검증
  - [ ] SMT 솔버 통합
  - [ ] 모델 체킹
- [ ] 성능 검증
  - [ ] 복잡도 분석
  - [ ] 벤치마크 자동화

---

## 10. 참고 문서

- [README.md](../README.md) (라인 205-277) - 검증 시스템 요구사항
- [specs/workflow.md](workflow.md) (Step 5) - 검증 워크플로우
- [specs/implementation-lang.md](implementation-lang.md) - 타입 시스템 설계
- [specs/ir-syntax.md](ir-syntax.md) - IR 문법

---

## 11. 검증 시스템 체크리스트

구현 시 반드시 포함해야 할 기능:

**타입 검증**:
- [ ] 타입 추론 엔진
- [ ] 타입 검사기
- [ ] 효과 시스템 검증

**명세 준수**:
- [ ] 예제 실행 및 비교
- [ ] 계약 조건 검증
- [ ] 제약 조건 테스트

**테스트 생성**:
- [ ] 예제 → 단위 테스트 변환
- [ ] 경계값 테스트 생성
- [ ] 속성 추출 및 테스트

**리포팅**:
- [ ] 명확한 에러 메시지
- [ ] 소스 위치 추적
- [ ] 검증 리포트 생성
