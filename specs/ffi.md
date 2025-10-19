# Pole FFI (Foreign Function Interface) 설계 v0.1

> C/C++ 라이브러리와의 상호운용성을 위한 FFI 시스템

## 개요

Pole FFI는 Pole 코드에서 C/C++ 라이브러리의 함수를 안전하게 호출할 수 있게 합니다.

**설계 목표:**
1. **타입 안전성**: Pole 타입 시스템과 C ABI 간의 명확한 매핑
2. **최소 오버헤드**: 네이티브 C 호출과 동일한 성능
3. **명시성**: 외부 함수 선언이 명확하게 표시됨
4. **점진적 채택**: 기존 코드와 호환

---

## 1. 기본 문법

### 1.1 외부 함수 선언

```pole-ir
@extern("c_function_name")
func pole_function_name(param1: Type1, param2: Type2) -> ReturnType
```

**예제: printf 호출**

```pole-ir
@extern("printf")
func c_printf(format: String) -> Int

func main() -> Unit :
  let _ = c_printf("Hello from C!\n") in
  ()
```

### 1.2 라이브러리 지정 (옵션)

```pole-ir
@extern("SDL_Init")
@link("SDL2")
func sdl_init(flags: Int) -> Int
```

- `@extern`: C 함수 이름
- `@link`: 링크할 라이브러리 (컴파일 시 `-lSDL2`)

---

## 2. 타입 매핑

### 2.1 기본 타입 매핑

| Pole IR 타입 | C 타입 | 크기 | 설명 |
|--------------|--------|------|------|
| `Int` | `int64_t` | 8 bytes | 64비트 정수 |
| `Nat` | `uint64_t` | 8 bytes | 부호 없는 64비트 정수 |
| `Bool` | `bool` (`_Bool`) | 1 byte | C99 bool |
| `Float64` | `double` | 8 bytes | 64비트 부동소수점 |
| `Unit` | `void` | 0 bytes | 반환값 없음 |
| `String` | `const char*` | 8 bytes (포인터) | NULL 종료 UTF-8 문자열 |

### 2.2 포인터 타입

```pole-ir
// Pole IR에서 명시적 포인터 타입 (Phase 6.2에서 도입 예정)
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>  // void*

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit
```

**M1에서는 제한적 지원:**
- `String` → `const char*` (읽기 전용)
- 명시적 포인터 타입은 Phase 6.2에서 추가

### 2.3 함수 포인터 (Phase 6.1 M3에서 지원)

```pole-ir
// M3: 콜백 지원
type CompareFunc = (Int, Int) -> Int

@extern("qsort")
func c_qsort(
  base: Ptr<Unit>,
  nmemb: Int,
  size: Int,
  compar: CompareFunc
) -> Unit
```

---

## 3. 메모리 안전성

### 3.1 String 전달 규칙

**Pole → C: 안전 (읽기 전용)**

```pole-ir
@extern("puts")
func c_puts(s: String) -> Int

func test() -> Unit :
  let msg = "Hello" in
  let _ = c_puts(msg) in
  ()
```

- Pole `String`은 내부적으로 NULL 종료 `const char*`로 변환
- C 함수는 문자열을 수정하지 않아야 함 (읽기 전용 계약)

**C → Pole: 제한적 (Phase 6.2에서 안전화)**

```pole-ir
@extern("getenv")
func c_getenv(name: String) -> String  // 위험: NULL 가능성

// M1에서는 간단한 케이스만 지원
// M2 이후: Option<String> 또는 Result<String, Error> 반환
```

### 3.2 메모리 수명 관리

**원칙:**
1. Pole이 할당한 메모리는 Pole이 해제
2. C가 할당한 메모리는 명시적으로 해제 (`free` 호출)
3. 스택 변수는 스코프를 벗어나면 자동 해제

**예제: malloc/free 사용**

```pole-ir
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit

func allocate_buffer(size: Int) -> Ptr<Unit> :
  c_malloc(size)

func deallocate_buffer(ptr: Ptr<Unit>) -> Unit :
  c_free(ptr)
```

---

## 4. 가변 인자 함수

### 4.1 제한적 지원 (M1)

```pole-ir
@extern("printf")
@variadic  // 가변 인자 함수 표시
func c_printf(format: String) -> Int
```

**M1 제약:**
- 고정된 인자만 전달 가능
- 타입 안전성 보장 없음 (C printf의 한계)

**안전한 대안 (권장):**

```pole-ir
@extern("puts")
func c_puts(s: String) -> Int

func safe_print(msg: String) -> Unit :
  let _ = c_puts(msg) in
  ()
```

### 4.2 향후 개선 (Phase 6.3)

```pole-ir
// 타입 안전 printf 래퍼 (미래)
func printf(format: String, ...args: Tuple) -> Unit :
  // 컴파일 타임 포맷 검증
  @compile_time_check(format, args)
  c_printf(format, ...args)
```

---

## 5. 구조체 전달 (Phase 6.1 M2)

### 5.1 Record → C struct 매핑

```pole-ir
@repr(C)  // C 메모리 레이아웃 보장
type Point:
  fields:
    - x: Float64
    - y: Float64

@extern("distance")
func c_distance(p1: Point, p2: Point) -> Float64
```

**메모리 레이아웃:**
```c
struct Point {
    double x;  // offset 0
    double y;  // offset 8
};  // total: 16 bytes
```

### 5.2 레이아웃 제어 어노테이션

```pole-ir
@repr(C)
@packed  // 패딩 제거
type Header:
  fields:
    - magic: Int     // 8 bytes
    - version: Int   // 8 bytes
    - flags: Int     // 8 bytes
```

---

## 6. 에러 처리

### 6.1 C 에러 코드 → Result

```pole-ir
@extern("open")
func c_open(path: String, flags: Int) -> Int

func safe_open(path: String) -> Result<Int, String> :
  let fd = c_open(path, 0) in
  if fd < 0 then
    Err("Failed to open file")
  else
    Ok(fd)
```

### 6.2 NULL 포인터 → Option

```pole-ir
@extern("getenv")
func c_getenv_raw(name: String) -> Ptr<Unit>

func getenv(name: String) -> Option<String> :
  let ptr = c_getenv_raw(name) in
  if is_null(ptr) then
    None
  else
    Some(ptr_to_string(ptr))
```

---

## 7. 링킹

### 7.1 정적 링크

```bash
# Pole 컴파일러가 자동으로 libc 링크
pole compile examples/19-ffi-printf.pole-ir -o output
# 내부적으로: clang output.o -o output
```

### 7.2 외부 라이브러리 링크

```pole-ir
@extern("SDL_Init")
@link("SDL2")  // -lSDL2 플래그 추가
func sdl_init(flags: Int) -> Int
```

```bash
pole compile game.pole-ir -o game --link SDL2
# 내부적으로: clang game.o -lSDL2 -o game
```

### 7.3 헤더 포함 (Phase 6.1 M4)

```pole-ir
@header("SDL2/SDL.h")
@extern("SDL_CreateWindow")
func sdl_create_window(
  title: String,
  x: Int, y: Int,
  w: Int, h: Int,
  flags: Int
) -> Ptr<Unit>
```

---

## 8. 예제: M1 데모

### 8.1 Hello from C (printf)

**파일: `examples/19-ffi-printf.pole-ir`**

```pole-ir
@extern("printf")
func c_printf(format: String) -> Int

func main() -> Unit :
  let _ = c_printf("Hello from C!\n") in
  ()

@test_case(expected=())
```

**실행:**
```bash
$ pole compile examples/19-ffi-printf.pole-ir -o hello
$ ./hello
Hello from C!
```

### 8.2 동적 메모리 할당 (malloc/free)

**파일: `examples/20-ffi-malloc.pole-ir`**

```pole-ir
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit

@extern("printf")
func c_printf(format: String) -> Int

func test_malloc() -> Unit :
  let size = 1024 in
  let buffer = c_malloc(size) in
  let _ = c_printf("Allocated 1024 bytes\n") in
  let _ = c_free(buffer) in
  let _ = c_printf("Freed buffer\n") in
  ()

@test_case(expected=())
```

---

## 9. 구현 계획

### Phase 6.1 M1: 간단한 C 함수 호출

**구현 작업:**

1. **IR 문법 확장**
   - `@extern(name)` 어노테이션 추가
   - `@link(lib)` 어노테이션 추가 (옵션)
   - `@variadic` 어노테이션 추가 (가변 인자)

2. **IR Parser 확장**
   - 어노테이션 파싱 (`@extern`, `@link`)
   - AST 노드: `ExternFunctionDecl`

3. **Type Checker 확장**
   - 외부 함수 타입 검증
   - C ABI 호환 타입만 허용

4. **CodeGen 확장**
   - `declare_libc_functions()` 제거 (하드코딩)
   - IR의 `@extern` 선언에서 동적 생성
   - LLVM `module.add_function()` 호출

5. **테스트**
   - `examples/19-ffi-printf.pole-ir`
   - `examples/20-ffi-malloc.pole-ir`

### Phase 6.1 M2: 구조체 전달

- `@repr(C)` 어노테이션
- Record → C struct 메모리 레이아웃 매핑
- 포인터 타입 도입

### Phase 6.1 M3: 콜백 지원

- Pole 함수 → C 함수 포인터 변환
- Closure 캡처 제한 (stateless만 지원)

### Phase 6.1 M4: SDL2 윈도우

- SDL2 바인딩 작성
- 이벤트 루프 구현
- 실제 그래픽 윈도우 데모

---

## 10. 제약사항 및 향후 계획

### 10.1 M1 제약사항

**지원하지 않음:**
- 포인터 타입 (`Ptr<T>`) - M2에서 추가
- 구조체 전달 - M2에서 추가
- 콜백 함수 - M3에서 추가
- 가변 인자 타입 안전성 - Phase 6.3

**지원:**
- 기본 타입 (Int, Bool, Float64, String)
- 간단한 libc 함수 (printf, puts, malloc, free)
- 외부 함수 선언 (`@extern`)

### 10.2 향후 개선 (Phase 6.2+)

- **명시적 포인터 타입**: `Ptr<T>`, `MutPtr<T>`
- **Unsafe 블록**: `unsafe { ... }` 명시
- **메모리 레이아웃 제어**: `@packed`, `@align(N)`
- **Zero-cost 추상화**: 인라인 래퍼 최적화

---

## 11. 참고 자료

### 11.1 기존 FFI 시스템 비교

| 언어 | 외부 함수 선언 | 타입 안전성 | 특징 |
|------|---------------|-------------|------|
| Rust | `extern "C" fn foo()` | 높음 | unsafe 블록 필수 |
| Zig | `extern fn foo()` | 높음 | 컴파일 타임 C 헤더 파싱 |
| Swift | `@_silgen_name("foo")` | 중간 | Optional 활용 |
| Pole | `@extern("foo") func` | 높음 (목표) | LLM 친화적 문법 |

### 11.2 설계 결정 근거

**Q: 왜 `@extern` 어노테이션인가?**
- LLM이 생성하기 쉬움 (명시적 메타데이터)
- 파싱이 간단 (어노테이션 블록)
- Pole 함수와 구분 명확

**Q: 왜 String → `const char*`인가?**
- Pole IR은 불변 문자열 사용
- C 함수의 99%는 읽기 전용 문자열 요구
- 안전성 향상 (수정 방지)

**Q: Ptr<T> 타입을 M1에서 제외한 이유?**
- 복잡도 감소 (M1은 최소 구현)
- String으로 printf/puts 데모 가능
- M2에서 포인터 시스템 전체 설계

---

## 12. 성공 기준

**Phase 6.1 M1 완료 조건:**

1. ✅ `@extern` 어노테이션 파싱 성공
2. ✅ printf 호출 예제 컴파일 성공
3. ✅ "Hello from C!" 출력 확인
4. ✅ malloc/free 예제 메모리 누수 없음 (Valgrind)
5. ✅ 기존 18개 예제 정상 작동 (회귀 테스트)

---

**문서 버전:** v0.1  
**작성일:** 2025-10-19  
**상태:** Phase 6.1 M1 설계
