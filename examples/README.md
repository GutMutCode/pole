# Pole 예제 모음

Pole IR 예제 파일들과 각 예제의 설명입니다.

---

## 📚 목차

- [기본 예제](#기본-예제)
- [고급 타입](#고급-타입)
- [FFI 예제](#ffi-예제)
- [SDL2 예제](#sdl2-예제)
- [실행 방법](#실행-방법)

---

## 기본 예제

### 01-factorial.pole-ir
**난이도:** ⭐ 초급  
**개념:** 재귀 함수, 패턴 매칭

```pole-ir
func factorial(n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | _ -> n * factorial(n - 1)
```

**실행:**
```bash
pole run examples/01-factorial.pole-ir factorial 5
# 출력: 120
```

---

### 02-fibonacci.pole-ir
**난이도:** ⭐ 초급  
**개념:** 재귀 함수, 수학적 계산

피보나치 수열의 n번째 항을 계산합니다.

**실행:**
```bash
pole run examples/02-fibonacci.pole-ir fibonacci 10
# 출력: 55
```

---

### 03-user-validation.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Record 타입, Variant 타입, Option, Result

사용자 입력 검증 예제:
- Email 형식 검증
- 나이 범위 검증
- `Result<User, ValidationError>` 반환

```pole-ir
type User = { email: String, age: Nat }
type ValidationError = | InvalidEmail | AgeTooYoung | AgeTooOld

func validate_user(email: String, age: Nat) -> Result<User, ValidationError>
```

---

### 04-simple-math.pole-ir
**난이도:** ⭐ 초급  
**개념:** 다중 함수, 산술 연산

```pole-ir
func abs(x: Int) -> Int
func sum_to_n(n: Nat) -> Nat
```

---

### 05-is-even.pole-ir
**난이도:** ⭐ 초급  
**개념:** Boolean, 조건문

짝수 판별 함수.

---

### 07-max.pole-ir
**난이도:** ⭐ 초급  
**개념:** 비교 연산, if-then-else

두 정수 중 최댓값 반환.

---

## 고급 타입

### 08-simple-record.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Record 타입, 필드 접근

```pole-ir
type Point = { x: Int, y: Int }

func distance_squared(p: Point) -> Int :
  p.x * p.x + p.y * p.y
```

---

### 09-simple-string.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** String 타입, 문자열 전달

---

### 10-string-literal.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** String 리터럴

---

### 11-simple-list.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** List 타입, 리스트 리터럴

```pole-ir
func list_sum(lst: List<Int>) -> Int
```

---

### 12-simple-variant.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Variant 타입 (enum)

```pole-ir
type Color = | Red | Green | Blue

func color_to_code(c: Color) -> Int :
  match c with
  | Red -> 0
  | Green -> 1
  | Blue -> 2
```

---

### 13-variant-tags.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Variant 태그 값

Variant 타입의 내부 표현 (정수 태그) 확인.

---

### 14-option-type.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Option 타입

```pole-ir
type Option<T> = | None | Some(T)
```

---

### 15-simple-option.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** Option 생성자

Some과 None 값 생성.

---

### 16-option-match.pole-ir
**난이도:** ⭐⭐⭐ 고급  
**개념:** Pattern matching, 값 추출

```pole-ir
func unwrap_or(opt: Option<Int>, default: Int) -> Int :
  match opt with
  | None -> default
  | Some(x) -> x
```

---

### 17-unit-type.pole-ir
**난이도:** ⭐ 초급  
**개념:** Unit 타입 (void)

반환값이 없는 함수.

---

### 18-string-length.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** String.length (내장 함수)

문자열 길이 계산.

---

## FFI 예제

### 19-ffi-printf.pole-ir
**난이도:** ⭐⭐ 중급  
**개념:** FFI, @extern, C 함수 호출

C의 printf 함수 호출:

```pole-ir
@extern("printf")
@variadic
func c_printf(format: String) -> Int

func main() -> Int :
  let _ = c_printf("Hello from C!\n") in
  0
```

**실행:**
```bash
cd compiler
cargo run --example test_ffi_printf
```

**참고:** [FFI Tutorial](../docs/tutorials/FFI_TUTORIAL.md)

---

### 20-ffi-malloc.pole-ir
**난이도:** ⭐⭐⭐ 고급  
**개념:** 포인터, malloc/free

동적 메모리 할당:

```pole-ir
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit
```

---

### 22-ffi-pointer.pole-ir
**난이도:** ⭐⭐⭐ 고급  
**개념:** Ptr<T> 타입

포인터를 C 함수에 전달:

```pole-ir
let ptr = c_malloc(1024) in
let _ = c_free(ptr) in
0
```

---

## SDL2 예제

### 23-sdl2-init.pole-ir
**난이도:** ⭐⭐⭐ 고급  
**개념:** SDL2, 초기화/종료

SDL2 라이브러리 초기화 및 정리:

```pole-ir
@extern("SDL_Init")
func SDL_Init(flags: Int) -> Int

@extern("SDL_Quit")
func SDL_Quit(dummy: Unit) -> Unit

func main() -> Int :
  let SDL_INIT_VIDEO = 32 in
  let result = SDL_Init(SDL_INIT_VIDEO) in
  if result == 0 then
    let _ = SDL_Quit(()) in
    0
  else
    1
```

**실행:**
```bash
cd compiler
SDL_VIDEODRIVER=dummy cargo run --example test_sdl2_init
```

---

### 24-sdl2-window.pole-ir
**난이도:** ⭐⭐⭐⭐ 고급  
**개념:** SDL2 윈도우, 리소스 관리

SDL2로 윈도우 생성 및 해제:

```pole-ir
@extern("SDL_CreateWindow")
func SDL_CreateWindow(
  title: String,
  x: Int,
  y: Int,
  w: Int,
  h: Int,
  flags: Int
) -> Ptr<Unit>

@extern("SDL_DestroyWindow")
func SDL_DestroyWindow(window: Ptr<Unit>) -> Unit
```

**특징:**
- 6개 매개변수 함수 호출
- 포인터 반환 및 전달
- 올바른 리소스 정리

**실행:**
```bash
cd compiler
SDL_VIDEODRIVER=dummy cargo run --example test_sdl2_window
```

---

### 25-sdl2-rendering.pole-ir
**난이도:** ⭐⭐⭐⭐ 전문가  
**개념:** SDL2 Renderer, 그래픽 렌더링

SDL2 Renderer로 화면에 픽셀 그리기:

```pole-ir
@extern("SDL_CreateRenderer")
func SDL_CreateRenderer(window: Ptr<Unit>, index: Int, flags: Int) -> Ptr<Unit>

@extern("SDL_SetRenderDrawColor")
func SDL_SetRenderDrawColor(renderer: Ptr<Unit>, r: Int, g: Int, b: Int, a: Int) -> Int

@extern("SDL_RenderDrawPoint")
func SDL_RenderDrawPoint(renderer: Ptr<Unit>, x: Int, y: Int) -> Int

@extern("SDL_RenderPresent")
func SDL_RenderPresent(renderer: Ptr<Unit>) -> Unit
```

**렌더링 내용:**
- 🟥 빨간색 패턴 (100, 100)
- 🟦 파란색 패턴 (200, 200)
- 🟩 녹색 패턴 (300, 300)

**실행:**
```bash
cd compiler
SDL_VIDEODRIVER=dummy cargo run --example test_sdl2_rendering
```

**참고:** [SDL2 Rendering Demo](../docs/SDL2_RENDERING_DEMO.md)

---

### 26-sdl2-interactive.pole-ir
**난이도:** ⭐⭐⭐⭐ 전문가  
**개념:** SDL2 인터랙티브 윈도우, 10초 표시

완전한 그래픽 데모 - 6가지 색상 패턴:

**특징:**
- 10초 동안 윈도우 표시
- 6가지 색상 패턴 (빨강, 파랑, 녹색, 노랑, 시안, 마젠타)
- 실제 GUI 환경에서 윈도우 확인 가능
- 하드웨어 가속 렌더러 사용

**실행 (headless):**
```bash
cd compiler
cargo run --example test_sdl2_interactive
```

**실제 윈도우 보기:**
```bash
/tmp/sdl2_interactive
```

윈도우가 10초간 표시되거나, 수동으로 닫을 수 있습니다.

---

## 실행 방법

### Python 인터프리터로 실행

```bash
# 기본 실행
pole run examples/01-factorial.pole-ir factorial 5

# 테스트 실행
pole test examples/01-factorial.pole-ir
```

### Rust 네이티브 컴파일 (빠름! 🚀)

```bash
cd compiler

# 개별 예제 실행
cargo run --example factorial_native
cargo run --example fibonacci_native
cargo run --example test_ffi_printf

# 모든 예제 테스트
cargo test
```

### 직접 컴파일 (고급)

```bash
cd compiler

# IR → LLVM IR → 네이티브 코드
cargo build --release
./target/release/pole-compiler compile ../examples/01-factorial.pole-ir -o factorial
./factorial 5
```

---

## 난이도 가이드

| 기호 | 난이도 | 설명 |
|------|--------|------|
| ⭐ | 초급 | 기본 문법, 간단한 함수 |
| ⭐⭐ | 중급 | 타입 시스템, 패턴 매칭 |
| ⭐⭐⭐ | 고급 | FFI, 포인터, 외부 라이브러리 |
| ⭐⭐⭐⭐ | 전문가 | 복잡한 시스템 통합 |

---

## 학습 경로

### 1단계: 기본 문법 (1-2시간)
1. `01-factorial.pole-ir` - 재귀 함수
2. `02-fibonacci.pole-ir` - 수학적 재귀
3. `05-is-even.pole-ir` - 조건문
4. `07-max.pole-ir` - if-then-else

### 2단계: 타입 시스템 (2-3시간)
1. `08-simple-record.pole-ir` - Record
2. `12-simple-variant.pole-ir` - Variant
3. `14-option-type.pole-ir` - Option
4. `16-option-match.pole-ir` - Pattern matching
5. `03-user-validation.pole-ir` - 종합 예제

### 3단계: FFI (1-2시간)
1. `19-ffi-printf.pole-ir` - 기본 C 함수 호출
2. `20-ffi-malloc.pole-ir` - 메모리 관리
3. `22-ffi-pointer.pole-ir` - 포인터 전달
4. 📖 [FFI Tutorial](../docs/tutorials/FFI_TUTORIAL.md)

### 4단계: SDL2 (2-3시간)
1. `23-sdl2-init.pole-ir` - SDL2 초기화
2. `24-sdl2-window.pole-ir` - 윈도우 생성
3. (향후) 렌더링, 이벤트 처리

---

## 추가 리소스

### 문서
- [IR 문법 레퍼런스](../specs/ir-syntax.md)
- [FFI 설계 문서](../specs/ffi.md)
- [타입 시스템](../ARCHITECTURE.md)

### 튜토리얼
- [FFI Tutorial](../docs/tutorials/FFI_TUTORIAL.md)
- [Getting Started Guide](../QUICKSTART.md)

### 소스 코드
- Rust 컴파일러: `compiler/src/`
- Python 인터프리터: `src/pole/runtime/`
- 테스트: `compiler/examples/test_*.rs`

---

## 기여하기

새로운 예제를 추가하고 싶으신가요?

1. `.pole-ir` 파일 작성
2. `@test_case(expected=결과)` 추가
3. Rust 테스트 파일 작성 (`compiler/examples/test_*.rs`)
4. 이 README에 문서 추가
5. Pull Request 제출

---

## 라이센스

모든 예제는 프로젝트 라이센스를 따릅니다. 자유롭게 학습 및 수정하세요!
