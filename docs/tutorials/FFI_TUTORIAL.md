# Pole FFI Tutorial: C 라이브러리 호출하기

> Pole에서 C 함수를 호출하여 SDL2, OpenGL 등의 라이브러리를 사용하는 방법

**작성일:** 2025-10-19  
**난이도:** 중급  
**소요 시간:** 30분

---

## 목차

1. [개요](#개요)
2. [기본 C 함수 호출](#기본-c-함수-호출)
3. [문자열 전달하기](#문자열-전달하기)
4. [포인터 타입 사용](#포인터-타입-사용)
5. [SDL2 윈도우 만들기](#sdl2-윈도우-만들기)
6. [에러 처리](#에러-처리)
7. [베스트 프랙티스](#베스트-프랙티스)

---

## 개요

**FFI (Foreign Function Interface)**는 Pole 코드에서 C 라이브러리의 함수를 호출할 수 있게 해주는 기능입니다.

### 지원되는 기능

| 기능 | 상태 | 예제 |
|------|------|------|
| 기본 C 함수 호출 | ✅ | `printf`, `puts` |
| 가변 인자 함수 | ✅ | `printf` |
| 포인터 전달/반환 | ✅ | `malloc`, `SDL_CreateWindow` |
| 문자열 전달 | ✅ | `Pole String → C char*` |
| 구조체 전달 | ✅ | `Pole Record → C struct` |
| 여러 매개변수 | ✅ | `SDL_CreateWindow` (6개) |
| 콜백 함수 | ❌ | 향후 지원 예정 |

---

## 기본 C 함수 호출

가장 간단한 예제부터 시작합니다: C의 `puts` 함수 호출

### Step 1: 외부 함수 선언

```pole-ir
@extern("puts")
func c_puts(s: String) -> Int
```

**설명:**
- `@extern("puts")`: C에서의 함수 이름
- `c_puts`: Pole에서 사용할 이름
- `String`: Pole 타입 (자동으로 `char*`로 변환됨)
- `Int`: 반환 타입 (puts는 int를 반환)

### Step 2: 함수 호출

```pole-ir
func main() -> Int :
  let _ = c_puts("Hello from Pole!") in
  0
```

### Step 3: 컴파일 및 실행

```bash
cd compiler
cargo run --example test_ffi_printf
```

**출력:**
```
Hello from Pole!
```

### 전체 예제

```pole-ir
@extern("puts")
func c_puts(s: String) -> Int

func main() -> Int :
  let _ = c_puts("Hello from Pole!") in
  let _ = c_puts("FFI is working!") in
  0

@test_case(expected=0)
```

📁 **파일:** `examples/simple-ffi.pole-ir`

---

## 문자열 전달하기

Pole의 `String` 타입은 자동으로 C의 `char*`로 변환됩니다.

### printf 사용하기

```pole-ir
@extern("printf")
@variadic
func c_printf(format: String) -> Int

func greet(name: String) -> Int :
  let _ = c_printf("Hello, %s!\n") in
  0
```

**주의사항:**
- `@variadic`: 가변 인자 함수임을 표시
- **현재 제한:** 추가 인자는 지원되지 않음 (문자열 포맷팅만 가능)

### String 함수 활용

```pole-ir
@extern("strlen")
func c_strlen(s: String) -> Int

func string_length(s: String) -> Int :
  c_strlen(s)
```

---

## 포인터 타입 사용

C 함수는 종종 포인터를 반환하거나 매개변수로 받습니다.

### Ptr<T> 타입

Pole은 `Ptr<T>` 타입으로 C 포인터를 표현합니다:

```pole-ir
Ptr<Unit>     // void*
Ptr<Int>      // int*
Ptr<String>   // char**
```

### malloc/free 예제

```pole-ir
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit

func test_memory() -> Int :
  let ptr = c_malloc(1024) in
  let _ = c_free(ptr) in
  0
```

**설명:**
- `c_malloc(1024)`: 1KB 메모리 할당
- `Ptr<Unit>`: void* 포인터 (타입이 지정되지 않음)
- `c_free(ptr)`: 메모리 해제

### 포인터 안전성

⚠️ **주의:**
- Pole은 현재 포인터 역참조를 지원하지 않음
- 포인터는 C 함수에 전달만 가능
- 메모리 안전성은 프로그래머 책임

---

## SDL2 윈도우 만들기

실제 그래픽 프로그램을 만들어봅시다.

### Step 1: SDL2 함수 선언

```pole-ir
@extern("SDL_Init")
func SDL_Init(flags: Int) -> Int

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

@extern("SDL_Quit")
func SDL_Quit(dummy: Unit) -> Unit
```

**주의:** SDL_Quit는 매개변수가 없지만, Pole의 커링 스타일 때문에 `dummy: Unit`이 필요합니다.

### Step 2: SDL2 상수 정의

```pole-ir
func main() -> Int :
  let SDL_INIT_VIDEO = 32 in
  let SDL_WINDOWPOS_UNDEFINED = 536805376 in
  let SDL_WINDOW_HIDDEN = 8 in
  ...
```

### Step 3: 윈도우 생성 로직

```pole-ir
func main() -> Int :
  let SDL_INIT_VIDEO = 32 in
  let init_result = SDL_Init(SDL_INIT_VIDEO) in
  
  if init_result == 0 then
    let SDL_WINDOWPOS_UNDEFINED = 536805376 in
    let SDL_WINDOW_HIDDEN = 8 in
    let window = SDL_CreateWindow(
      "My Pole Window",
      SDL_WINDOWPOS_UNDEFINED,
      SDL_WINDOWPOS_UNDEFINED,
      800,
      600,
      SDL_WINDOW_HIDDEN
    ) in
    
    let _ = SDL_DestroyWindow(window) in
    let _ = SDL_Quit(()) in
    0
  else
    1
```

### Step 4: 컴파일 및 실행

```bash
cd compiler
SDL_VIDEODRIVER=dummy cargo run --example test_sdl2_window
```

**출력:**
```
Initializing SDL2 with dummy video driver...
Creating invisible window...
Window created successfully!
Destroying window...
Done!
```

📁 **전체 코드:** `examples/24-sdl2-window.pole-ir`

---

## 에러 처리

C 함수는 보통 에러를 반환값으로 표시합니다.

### 패턴 1: 0 = 성공

```pole-ir
let result = SDL_Init(SDL_INIT_VIDEO) in
if result == 0 then
  // 성공
  ...
else
  // 실패
  ...
```

### 패턴 2: NULL 포인터 체크

⚠️ **현재 제한:** Pole은 NULL 체크를 직접 지원하지 않습니다.

**해결책:**
- C 함수 결과를 항상 사용하기 전에 확인
- 실패 시 프로그램 종료

```pole-ir
let window = SDL_CreateWindow(...) in
// window가 NULL일 수 있지만 체크 불가
// SDL2는 실패 시 에러 로그를 출력하므로 의존
```

---

## 베스트 프랙티스

### 1. 함수 이름 규칙

```pole-ir
// ❌ 나쁨: 원본 이름 그대로
@extern("printf")
func printf(s: String) -> Int

// ✅ 좋음: c_ 접두사로 명확히
@extern("printf")
func c_printf(s: String) -> Int
```

### 2. 리소스 정리

```pole-ir
// ✅ 좋음: 항상 정리
let window = SDL_CreateWindow(...) in
let _ = do_something(window) in
let _ = SDL_DestroyWindow(window) in  // 반드시 해제
let _ = SDL_Quit(()) in
0
```

### 3. 에러 처리

```pole-ir
// ✅ 좋음: 에러 확인
let result = risky_c_function() in
if result == 0 then
  // 성공 경로
  ...
else
  // 실패 경로
  ...
```

### 4. 문자열 상수

```pole-ir
// ✅ 좋음: 리터럴 사용
SDL_CreateWindow("My Game", ...)

// ❌ 피할 것: 복잡한 문자열 조작
// (현재 지원 제한)
```

### 5. 타입 안전성

```pole-ir
// ✅ 좋음: 타입 명시
func c_malloc(size: Int) -> Ptr<Unit>

// ❌ 나쁨: 타입 생략 불가
```

---

## 다음 단계

### 학습 경로

1. ✅ **이 튜토리얼** - FFI 기본
2. 📖 **SDL2 Tutorial** - 그래픽 프로그래밍
3. 📖 **Advanced FFI** - 콜백 및 복잡한 구조체 (향후)

### 예제 코드

전체 작동하는 예제는 `examples/` 디렉토리를 참고하세요:

- `19-ffi-printf.pole-ir` - printf 호출
- `20-ffi-malloc.pole-ir` - 메모리 할당
- `22-ffi-pointer.pole-ir` - 포인터 전달
- `23-sdl2-init.pole-ir` - SDL2 초기화
- `24-sdl2-window.pole-ir` - SDL2 윈도우

### 참고 문서

- [IR 문법 레퍼런스](../specs/ir-syntax.md)
- [FFI 설계 문서](../specs/ffi.md)
- [타입 시스템](../specs/types.md)

---

## 자주 묻는 질문 (FAQ)

### Q: 왜 SDL_Quit에 `dummy: Unit` 매개변수가 필요한가요?

A: Pole은 커링 스타일 함수 호출을 사용합니다. 매개변수가 없는 함수는 현재 지원되지 않으므로, Unit 타입의 더미 매개변수를 추가합니다.

### Q: 포인터를 역참조할 수 있나요?

A: 현재는 불가능합니다. Phase 6.2에서 `*ptr` 연산자가 추가될 예정입니다.

### Q: 콜백 함수는 어떻게 전달하나요?

A: 아직 지원되지 않습니다. Phase 6.1 M3에서 추가될 예정입니다.

### Q: C 구조체를 Pole로 가져올 수 있나요?

A: Pole Record 타입은 C struct와 ABI 호환됩니다:

```pole-ir
type Point = { x: Int, y: Int }  // C: struct { int64_t x, y; }
```

### Q: 여러 라이브러리를 동시에 사용할 수 있나요?

A: 네, 각 함수에 `@extern` 어노테이션만 추가하면 됩니다. 링크 시 `-l` 플래그로 라이브러리를 지정합니다.

```bash
cc output.o -lSDL2 -lGL -lm -o game
```

---

## 마무리

이 튜토리얼을 완료하셨습니다! 이제 다음을 할 수 있습니다:

- ✅ C 함수 호출
- ✅ 문자열과 포인터 전달
- ✅ SDL2 라이브러리 사용
- ✅ 기본적인 에러 처리

**다음 단계:** [SDL2 Window Tutorial](SDL2_WINDOW_TUTORIAL.md)에서 실제 그래픽 렌더링을 배워보세요!

---

**피드백:** 이 튜토리얼에 대한 질문이나 개선 제안은 [GitHub Issues](https://github.com/pole-lang/pole/issues)에 남겨주세요.
