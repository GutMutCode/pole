# Pole 구현 언어 (IR) 설계 v0.1

> LLM이 생성하는 형식적이고 검증 가능한 중간 언어

## 설계 원칙

Pole IR은 명세 언어(.pole)와 실행 코드 사이의 **검증 가능한 중간 표현**입니다.

### 핵심 목표
1. **완전한 형식성**: 모든 동작이 수학적으로 정의됨
2. **타입 안전성**: 컴파일 타임에 모든 타입 오류 검출
3. **검증 가능성**: 정적 분석 및 형식 검증 가능
4. **추적성**: 명세로 역추적 가능한 메타데이터 포함
5. **최적화 가능성**: 순수 함수형 스타일로 변환 용이

---

## 1. 타입 시스템

### 1.1 기본 타입

```
Int        : 임의 정밀도 정수 (오버플로우 안전)
Nat        : 자연수 (0 이상의 정수)
Float64    : 64비트 부동소수점
Bool       : true | false
String     : UTF-8 문자열
Unit       : () - 값이 없음을 나타냄
```

### 1.2 복합 타입

#### Option 타입 (널 안전성)
```
Option<T> = Some(T) | None
```
명세의 "있을 수도 없을 수도"를 표현

#### Result 타입 (에러 처리)
```
Result<T, E> = Ok(T) | Err(E)
```
모든 실패 가능한 연산은 Result 반환

#### List 타입
```
List<T> = Nil | Cons(T, List<T>)
```

#### Tuple 타입
```
(T1, T2, ..., Tn)
```

#### Record 타입
```
{ field1: T1, field2: T2, ... }
```

### 1.3 함수 타입

```
T1 -> T2              : 순수 함수
T1 ->IO T2            : IO 효과를 가진 함수
T1 ->State<S> T2      : 상태 효과를 가진 함수
T1 ->Error<E> T2      : 에러 효과를 가진 함수
```

**효과 시스템**으로 부작용 명시적 표현

### 1.4 의존 타입 (제한적 지원)

```
Vec<T, n: Nat>        : 길이가 n인 벡터
Range<min: Int, max: Int> : min과 max 사이의 정수
```

제약 조건을 타입에 인코딩하여 컴파일 타임 검증

### 1.5 제네릭 타입

```
forall T. List<T>
forall T, E. Result<T, E>
```

---

## 2. 형식 의미론

### 2.1 표현식 (Expressions)

#### 리터럴
```
42          : Int
3.14        : Float64
true        : Bool
"hello"     : String
()          : Unit
```

#### 변수
```
x           : 환경에서 x의 값 참조
```

#### 함수 적용
```
f x         : 함수 f에 인자 x 적용
f x y       : 커링된 함수 적용 (f x) y
```

#### 람다 추상화
```
\x -> e     : x를 인자로 받아 e를 평가하는 함수
\x y -> e   : 다중 인자 (\x -> (\y -> e))
```

#### Let 바인딩
```
let x = e1 in e2
  : e1을 평가하여 x에 바인딩, e2에서 x 사용 가능
```

#### 조건식
```
if cond then e1 else e2
  : cond가 true면 e1, false면 e2 평가
```

#### 패턴 매칭
```
match expr with
  | Pattern1 -> e1
  | Pattern2 -> e2
  | _ -> default
```

### 2.2 패턴 (Patterns)

```
_                  : 와일드카드 (무시)
x                  : 변수 바인딩
42                 : 리터럴 매칭
Some(x)            : 생성자 패턴
(x, y)             : 튜플 패턴
{ field1: x, ... } : 레코드 패턴
```

### 2.3 평가 전략

**Call-by-value + Lazy evaluation**
- 기본: Strict evaluation (인자를 먼저 평가)
- 필요시: `lazy` 키워드로 지연 평가

```
let x = lazy expensive_computation in
  if condition then x else 0
```

---

## 3. 메모리 모델

### 3.1 불변성 기본

모든 값은 **기본적으로 불변(immutable)**

```
let x = 42 in
let x = x + 1 in  // 새로운 바인딩, 기존 x는 섀도잉됨
  x
```

### 3.2 소유권 시스템 (선형 타입)

**리소스 관리가 필요한 타입**은 선형 타입 사용

```
File : Linear      // 정확히 한 번만 사용되어야 함
Socket : Linear
```

선형 타입은:
- 정확히 한 번만 소비됨
- 복제 불가
- Drop 시 자동으로 리소스 해제

### 3.3 참조와 차용

기본적으로 값 의미론이지만, 큰 데이터 구조는 내부적으로 공유

```
let list1 = [1, 2, 3, 4, 5] in
let list2 = list1 in
  // list1과 list2는 동일한 메모리 공유 (내부 최적화)
  // 하지만 의미론적으로는 독립적 값
```

### 3.4 가비지 컬렉션

자동 메모리 관리:
- 참조 카운팅 + 세대별 GC
- 순환 참조 탐지
- 결정론적 소멸자 (선형 타입)

---

## 4. 효과 시스템

### 4.1 순수 함수

```
pure_add : Int -> Int -> Int
pure_add = \x y -> x + y
```

부작용 없음, 참조 투명성 보장

### 4.2 IO 효과

```
print : String ->IO Unit
read_line : Unit ->IO String
```

입출력 연산은 IO 효과로 표시

### 4.3 State 효과

```
get : Unit ->State<S> S
put : S ->State<S> Unit
```

상태 변경 연산

### 4.4 Error 효과

```
throw : E ->Error<E> T
catch : (Unit ->Error<E> T) -> (E -> T) -> T
```

예외 처리

### 4.5 효과 조합

```
read_and_validate : Unit ->IO (Result<User, ValidationError>)
```

여러 효과 조합 가능

---

## 5. 함수 정의 문법

### 5.1 기본 형태

```
func <name> (param1: T1, param2: T2) -> ReturnType :
  <body>
```

### 5.2 사전/사후 조건

```
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
  ensures (n == 0) ==> (result == 1)
:
  match n with
  | 0 -> 1
  | _ -> n * factorial (n - 1)
```

### 5.3 효과 표시

```
func validate_user (user: User) -> Result<Unit, ValidationError> :
  if String.length user.name < 1 then
    Err(NameEmpty)
  else if String.length user.name > 50 then
    Err(NameTooLong)
  else if not (String.contains user.email "@") then
    Err(InvalidEmail)
  else if user.age < 0 or user.age > 150 then
    Err(InvalidAge)
  else
    Ok(())
```

---

## 6. 타입 정의 문법

### 6.1 레코드 타입

```
type User = {
  name: String,
  email: String,
  age: Nat
}
```

### 6.2 합 타입 (Algebraic Data Types)

```
type ValidationError =
  | NameEmpty
  | NameTooLong
  | InvalidEmail
  | InvalidAge
```

### 6.3 제네릭 타입

```
type Option<T> =
  | Some(T)
  | None

type List<T> =
  | Nil
  | Cons(T, List<T>)
```

---

## 7. 모듈 시스템

### 7.1 모듈 정의

```
module Math :
  func add (x: Int, y: Int) -> Int : x + y
  func mul (x: Int, y: Int) -> Int : x * y
end
```

### 7.2 모듈 임포트

```
import Math
import List as L

func test : Int :
  Math.add 1 2
```

### 7.3 시그니처 (인터페이스)

```
signature Comparable<T> :
  func compare : T -> T -> Ordering
  func equal : T -> T -> Bool
end

module IntComparable : Comparable<Int> :
  func compare (x: Int, y: Int) -> Ordering :
    if x < y then Less
    else if x > y then Greater
    else Equal
  
  func equal (x: Int, y: Int) -> Bool :
    compare x y == Equal
end
```

---

## 8. 메타데이터 및 추적성

### 8.1 소스 위치 정보

모든 IR 노드는 원본 명세의 위치 정보 포함

```
@source("examples/01-factorial.pole", line=3, col=10)
func factorial (n: Nat) -> Nat :
  ...
```

### 8.2 생성 근거

```
@generated_from("명세: compute efficiently")
@reasoning("메모이제이션 사용하여 중복 계산 방지")
func fibonacci_memo (n: Nat) -> Nat :
  ...
```

### 8.3 최적화 힌트

```
@inline
@pure
@tailrec
func helper (acc: Int, n: Int) -> Int :
  ...
```

---

## 9. 명세 언어와의 대응

### 9.1 타입 매핑

| 명세 언어 | Pole IR |
|-----------|---------|
| integer, int | Int |
| non-negative integer | Nat |
| string, text | String |
| bool, boolean | Bool |
| list\<T\> | List\<T\> |
| option\<T\> | Option\<T\> |

### 9.2 제약 조건 → 사전조건

명세의 `constraints` → IR의 `requires`

```pole
// 명세
constraints:
  - n >= 0
```

```
// IR
func factorial (n: Nat) -> Nat
  requires n >= 0
```

### 9.3 예제 → 테스트

명세의 `examples` → IR의 테스트 케이스

```pole
// 명세
examples:
  - 0 → 1
  - 5 → 120
```

```
// IR (메타데이터)
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat : ...
```

---

## 10. 예제: Factorial

### 명세 언어 (.pole)
```pole
function factorial:
  purpose: calculate factorial of given integer
  input: n (non-negative integer)
  output: n! (integer)
  
  constraints:
    - n >= 0
    - handle overflow safely if n is too large
  
  examples:
    - 0 → 1
    - 1 → 1
    - 5 → 120
```

### 구현 언어 (Pole IR)
```
@source("examples/01-factorial.pole", line=3)
@test_case(input=0, expected=1)
@test_case(input=1, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
  ensures (n == 0) ==> (result == 1)
  ensures (n == 1) ==> (result == 1)
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
```

---

## 11. 예제: User Validation

### 명세 언어 (.pole)
```pole
type User:
  fields:
    - name: string
    - email: string
    - age: integer

function validate_user:
  input: user (User type)
  output: validation result (success/failure + error messages)
  
  constraints:
    - name must be between 1 and 50 characters
    - email must be valid format
    - age must be between 0 and 150
```

### 구현 언어 (Pole IR)
```
type User = {
  name: String,
  email: String,
  age: Int
}

type ValidationError =
  | NameEmpty
  | NameTooLong
  | InvalidEmail
  | InvalidAge

func validate_name (name: String) -> Result<Unit, ValidationError> :
  let len = String.length name in
  if len < 1 then
    Err(NameEmpty)
  else if len > 50 then
    Err(NameTooLong)
  else
    Ok(())

func validate_email (email: String) -> Result<Unit, ValidationError> :
  if String.contains email "@" then
    Ok(())
  else
    Err(InvalidEmail)

func validate_age (age: Int) -> Result<Unit, ValidationError> :
  if age < 0 or age > 150 then
    Err(InvalidAge)
  else
    Ok(())

func validate_user (user: User) -> Result<Unit, List<ValidationError>> :
  let errors = List.concat [
    Result.err_to_list (validate_name user.name),
    Result.err_to_list (validate_email user.email),
    Result.err_to_list (validate_age user.age)
  ] in
  match errors with
  | Nil -> Ok(())
  | errs -> Err(errs)
```

---

## 12. 형식 의미론 (간략)

### 12.1 타입 규칙 (예시)

```
Γ ⊢ e1 : T1    Γ ⊢ e2 : T2
─────────────────────────── (T-Tuple)
Γ ⊢ (e1, e2) : (T1, T2)


Γ, x:T1 ⊢ e : T2
───────────────────── (T-Lambda)
Γ ⊢ \x -> e : T1 -> T2


Γ ⊢ e1 : T1 -> T2    Γ ⊢ e2 : T1
────────────────────────────────── (T-App)
Γ ⊢ e1 e2 : T2
```

### 12.2 조작적 의미론 (Small-step)

```
────────────── (E-Value)
v ⇝ v


e1 ⇝ e1'
─────────────────── (E-App1)
e1 e2 ⇝ e1' e2


(\x -> e) v ⇝ e[x := v]  (E-AppLam)


if true then e1 else e2 ⇝ e1   (E-IfTrue)
if false then e1 else e2 ⇝ e2  (E-IfFalse)
```

---

## 13. 구현 전략

### 13.1 LLM 생성 시

1. 명세 분석
2. 타입 추론
3. 제약 조건 → requires/ensures 변환
4. 알고리즘 선택 (순수 함수 우선)
5. IR 생성
6. 메타데이터 추가

### 13.2 검증 단계

1. 타입 체킹
2. 사전/사후 조건 검증
3. 예제 테스트 실행
4. 부작용 검사 (효과 시스템)
5. 메모리 안전성 검증

---

## 14. 확장성

### Phase 1 (현재)
- 기본 타입 시스템
- 순수 함수형 핵심
- 패턴 매칭
- 간단한 효과 시스템

### Phase 2 (향후)
- 완전한 의존 타입
- 세션 타입 (프로토콜 검증)
- 병렬성 프리미티브
- 증명 보조기 통합

### Phase 3 (장기)
- 점진적 타입 시스템
- 타입 추론 개선
- 최적화 패스 추가
- LLVM 백엔드

---

## 참고 문서

- README.md (라인 134-201): 구현 언어 요구사항
- ARCHITECTURE.md: 시스템 아키텍처
- specs/syntax-v0.md: 명세 언어 문법
- specs/workflow.md: LLM 변환 워크플로우
