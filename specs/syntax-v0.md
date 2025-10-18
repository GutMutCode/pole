# Pole 명세 언어 문법 v0.1

## 설계 원칙
- 자연어에 가까운 표현
- 엄격한 순서나 형식 불필요
- LLM이 의도를 파악할 수 있는 수준의 구조

## 기본 구조

### 함수 정의
```
function <name>:
  purpose: <what the function does>
  input: <parameter description>
  output: <return value description>
  constraints: <constraint conditions>
  examples:
    - <input> → <output>
```

**유연성**:
- 순서 무관: "output", "input", "purpose" 순서 바꿔도 됨
- 생략 가능: "constraints" 없으면 생략
- 표현 자유: "returns", "result", "output" 등 동의어 허용
- 들여쓰기: 탭/스페이스 혼용 가능

## 타입 표현

### 기본 타입
```
- int, integer, number
- string, text
- bool, boolean
- list<type>, array<type>
- option<type>, maybe<type> (nullable)
```

### 구조체/레코드
```
type <Name>:
  fields:
    - <field_name>: <type> - <description>
```

## 제약 조건 표현

```
constraints:
  - <condition in natural language>
  - must <something>
  - <value> between <range>
  - if <situation> then <condition>
```

## 의도적 모호성

다음은 **의도적으로 모호하게** 둘 수 있음:
- 알고리즘 선택: "efficiently", "fast"
- 자료구조: "appropriate collection"
- 에러 처리: "handle safely"
- 최적화: "consider performance"

LLM이 컨텍스트와 제약 조건을 보고 최적 선택

## 예제 포함

```
examples:
  - input: <value>
    output: <value>
    note: <why this happens>
    
  - edge_case: <special case>
    result: <expected behavior>
```

## 주석

```
// single line comment
/* 
   multi-line comment
   provides additional context to LLM
*/
```

## 완전한 예제

```pole
function factorial:
  purpose: calculate factorial of given integer
  input: n (non-negative integer)
  output: n! (integer)
  
  constraints:
    - n >= 0
    - handle overflow if n is too large
  
  examples:
    - 0 → 1
    - 1 → 1
    - 5 → 120
    - -1 → error (negative not allowed)
```
