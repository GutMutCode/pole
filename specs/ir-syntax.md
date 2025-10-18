# Pole IR 문법 명세 v0.1

> Pole 구현 언어(IR)의 구체적 문법 정의

## 개요

이 문서는 `specs/implementation-lang.md`에 정의된 Pole IR의 구체적 문법을 EBNF 형식으로 정의합니다.

**설계 원칙**:
- LLM이 생성하기 쉬운 구조
- 파싱이 명확하고 간단
- 의미가 명시적
- 메타데이터 포함 가능

---

## 1. 어휘 구조 (Lexical Structure)

### 1.1 키워드

```
func        type        module      signature   import
let         in          match       with        if
then        else        requires    ensures     lazy
forall      end         as
```

### 1.2 연산자 및 구두점

```
->          =>          :           =           |
(           )           {           }           [
]           ,           .           @           _
<           >           
```

### 1.3 리터럴

```ebnf
integer     = ["-"] digit+ ;
float       = ["-"] digit+ "." digit+ ;
boolean     = "true" | "false" ;
string      = '"' char* '"' ;
unit        = "()" ;
```

### 1.4 식별자

```ebnf
identifier  = lowercase (letter | digit | "_")* ;
type_id     = uppercase (letter | digit | "_")* ;
```

### 1.5 주석

```ebnf
line_comment  = "//" anything* newline ;
block_comment = "/*" anything* "*/" ;
```

---

## 2. 타입 문법 (Type Grammar)

### 2.1 기본 문법

```ebnf
type = basic_type
     | compound_type
     | function_type
     | type_variable
     | type_application
     ;

basic_type = "Int" | "Nat" | "Float64" | "Bool" | "String" | "Unit" ;

compound_type = option_type
              | result_type
              | list_type
              | tuple_type
              | record_type
              ;

option_type = "Option" "<" type ">" ;
result_type = "Result" "<" type "," type ">" ;
list_type   = "List" "<" type ">" ;
tuple_type  = "(" type ("," type)+ ")" ;
record_type = "{" record_field ("," record_field)* "}" ;

record_field = identifier ":" type ;
```

### 2.2 함수 타입

```ebnf
function_type = type "->" type
              | type "->IO" type
              | type "->State<" type ">" type
              | type "->Error<" type ">" type
              ;
```

### 2.3 제네릭 타입

```ebnf
type_variable   = identifier ;
type_application = type_id "<" type_args ">" ;
type_args       = type ("," type)* ;
forall_type     = "forall" type_variable+ "." type ;
```

### 2.4 의존 타입 (제한적)

```ebnf
dependent_type = "Vec" "<" type "," nat_expr ">"
               | "Range" "<" int_expr "," int_expr ">"
               ;
```

---

## 3. 표현식 문법 (Expression Grammar)

### 3.1 기본 표현식

```ebnf
expr = literal
     | variable
     | lambda
     | application
     | let_expr
     | if_expr
     | match_expr
     | constructor_expr
     | tuple_expr
     | record_expr
     | field_access
     | "(" expr ")"
     ;

literal  = integer | float | boolean | string | unit ;
variable = identifier ;
```

### 3.2 람다 표현식

```ebnf
lambda = "\" param+ "->" expr ;
param  = identifier ;
```

### 3.3 함수 적용

```ebnf
application = expr expr ;
```

### 3.4 Let 바인딩

```ebnf
let_expr = "let" identifier "=" expr "in" expr ;
```

### 3.5 조건식

```ebnf
if_expr = "if" expr "then" expr "else" expr ;
```

### 3.6 패턴 매칭

```ebnf
match_expr = "match" expr "with" match_arm+ ;
match_arm  = "|" pattern "->" expr ;
```

### 3.7 생성자 및 복합 표현식

```ebnf
constructor_expr = type_id ["(" expr ("," expr)* ")"] ;
tuple_expr       = "(" expr "," expr ("," expr)* ")" ;
record_expr      = "{" field_binding ("," field_binding)* "}" ;
field_binding    = identifier ":" expr ;
field_access     = expr "." identifier ;
```

---

## 4. 패턴 문법 (Pattern Grammar)

```ebnf
pattern = wildcard_pattern
        | variable_pattern
        | literal_pattern
        | constructor_pattern
        | tuple_pattern
        | record_pattern
        ;

wildcard_pattern    = "_" ;
variable_pattern    = identifier ;
literal_pattern     = literal ;
constructor_pattern = type_id ["(" pattern ("," pattern)* ")"] ;
tuple_pattern       = "(" pattern "," pattern ("," pattern)* ")" ;
record_pattern      = "{" field_pattern ("," field_pattern)* [", ..."] "}" ;
field_pattern       = identifier ":" pattern ;
```

---

## 5. 선언 문법 (Declaration Grammar)

### 5.1 함수 선언

```ebnf
func_decl = annotation*
            "func" identifier 
            "(" param_list ")" "->" type
            [contract]
            ":"
            expr
            ;

param_list = [param_decl ("," param_decl)*] ;
param_decl = identifier ":" type ;

contract = requires_clause* ensures_clause* ;
requires_clause = "requires" expr ;
ensures_clause  = "ensures" expr ;
```

### 5.2 타입 선언

```ebnf
type_decl = "type" type_id [type_params] "=" type_def ;

type_params = "<" type_variable ("," type_variable)* ">" ;

type_def = record_type_def
         | sum_type_def
         ;

record_type_def = "{" record_field ("," record_field)* "}" ;

sum_type_def = "|" constructor_def ("|" constructor_def)* ;
constructor_def = type_id ["(" type ("," type)* ")"] ;
```

### 5.3 모듈 선언

```ebnf
module_decl = "module" type_id [module_sig] ":"
              declaration*
              "end"
              ;

module_sig = ":" type_id ;

declaration = func_decl | type_decl ;
```

### 5.4 시그니처 선언

```ebnf
signature_decl = "signature" type_id [type_params] ":"
                 signature_item*
                 "end"
                 ;

signature_item = func_signature | type_signature ;

func_signature = "func" identifier ":" function_type ;
type_signature = "type" type_id [type_params] ;
```

### 5.5 임포트 선언

```ebnf
import_decl = "import" module_path ["as" identifier] ;
module_path = type_id ("." type_id)* ;
```

---

## 6. 메타데이터 문법 (Annotation Grammar)

```ebnf
annotation = "@" annotation_name ["(" annotation_args ")"] ;

annotation_name = "source"
                | "test_case"
                | "generated_from"
                | "reasoning"
                | "inline"
                | "pure"
                | "tailrec"
                ;

annotation_args = annotation_arg ("," annotation_arg)* ;
annotation_arg  = identifier "=" literal ;
```

**예시**:
```
@source("examples/01-factorial.pole", line=3, col=10)
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
@inline
@pure
```

---

## 7. 프로그램 구조 (Program Structure)

```ebnf
program = top_level_item* ;

top_level_item = import_decl
               | type_decl
               | func_decl
               | module_decl
               | signature_decl
               ;
```

---

## 8. 문법 예시

### 8.1 Factorial 함수

```
@source("examples/01-factorial.pole", line=3)
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
```

### 8.2 타입 정의

```
type Option<T> =
  | Some(T)
  | None

type User = {
  name: String,
  email: String,
  age: Nat
}

type ValidationError =
  | NameEmpty
  | NameTooLong
  | InvalidEmail
  | InvalidAge
```

### 8.3 모듈 정의

```
module Math:
  func add (x: Int, y: Int) -> Int :
    x + y

  func mul (x: Int, y: Int) -> Int :
    x * y
end
```

### 8.4 패턴 매칭 예시

```
func validate_name (name: String) -> Result<Unit, ValidationError> :
  let len = String.length name in
  if len < 1 then
    Err(NameEmpty)
  else if len > 50 then
    Err(NameTooLong)
  else
    Ok(())
```

---

## 9. 우선순위 및 결합성

### 9.1 연산자 우선순위 (높음 → 낮음)

1. 함수 적용 (좌결합)
2. 필드 접근 `.` (좌결합)
3. 산술 연산자 `*`, `/` (좌결합)
4. 산술 연산자 `+`, `-` (좌결합)
5. 비교 연산자 `<`, `>`, `<=`, `>=`, `==`, `!=`
6. 논리 연산자 `and` (좌결합)
7. 논리 연산자 `or` (좌결합)
8. 함수 타입 `->` (우결합)
9. Let 바인딩 `let ... in ...`
10. 람다 `\ ... ->`
11. 조건식 `if ... then ... else ...`
12. 패턴 매칭 `match ... with ...`

### 9.2 괄호 규칙

- 괄호로 우선순위 명시 가능
- 타입 표현에서는 `<`, `>` 사용
- 튜플/레코드는 `(`, `)`, `{`, `}` 사용

---

## 10. 공백 및 포매팅

### 10.1 공백 규칙

- 토큰 간 공백/개행 무시
- 들여쓰기는 의미 없음 (가독성을 위해 권장)
- 주석은 공백과 동일하게 처리

### 10.2 권장 스타일

```
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
```

- 함수 선언 후 계약은 들여쓰기
- 본문은 `:` 다음 줄부터 들여쓰기
- 패턴 매칭의 각 arm은 동일한 들여쓰기

---

## 11. 확장성

### 11.1 향후 확장 가능 문법

- 효과 핸들러 (effect handlers)
- 계산 표현식 (computation expressions)
- 매크로 시스템
- 사용자 정의 연산자
- 타입 클래스 (type classes)

### 11.2 예약어

향후 사용을 위해 예약:
```
effect   handler   do      where   class
instance trait     impl    async   await
yield    return    throw   catch   try
```

---

## 12. 문법 완전성 체크리스트

IR 코드가 올바른 문법을 따르는지 확인:

- [ ] 모든 함수에 타입 시그니처 존재
- [ ] 모든 변수 바인딩에 타입 명시
- [ ] 패턴 매칭이 완전함 (모든 경우 커버)
- [ ] 사전/사후 조건이 boolean 표현식
- [ ] 메타데이터 형식 올바름
- [ ] 식별자 네이밍 규칙 준수
- [ ] 괄호 균형 맞음
- [ ] 효과 표시 일관성

---

## 참고 문서

- `specs/implementation-lang.md` - IR 설계 및 타입 시스템
- `specs/syntax-v0.md` - 명세 언어 문법
- `specs/workflow.md` - LLM 변환 워크플로우
- `examples/*.pole-ir` - 구체적 예제 (작성 예정)
