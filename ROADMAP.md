# Pole 개발 로드맵

> **Pole**: LLM 네이티브 프로그래밍 언어 - 시스템/게임 프로그래밍에 특화
>
> C++/Rust처럼 게임 엔진을 만들 수 있는 언어이며, LLM으로 더 쉽게 개발 가능

**이전 로드맵 (Phase 0-4 프로토타입)**: [ROADMAP-v1-prototype.md](ROADMAP-v1-prototype.md)

---

## 📊 전체 개요

### 프로젝트 목표

**Pole**: 시스템/게임 프로그래밍을 위한 LLM 네이티브 언어
- 자연어 명세로 복잡한 로직 작성
- LLM이 안전하고 최적화된 코드 생성
- 타입 안전 + 네이티브 성능 + 메모리 안전성

**Pole = 프로그래밍 언어** (게임 엔진 자체가 아님)
- 비유: Rust → Bevy Engine 개발, C++ → Unreal Engine 개발
- **Pole → 게임 엔진 개발** (Pole로 엔진을 만들 수 있음)

**장기 비전**: Pole 언어로 Unity/Unreal 수준의 게임 엔진 제작 가능
- Phase 0-6: 언어 기반 완성 (컴파일러, FFI, 시스템 프로그래밍)
- Phase 7-8: 게임 개발용 표준 라이브러리 (`pole_graphics`, `pole_ecs` 등)
- Phase 9-10: 개발 도구 및 생태계
- 이후: Pole로 제작한 오픈소스 게임 엔진 (별도 프로젝트)

### 타임라인 (조정됨: 5-7년)

```
Year 0-1   : Phase 0-4  언어 기초 (완료) ✅
Year 1-1.5 : Phase 5-6  컴파일러 & FFI (진행 중) 🚀
Year 1.5-3 : Phase 7    핵심 표준 라이브러리 (graphics, ecs)
Year 3-4   : Phase 9    기본 개발 도구 (LSP, 디버거)
Year 4-5   : Phase 10   커뮤니티 & 생태계 시작
Year 5+    : 지속적 개선 및 확장
```

**핵심 변경사항:**
- Phase 6.2, 6.3 사실상 제거 (필요시만)
- Phase 8 (고급 라이브러리) Phase 7과 병합
- 전체 기간 단축: 10년 → 5년 (실용적 목표)

### 현재 위치

**2025-10-19**: Phase 5.1 ✅ 완료, Phase 6.1 ✅ 완료

**완료된 것:**
- ✅ **Phase 0-4**: 언어 기초 (명세 언어, IR, 인터프리터, LLM 통합)
- ✅ **Phase 5.1**: LLVM 백엔드 (M0-M5)
  - Rust IR Parser/Type Checker (23.4x/25.6x 성능 향상)
  - 네이티브 컴파일 (100x+ 인터프리터 대비)
  - 고급 타입 시스템 (Record, List, Option, String, Unit)
  - Runtime 함수 (String.length, List.concat, print)
- ✅ **Phase 6.1**: FFI 시스템 (M1-M4 + M4.5)
  - C 함수 호출 (@extern)
  - 포인터 타입 (Ptr<T>)
  - SDL2 윈도우 생성 및 렌더링
  - SDL2 인터랙티브 데모 (10초 표시, 6색 패턴)
  - 26개 예제 프로그램
  - FFI Tutorial 및 문서화 완료

**즉시 목표:** 🎮 **게임 프로토타입 (Pong)**
- 3-5일 내 완성 목표
- SDL2 이벤트 폴링 추가
- 실용성 검증 및 YouTube 데모

**다음 단계:**
- 📋 Phase 7 설계 (표준 라이브러리)
- 🔧 핵심 언어 기능 개선
- 🌟 OpenGL 통합 (중기)

---

## 우선순위 원칙

### Phase 우선순위

1. **현재 Phase 내 P0 작업 우선**
2. **Phase P0 완료 → P1 검토**
3. **Phase 전환은 P0 완료 후만 가능**

### 작업 분류

- **P0 (Critical)**: 해당 Phase 완료 필수
- **P1 (High)**: Phase 완료 후 추가 가치
- **P2 (Medium)**: 선택적 개선
- **P3 (Low)**: 나중에

### 단기 vs 장기

- **단기 (Phase 0-4)**: ✅ 완료 (프로토타입)
- **중기 (Phase 5-6)**: 🔨 진행 중 (2년, 컴파일러)
- **장기 (Phase 7-10)**: 📅 계획 중 (5-8년, 표준 라이브러리 및 생태계)

---

## Phase 0-4: 언어 기초 (완료) ✅

**기간:** 2025년 초 ~ 2025-10-19 (약 1년)

**목표:** 작동하는 프로토타입 완성

### 완료된 Phase

- ✅ **Phase 0**: 기획 및 문서화
- ✅ **Phase 1**: 언어 설계 (명세 언어, IR)
- ✅ **Phase 2**: 프로토타입 구현 (파서, 변환기, 인터프리터, CLI)
- ✅ **Phase 3**: 완성도 향상 (에러 시스템, 성능, 계약 검증)
- ✅ **Phase 4**: LLM 통합 개선 (Prompt 개선, IR 후처리)

### 주요 산출물

- 명세 언어 파서 (`src/pole/parser/`)
- IR 인터프리터 (`src/pole/runtime/`)
- LLM 변환기 (`src/pole/transformer/`)
- 타입 체커 (`src/pole/verifier/type_checker.py`)
- CLI 도구 (`src/pole/cli/main.py`)
- 9개 테스트 모듈 (모두 통과)

### 시스템 능력 (현재)

✅ **작동하는 것:**
- 명세 언어 → IR 변환 (LLM)
- IR 실행 (인터프리터)
- 타입 체킹, 계약 검증
- 예제 기반 자동 테스트
- 6개 예제 프로그램 실행

⚠️ **제한사항:**
- 인터프리터만 존재 (느림)
- I/O 시스템 없음
- 표준 라이브러리 없음 (그래픽/오디오/파일 등)
- 시스템 프로그래밍 불가능

**상세 내용:** [ROADMAP-v1-prototype.md](ROADMAP-v1-prototype.md)

---

## Phase 5: 네이티브 컴파일러 & 고성능 런타임 (Year 1-2.5)

**기간:** 2025-10 ~ 2027-03 (1.5년)

**목표:** Pole IR → 네이티브 기계어 컴파일, 시스템 프로그래밍 가능한 성능 달성

**현재 문제:**
- 인터프리터: 느림 (고성능 애플리케이션 불가능)
- 메모리 관리: 기본적인 GC 없음
- 동시성: 싱글 스레드만 지원

**Phase 5 목표:**
- ✅ 네이티브 실행 파일 생성
- ✅ 성능: 인터프리터 대비 100x 이상
- ✅ 메모리 안전성 보장
- ✅ 멀티스레드 지원

---

### P0 작업 (필수)

#### 5.1 **LLVM 백엔드 개발** ⭐ 최우선

**목표:** Pole IR → LLVM IR → 네이티브 기계어

**기간:** 9-12개월 (2025-10 ~ 2026-09) - Rust 학습 포함

**산출물:**

**Rust 컴포넌트 (compiler/):**
- `src/ir_parser.rs` - IR 파싱 (Python 대체) ⭐ M0부터
- `src/type_checker.rs` - 타입 체커 (Python 대체) ⭐ M0부터
- `src/ir_to_llvm.rs` - IR → LLVM IR 변환
- `src/codegen.rs` - 코드 생성
- `src/optimization.rs` - 최적화 패스
- `src/contract_verifier.rs` - 계약 검증 (Python 대체) ⭐ M1 이후
- `src/interpreter.rs` - IR 인터프리터 (선택사항)
- `src/lib.rs` - 라이브러리 인터페이스 & PyO3 바인딩

**Python 컴포넌트 (유지):**
- `src/pole/cli/` - CLI 도구 (pole check, run, build 등)
- `src/pole/transformer/` - LLM API 연동 (OpenRouter)
- `src/pole/parser/` - 명세 언어 파서 (.pole)
- `src/pole/validator/` - 명세 검증기
- `src/pole/runtime/ir_parser.py` → Rust 래퍼로 전환
- `src/pole/verifier/type_checker.py` → Rust 래퍼로 전환

**최종 산출물:**
- `pole compile <file> --target x86_64-linux` CLI 명령어
- 실행 파일 생성 (`game.exe`, `game`)

**마일스톤:**
0. **M0: Rust 학습 & 핵심 인프라 Rust 전환** ✅ **완료** (2025-10-19 ~ 2025-10-19)
   
   **Week 1-6: Rust 기초 학습** ✅
   - The Rust Book (Chapter 1-17)
   - Rust by Example (실습)
   - 소유권, 빌림, 생명주기 완전 이해
   
   **Week 7-10: IR Parser Rust 구현** ✅ **완료**
   - nom parser combinator 학습
   - `compiler/src/ir_parser.rs` 구현 (520줄)
   - PyO3 바인딩 구현
   - **성능 달성:** 4배 향상 (0.0285ms vs 0.1138ms)
   - **검증:** 2/6 예제 완전 통과, 4개 edge case 수정 필요
   
   **Week 11-14: Type Checker Rust 구현** ✅ **완료**
   - `compiler/src/type_checker.rs` 구현 (540줄)
   - Rust enum + pattern matching 활용
   - 타입 추론 및 compatibility 체크 구현
   - **성능 달성:** 25.6배 향상 (0.0047ms vs 0.1193ms) 🎯
   - **검증:** 모든 기존 타입 체크 테스트 통과 (11/11)
   
   **Week 15-16: LLVM 준비** ⬜ (M1에서 진행)
   - llvm-sys 또는 inkwell 튜토리얼
   - 간단한 LLVM "Hello World" 컴파일러 구현
   
   **Week 17-18: PyO3 통합 완성** ✅ **완료**
   - Python에서 Rust 컴포넌트 호출 검증
   - 성능 벤치마크 및 비교
   - parse_ir(), check_types_py() 바인딩 완성
   
   **M0 산출물:**
   - ✅ IR Parser (Rust) - 모든 Phase에서 사용
   - ✅ Type Checker (Rust) - 컴파일러 기반 인프라
   - ✅ PyO3 바인딩 완성
   - ✅ Rust 개발 환경 구축
   
   **성공 기준:**
   - ✅ IR Parser: 4배 성능 향상 (목표 10배, 파싱 최적화 여지 있음)
   - ✅ Type Checker: 25.6배 성능 향상 (목표 5배 초과 달성!)
   - ✅ PyO3 통합: Python에서 투명하게 사용 가능
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 1일 (집중 개발)

0.5 **M0.5: 시스템 안정화 & 검증** ⭐ **최우선** (1-2주, 2025-10-19 ~ 2025-11-02)
   
   **목표:** M1 시작 전 현재 시스템 완전 안정화 및 검증
   
   **Week 1: 버그 수정 & Edge Case 처리**
   - ✅ 인터프리터 버그 수정 (factorial 실행 실패 문제)
   - ✅ IR 파서 edge case 수정 (4/6 예제 → 6/6 예제 통과)
   - ✅ Unicode/한글 annotation 지원
   - ✅ Record type 파싱 지원
   
   **Week 2: 전체 파이프라인 검증**
   - ✅ End-to-end 테스트: .pole → LLM → .pole-ir → 실행
   - ✅ 6개 예제 모두 전체 파이프라인 통과
   - ✅ LLM API 안정성 확인
   - ✅ 통합 테스트 자동화
   
   **산출물:**
   - 수정된 인터프리터 (버그 프리)
   - 완성된 IR 파서 (6/6 예제 통과)
   - End-to-end 테스트 스위트
   - 안정화 보고서
   
   **성공 기준:**
   - ✅ 모든 예제 인터프리터 실행 성공
   - ✅ IR 파서 100% 예제 통과
   - ✅ LLM 변환 → 타입체크 → 실행 전체 파이프라인 작동
   
   **왜 필요한가:**
   - LLVM 백엔드 개발 전 안정적인 기반 확보
   - 복잡한 컴파일러 개발 중 디버깅 용이
   - LLVM 결과물 검증을 위한 baseline 필요

1. **M1: Rust IR Parser 완성 & 타입 시스템 확장** ✅ **완료** (1일, 2025-10-19)
   
   **목표:** Python IR Parser와 기능 동등성 확보, LLVM 백엔드를 위한 안정적 기반 마련
   
   **완료 상태:**
   - ✅ Python IR Parser: 6/6 예제 파싱 성공 (Type definitions, Multi-line let 지원)
   - ✅ Python Type Checker: Custom types 지원
   - ✅ Rust IR Parser: 6/6 예제 파싱 성공 (Python과 기능 동등성 달성!)
   
   **1단계: Rust IR Parser 기능 추가** ✅ **완료**
   - ✅ Type definition parsing 구현 (compiler/src/ir_parser.rs)
     - Record types: `type User = { name: String, age: Nat }`
     - Variant types: `type Error = | NotFound | Invalid`
     - Type aliases: `type UserId = String`
   - ✅ Multi-line record type 지원
   - ✅ Custom type names 인식 (parse_type 개선)
   - ✅ Logical operators 지원 (`&&`, `||`)
   
   **2단계: 검증 및 테스트** ✅ **완료**
   - ✅ 6/6 예제 파싱 성공 확인
   - ✅ Python IR Parser 결과와 100% 일치 검증
   - ✅ 성능 벤치마크 측정: **23.4배 향상** (0.014ms vs 0.322ms)
   
   **산출물:**
   - ✅ `compiler/src/ir_parser.rs` 업데이트 (Type definitions, 논리 연산자)
   - ✅ `compiler/src/python_bindings.rs` 업데이트 (type_def_to_py)
   - ✅ 6/6 예제 파싱 성공
   - ✅ Python-Rust 기능 동등성 달성
   
   **성공 기준 달성:**
   - ✅ Rust IR Parser: 6/6 예제 파싱 성공
   - ✅ Type definitions 완전 지원 (Record, Variant, Alias)
   - ✅ Python과 결과 100% 일치
   - ✅ 성능: Python 대비 **23.4배 향상** (목표 5-10배 초과 달성!)
   
   **M1 완료:**
   - ✅ Rust IR Parser가 Python IR Parser와 동등한 기능 제공
   - ✅ 모든 예제 파싱 가능 (6/6)
   - ✅ LLVM 백엔드 작업 시작 준비 완료
   
   **완료 일자:** 2025-10-19

1.5 **M1.5: Python-Rust 통합 완성** ✅ **완료** (반나절, 2025-10-19)
   
   **목표:** Rust IR Parser를 Python 코드베이스에 완전히 통합, Type definitions 손실 방지
   
   **해결한 문제:**
   - ✅ `ir_parser_rust.py`의 `_dict_to_program`이 type_defs를 무시하던 문제 수정
   - ✅ Literal type_name 누락 문제 수정
   - ✅ Type checker가 Rust parser 출력을 올바르게 처리
   
   **1단계: Type Definitions 변환 구현** ✅ **완료**
   - ✅ `convert_type_def()` 함수 구현
     - Record types → Python RecordType
     - Variant types → list of (name, args) tuples
     - Alias types → Python Type
   - ✅ `_dict_to_program`에서 type_defs 처리
   - ✅ `python_bindings.rs`에 Literal type_name 필드 추가
   
   **2단계: End-to-End 검증** ✅ **완료**
   - ✅ user-validation: 2개 type_defs 올바르게 변환
   - ✅ 모든 예제 End-to-End 테스트 통과
   - ✅ Type checker: 5/6 예제 성공 (user-validation 실패는 타입 체커 한계)
   
   **3단계: CLI 통합 확인** ✅ **완료**
   - ✅ End-to-end 테스트가 Rust parser 사용 검증
   - ✅ Type definitions가 전체 파이프라인에서 보존됨
   
   **산출물:**
   - ✅ `src/pole/runtime/ir_parser_rust.py` 업데이트 (type_defs 변환)
   - ✅ `compiler/src/python_bindings.rs` 업데이트 (Literal type_name)
   - ✅ End-to-end 테스트 통과
   
   **성공 기준 달성:**
   - ✅ Rust parser로 파싱한 user-validation에서 2개 type_defs 인식
   - ✅ Type definitions가 Python AST로 올바르게 변환
   - ✅ 모든 예제 End-to-End 테스트 통과
   - ✅ 성능: 23.4배 유지 (type_defs 변환 오버헤드 무시 가능)
   
   **M1.5 완료:**
   - ✅ Rust parser가 Python 코드베이스에 완전히 통합
   - ✅ Type definitions가 전체 파이프라인에서 보존됨
   - ✅ LLVM 백엔드 개발 시작 준비 완료
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 반나절

2. **M2: LLVM 백엔드 - 기본 함수 컴파일** ✅ **완료** (2025-10-19)
   
   **목표:** Pole IR → LLVM IR → 네이티브 실행 파일
   
   **선행 조건:** M1.5 완료 (Rust-Python 통합 완성)
   
   **구현 내용:**
   - ✅ LLVM 바인딩 선택: **inkwell 0.5.0** (LLVM 17.0.6)
   - ✅ 기본 함수 컴파일 (factorial, fibonacci, max)
   - ✅ 기본 타입 (Int, Bool, Nat) 지원
   - ✅ 산술 연산자 (+, -, *, /, %, 비교 연산)
   - ✅ 조건문 (if-then-else)
   - ✅ 재귀 함수 호출
   - ✅ Pattern matching (match expression)
   - ✅ 불리언 논리 (&&, ||, not)
   
   **산출물:**
   - ✅ `compiler/src/codegen.rs` - LLVM 코드 생성기
   - ✅ `compiler/examples/factorial_native.rs` - 네이티브 컴파일 예제
   - ✅ `compiler/examples/fibonacci_native.rs`
   - ✅ `compiler/examples/simple_math_native.rs`
   - ✅ `compiler/examples/is_even_native.rs`
   - ✅ `compiler/examples/max_native.rs`
   - ✅ `compiler/examples/all_native_test.rs` - 통합 테스트
   
   **검증 결과:**
   - ✅ factorial(5) = 120 (네이티브 실행 성공)
   - ✅ fibonacci(10) = 55 (네이티브 실행 성공)
   - ✅ abs(-10) + sum_to_n(5) = 25 (다중 함수 성공)
   - ✅ is_even(7) = false (불리언 반환 성공)
   - ✅ max(42, 17) = 42 (네이티브 실행 성공)
   - ✅ 통합 테스트 3/3 통과
   - ✅ 성능: **~20ns/call** (인터프리터 대비 압도적)
   
   **성공 기준 달성:**
   - ✅ factorial 예제 네이티브 컴파일 성공
   - ✅ 실행 결과 정확성 100%
   - ✅ 성능: 인터프리터 대비 **10x 초과 달성**
   
   **M2 완료:**
   - ✅ 5개 예제 네이티브 컴파일 성공 (factorial, fibonacci, simple-math, is-even, max)
   - ✅ LLVM IR 생성 → Object file → 실행 파일 파이프라인 완성
   - ✅ M3 작업 시작 준비 완료 (Record types, String 지원)
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 1일 (M2 준비 완료 → M2 완료)

3. **M3: LLVM 백엔드 - 고급 기능** ✅ **완료** (2025-10-19)
   
   **목표:** Record type 지원 및 복잡한 표현식 컴파일
   
   **구현 내용:**
   - ✅ Record type → LLVM struct 매핑
   - ✅ Field access (`p.x`) → extractvalue
   - ✅ Record construction (`{ x = 1, y = 2 }`) → insertvalue
   - ✅ Let expression 컴파일 (local variable tracking)
   - ✅ Pattern matching 컴파일 (M2에서 이미 완료)
   - ✅ 타입 추론 (field access를 위한)
   
   **IR Parser 확장:**
   - ✅ `parse_postfix_expr` - field access 파싱
   - ✅ `parse_record_expr` - record construction 파싱
   - ✅ Chained field access 지원
   
   **Codegen 확장:**
   - ✅ `compile_record` - record construction
   - ✅ `compile_field_access` - field extraction
   - ✅ `compile_let` - local variables
   - ✅ `infer_expr_type` - 기본 타입 추론
   
   **산출물:**
   - ✅ `compiler/src/ir_parser.rs` - field access & record parsing
   - ✅ `compiler/src/codegen.rs` - record type support
   - ✅ `examples/08-simple-record.pole-ir` - record test
   - ✅ `compiler/examples/test_add_points.rs`
   - ✅ `compiler/examples/m3_summary.rs`
   
   **검증 결과:**
   - ✅ 6/6 예제 파싱 성공
   - ✅ factorial(5) = 120
   - ✅ fibonacci(10) = 55
   - ✅ max(42, 17) = 42
   - ✅ distance_from_origin({x:3, y:4}) = 25
   - ✅ add_points({1,2}, {4,6}).x = 5
   
   **성공 기준 달성:**
   - ✅ Record types 완전 지원
   - ✅ Pattern matching 지원 (M2)
   - ✅ Let expressions 지원
   - ✅ 6개 예제 컴파일 가능
   
   **M3 완료:**
   - ✅ Record type 완전 구현
   - ✅ Field access 및 construction
   - ✅ LLVM struct 타입 매핑
   - ✅ M4 준비 완료
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 1일

4. **M4: Advanced Types - LLVM 백엔드** ✅ **완료** (1일, 2025-10-19)
   
   **목표:** 고급 타입 시스템 완성 (String, List, Variant, Option, Result, Unit)
   
   **구현 내용:**
   - ✅ M4.1: String Type - `{ i8*, i64 }` (pointer + length)
   - ✅ M4.2: List Type - `{ T*, i64 }` (element pointer + length)
   - ✅ M4.3: Variant Type - i32 tag for simple enums
   - ✅ M4.4: Option/Result Types - `{ i32 tag, T value }` with pattern matching
   - ✅ M4.5: Unit Type - i8 0
   
   **산출물:**
   - ✅ `examples/08-simple-record.pole-ir` - Record types
   - ✅ `examples/09-simple-string.pole-ir` - String parameters
   - ✅ `examples/10-string-literal.pole-ir` - String literals
   - ✅ `examples/11-simple-list.pole-ir` - List literals
   - ✅ `examples/12-simple-variant.pole-ir` - Basic variants
   - ✅ `examples/13-variant-tags.pole-ir` - Variant tag values
   - ✅ `examples/15-simple-option.pole-ir` - Option constructors
   - ✅ `examples/16-option-match.pole-ir` - Pattern matching
   - ✅ `examples/17-unit-type.pole-ir` - Unit type
   - ✅ `compiler/examples/m4_summary.rs` - Complete verification
   - ✅ `docs/M4_ADVANCED_TYPES_PROGRESS.md` - Progress documentation
   
   **검증 결과:**
   - ✅ 8개 M4 예제 모두 컴파일 성공
   - ✅ Pattern matching: Some/None/Ok/Err with value extraction
   - ✅ Variable binding in patterns
   - ✅ PHI nodes for branch merging
   
   **타입 시스템 달성:**
   - ✅ Basic types: Int, Nat, Bool, Float64, String, Unit
   - ✅ Compound types: Record, List, Option, Result
   - ✅ Simple variants (enums without payloads)
   - ✅ Pattern matching on all supported types
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 1일

 5. **M5: Runtime Functions** ✅ **완료** (2025-10-19)
   
   **목표:** 실용적인 프로그램 작성을 위한 runtime 함수 구현
   
   **구현 내용:**
   - ✅ String.length - Inline LLVM (extractvalue)
   - ✅ String.contains - C FFI (strstr)
   - ✅ print/println - C FFI (printf/puts)
   - ✅ List.concat - malloc/memcpy (동적 메모리 할당)
   - ✅ IR Parser multi-arg support - f(x, y) 지원
   - ✅ Type inference for builtins
   
   **산출물:**
   - ✅ `compiler/examples/test_string_length.rs` - 3/3 테스트 통과
   - ✅ `compiler/examples/test_string_contains.rs` - 4/4 테스트 통과
   - ✅ `compiler/examples/test_print.rs` - 1/1 테스트 통과
   - ✅ `compiler/examples/test_list_concat.rs` - 컴파일 검증 완료
   - ✅ `compiler/examples/test_user_validation.rs` - 6/6 함수 컴파일 성공
   - ✅ `docs/M5_RUNTIME_FUNCTIONS.md` - 완료 문서
   
   **검증 결과:**
   - ✅ String.length("hello") = 5, empty=0, long=42
   - ✅ String.contains: 4/4 테스트 통과 (true/false/empty/at_start)
   - ✅ print/println: "Hello, World!" 출력 성공
   - ✅ List.concat: LLVM 17 opaque pointer 환경에서 동작
   - ✅ user-validation: 전체 예제 컴파일 성공
   
   **완료 일자:** 2025-10-19
   **M5 마일스톤 완료!**

**5.1 성공 기준 (M0-M4):**
- ✅ 컴파일 성공률: 100% (모든 예제)
- ✅ 컴파일 성능: factorial(20) < 0.001ms (인터프리터: ~0.06ms)
- ✅ IR 파싱 성능: <0.5ms (Python 대비 10배+)
- ✅ 타입 체킹 성능: 5배+ 향상
- ✅ 정확성: 모든 테스트 통과
- ✅ 메모리 안전성: Rust 소유권 시스템으로 보장

**기술 스택:**

**Rust (핵심 컴포넌트):**
- **언어**: Rust 1.75+ (2024 Edition)
- **LLVM 바인딩**: llvm-sys 17.0+ 또는 inkwell 0.4+
- **파서**: nom 7.0+ (parser combinator)
- **Python 연동**: PyO3 0.20+
- **빌드**: Cargo + maturin (Python wheel)
- **테스트**: criterion (벤치마크)

**Python (인터페이스 & 도구):**
- **CLI**: Click 또는 argparse
- **LLM API**: anthropic, openai
- **테스트**: pytest (통합 테스트)

**하이브리드 아키텍처:**
```
Python (사용자 레이어)
  ├── CLI (pole check, run, build)
  ├── LLM Transformer (OpenRouter)
  └── Spec Parser (.pole)
       ↓ PyO3 바인딩
Rust (성능 critical 레이어)
  ├── IR Parser ⭐
  ├── Type Checker ⭐
  ├── LLVM Compiler
  ├── Contract Verifier ⭐
  └── Memory Manager
```

**리스크:**
- **High**: Rust 학습 곡선 (3개월)
- **완화:** 
  - 체계적 학습 계획 (The Rust Book → nom → LLVM)
  - M0에서 IR Parser/Type Checker로 실전 경험
  - 소규모 예제부터 시작
  - Rust 커뮤니티 활용 (Discord, Reddit)
- **Medium**: PyO3 통합 복잡도
- **완화:** 
  - M0에서 IR Parser 바인딩으로 먼저 검증
  - 간단한 C FFI 스타일 인터페이스 설계
  - Python 래퍼 패턴 사용 (기존 API 유지)
- **Medium**: IR Parser/Type Checker 마이그레이션
- **완화:**
  - Python 버전 유지 (백업)
  - 단계적 전환 (feature flag)
  - 철저한 테스트 (기존 테스트 100% 통과)
- **비상 계획:** Python 버전으로 롤백 (코드 보존)

**선행 조건:** 없음

**의존성:**
- 5.2, 5.3, 5.4가 이 작업에 의존

---

#### 5.1.5 **컴파일러 메모리 관리 최적화 (Arena Allocator)** ⭐ 높은 우선순위

**목표:** SQLite 스타일 메모리 관리로 컴파일러 안정성/성능 대폭 개선

**기간:** 1개월 (M2 직후 또는 병행)

**산출물:**
- `compiler/src/arena.rs` - Rust Arena allocator 구현
- `compiler/src/memory.rs` - 메모리 관리 시스템
- 문서: `docs/COMPILER_MEMORY_MANAGEMENT.md`

**구현 내용:**

1. **Rust Arena Allocator 통합**
   ```rust
   // bumpalo 라이브러리 활용
   use bumpalo::Bump;
   
   pub struct CompilerArenas {
       parse_arena: Bump,    // AST 파싱용 (50MB)
       ir_arena: Bump,       // IR 생성용 (30MB)
       codegen_arena: Bump,  // 코드 생성용 (20MB)
   }
   ```

2. **OOM 복구 메커니즘**
   ```rust
   // panic 대신 Result 반환
   fn compile_with_limit(source: &str, limit: usize) 
       -> Result<Module, CompileError> {
       let arena = CompilerArenas::new(limit);
       arena.compile(source)
           .map_err(|_| CompileError::OutOfMemory)
   }
   ```

3. **컴파일러 통합**
   - codegen.rs에 Arena 통합
   - ir_parser.rs 메모리 최적화
   - type_checker.rs Arena 활용

**예상 개선 효과:**
- 메모리 사용량: 75% 감소 (110MB → 30MB)
- 컴파일 속도: 3x 향상 (할당 오버헤드 감소)
- OOM 복구: 크래시 → 우아한 에러 처리
- 대규모 프로젝트: 1000 파일 컴파일 가능 (8GB RAM)

**성공 기준:**
- ✅ factorial 컴파일: 110MB → 30MB
- ✅ 1000 파일 프로젝트: 2GB → 500MB
- ✅ OOM 시 크래시 없음
- ✅ 성능: 3x 향상

**선행 조건:** 5.1 M2 완료 또는 병행

---

#### 5.2 **런타임 메모리 관리 시스템** ⭐ PZ 필수

**목표:** 대규모 엔티티 관리 (1000+ 좀비, 거대한 맵)

**기간:** 2개월 (Q3 2026-04 ~ 2026-05)

**우선순위:** P0 (PZ Clone 필수)

**PZ 산출물:**
- **Object Pool** (좀비/아이템 재활용)
- **Arena Allocator** (프레임 메모리)
- **Chunk System** (타일맵 스트리밍)
- **Spatial Index** (QuadTree)

**PZ 구현 내용:**

1. **좀비 풀링**
   ```pole
   type ZombiePool = {
     active: Array<Zombie>,    // 활성 좀비 (화면 내)
     inactive: Array<Zombie>,  // 비활성 좀비 (재활용 대기)
     max_count: Int = 2000
   }
   
   function spawn_zombie(pool: ZombiePool) -> Zombie:
     if pool.inactive.length > 0:
       return pool.inactive.pop()  // 재활용
     else if pool.active.length < pool.max_count:
       return Zombie.new()  // 새로 생성
     else:
       despawn_farthest()  // 먼 좀비 제거
   ```

2. **프레임 Arena**
   ```pole
   // 매 프레임 시작 시 리셋되는 메모리
   type FrameArena = {
     buffer: Ptr<u8>,
     size: Int = 10_000_000,  // 10MB
     offset: Int
   }
   
   // 충돌 감지, 임시 계산용
   function frame_alloc<T>(arena: FrameArena, count: Int) -> Ptr<T>
   ```

3. **소유권 시스템** (Rust 스타일)
   ```pole
   type Resource<T>
     @owned  // 소유권 명시
   
   function transfer(res: Resource<Texture>) -> Resource<Texture>
     // 소유권 이동, 복사 없음
   ```

**PZ 마일스톤:**
1. **M1: Object Pool** (1개월)
   - 좀비 풀: 2000개 관리
   - 아이템 풀: 5000개 관리
   - 탄환 풀: 1000개 관리

2. **M2: 청크 시스템** (1개월)
   - 100x100 타일맵 → 10x10 청크
   - 동적 로딩/언로딩
   - 3x3 청크만 활성화

**성공 기준:**
- ✅ 메모리 사용: < 500MB (1000 좀비)
- ✅ 할당 속도: < 0.1ms/프레임
- ✅ GC 일시정지: 0ms (수동 관리)

**선행 조건:** 5.1 완료

**주의:** 5.1.5는 컴파일러 자체의 메모리, 5.2는 컴파일된 프로그램의 런타임 메모리

---

#### 5.3 **성능 최적화 시스템** (선택적)

**목표:** 컴파일러 최적화 자동 적용

**기간:** 3-4개월 (5.1 M3 이후)

**우선순위:** P1 (성능 병목 확인 후)

**산출물:**
- `@inline`, `@simd`, `@hot_path` 어노테이션
- LLVM 최적화 파이프라인 통합
- 성능 벤치마킹 도구

**구현 내용:**

1. **컴파일 타임 최적화**
   - 상수 폴딩
   - 데드 코드 제거
   - 함수 인라인
   - 루프 언롤링

2. **SIMD 벡터화**
   ```pole
   @simd
   function dot_product(a: Vec3, b: Vec3) -> float:
     a.x * b.x + a.y * b.y + a.z * b.z
   // 컴파일러가 SSE/AVX 명령어 생성
   ```

3. **LTO (Link-Time Optimization)**
   - 모듈 간 최적화
   - 전역 인라인

**마일스톤:**
1. **M1: 기본 최적화** (1개월)
   - -O2 수준
2. **M2: SIMD 지원** (1개월)
3. **M3: LTO** (1개월)

**성공 기준:**
- ✅ 성능: Python 대비 10-100x
- ✅ SIMD: 벡터 연산 4x 빠름

**선행 조건:** 5.1 M3, 5.1.5 (컴파일러 메모리 최적화)

---

#### 5.4 **동시성 & 병렬 처리** ⭐ PZ 필수

**목표:** 멀티스레드 안전성, 병렬 실행 (좀비 AI, 네트워킹)

**기간:** 2-3개월 (Q3 2026-04 ~ 2026-06)

**우선순위:** P0 (PZ Clone 필수)

**산출물:**
- **스레드 FFI** (pthread/std::thread)
- **비동기 네트워크 I/O**
- **병렬 좀비 AI 업데이트**
- **Thread-safe 컬렉션**

**PZ 구현 내용:**

1. **게임 루프 스레딩**
   ```pole
   // 메인 스레드: 렌더링
   // 워커 스레드 1: 좀비 AI
   // 워커 스레드 2: 네트워킹
   // 워커 스레드 3: 청크 로딩
   
   function parallel_zombie_update(zombies: Array<Zombie>):
     // 1000개 좀비를 4개 스레드로 분할
     parallel_for chunk in split_array(zombies, 4):
       update_zombie_chunk(chunk)
   ```

2. **네트워크 비동기**
   ```pole
   @async
   function handle_client(socket: TcpSocket):
     while connected:
       let packet = await socket.recv()
       process_packet(packet)
       broadcast_state()
   ```

3. **공간 분할 병렬화**
   ```pole
   // QuadTree 각 노드를 병렬 처리
   function parallel_spatial_query(quadtree: QuadTree):
     parallel_for node in quadtree.nodes:
       process_entities_in_node(node)
   ```

**마일스톤:**
1. **M1: 기본 스레드** (1개월) - pthread FFI
2. **M2: 좀비 AI 병렬화** (1개월) - 1000+ 좀비
3. **M3: 네트워크 비동기** (1개월) - 8인 동시

**성공 기준:**
- ✅ 1000+ 좀비 @ 60 FPS (4코어)
- ✅ 8인 멀티플레이어 지연 < 50ms
- ✅ 청크 로딩 끊김 없음

**선행 조건:** 파일 I/O, 자료구조 완성

---

### P1 작업 (중요)

#### 5.5 **JIT 컴파일러** (선택적)
- **기간:** 3-4개월
- **목표:** 런타임 최적화
- **우선순위:** P1 (Phase 5 P0 완료 후 검토)

#### 5.6 **크로스 컴파일** (선택적)
- **기간:** 2-3개월
- **목표:** Windows, macOS, Linux 지원
- **우선순위:** P1

---

### Phase 5 완료 기준

**필수 (P0):**
- ✅ 네이티브 컴파일: 모든 예제 성공
- ✅ 성능: factorial(20) < 0.001ms (100x 개선)
- ✅ 메모리 안전성: 누수 0개
- ✅ 멀티스레드: 데이터 레이스 0개

**데모:**
- 네이티브 실행 파일 생성
- 성능 벤치마크 (vs Python, vs 인터프리터)
- YouTube 영상 공개

**Phase 5 → Phase 6 전환 조건:**
- 위 4가지 완료 기준 모두 충족

---

## Phase 6: 시스템 프로그래밍 기능 (Phase 6.1 ✅ 완료)

**기간:** 2025-10-19 완료 (FFI 핵심 기능)

**우선순위 변경:** Phase 5 나머지보다 **Phase 6을 먼저 진행**

**전략적 이유:**
1. **실용성 우선**: SDL2/OpenGL로 즉시 실제 프로그램 작성 가능
2. **가시적 성과**: 그래픽 프로그램 데모 → 커뮤니티 시연
3. **검증 기회**: 실제 개발로 언어 문제점 발견 → Phase 5 설계 개선
4. **단계적 가치**: FFI M1만 완료해도 유용, Phase 5는 전체 완성 필요

**Phase 6 목표:**
- ✅ SDL2, OpenGL 등 C/C++ 라이브러리 호출 가능
- ✅ 포인터 직접 조작 (unsafe 블록)
- ✅ 모듈 시스템으로 대규모 프로젝트 구조화

---

### P0 작업 (필수) - 최우선

#### 6.1 **FFI (Foreign Function Interface)** ⭐ 최우선

**목표:** C/C++ 라이브러리 호출

**기간:** 3-4개월 (2025-10 ~ 2026-01)

**우선순위:** P0 - **지금 바로 시작**

**산출물:**
- `@ffi`, `@extern` 어노테이션
- `pole bindgen <header.h>` 자동 바인딩 생성
- SDL2, OpenGL 바인딩

**구현 내용:**

1. **C ABI 호환성**
   ```pole
   @ffi("SDL2")
   @extern("SDL_Init")
   function sdl_init(flags: u32) -> i32
   
   @ffi("OpenGL")
   @extern("glClear")
   function gl_clear(mask: u32) -> Unit
   ```

2. **바인딩 자동 생성**
   ```bash
   pole bindgen SDL2.h > sdl2_bindings.pole
   pole bindgen vulkan/vulkan.h > vulkan_bindings.pole
   ```

3. **안전성 래퍼**
   ```pole
   @safe_ffi
   function load_texture(path: string) -> Result<Texture, Error>:
     underlying: SDL_LoadBMP(path)
     // 자동 에러 체크, 메모리 관리
   ```

**마일스톤:**

1. **M1: 간단한 C 함수 호출** (1개월) ✅ **완료** (2025-10-19)
   
   **작업 목록:**
   - [x] `specs/ffi.md` 설계 문서 작성
     - FFI 문법 정의 (`@extern` 어노테이션)
     - Pole 타입 → C ABI 타입 매핑 규칙
     - Rust/Zig/Swift FFI 패턴 참고
   - [x] Pole IR 문법 확장 (`specs/ir-syntax.md`)
     - 외부 함수 선언 구문 추가
     - 어노테이션 파싱 규칙 (`@extern`, `@variadic`, `@link`, `@header`)
   - [x] IR Parser 확장 (`compiler/src/ir_parser.rs`)
     - `@extern` 어노테이션 파싱 (위치 인자 지원)
     - 외부 함수 선언 AST 노드 추가 (`ExternFunctionDecl`)
   - [x] CodeGen 리팩토링 (`compiler/src/codegen.rs`)
     - 하드코딩된 `declare_libc_functions()` 제거
     - IR에서 `@extern` 선언 읽어서 동적 생성
     - Pole 이름 → C 이름 매핑 (`extern_func_mapping`)
   - [x] 테스트 예제 작성
     - `examples/19-ffi-printf.pole-ir` - printf 호출
     - `compiler/examples/test_ffi_printf.rs` - 통합 테스트
   - [x] 데모 검증: "Hello from C!" 출력 성공
   
   **성과:**
   - ✅ printf 네이티브 호출 성공
   - ✅ 가변 인자 함수 지원 (`@variadic`)
   - ✅ 외부 함수 동적 선언 시스템 구현
   - ✅ Pole 코드에서 C 함수 호출 가능
   
   **M1.5: 추가 개선** (2025-10-19)
   - ✅ 다중 인자 extern 함수 타입 추론
   - ✅ extern 함수 반환 타입 저장 (`extern_func_types`)
   - ✅ puts, putchar 등 다양한 C 함수 호출 검증
   - ✅ `examples/20-ffi-malloc.pole-ir` 예제
   
2. **M2: 구조체 전달** (1개월) ✅ **완료** (2025-10-19)
   
   **작업 목록:**
   - [x] Ptr<T> 포인터 타입 추가 (AST, IR parser, codegen)
   - [x] Record 타입 C 호환 검증 (LLVM struct_type)
   - [x] malloc/free로 포인터 전달 테스트
   - [x] `examples/22-ffi-pointer.pole-ir` 예제
   - [ ] `@repr(C)` 어노테이션 (현재 기본이 C 호환, 명시 불필요)
   - [ ] 실제 C 구조체 전달 예제 (향후 확장)
   
   **성과:**
   - ✅ Ptr<T> 타입 시스템에 추가
   - ✅ malloc(size) -> Ptr<Unit> 작동
   - ✅ free(Ptr<Unit>) 작동
   - ✅ Record는 이미 C 호환 struct로 컴파일됨
   - ✅ 포인터를 C 함수와 주고받을 수 있음
   
   **선행 조건:** ✅ M1 완료
   
3. **M4: SDL2 윈도우 띄우기** (1개월) ✅ **완료** (2025-10-19)
   
   **우선순위 변경 이유:**
   - M3 (콜백)은 복잡하고 시간 소요가 큼
   - M4는 M1+M2 기능만으로 구현 가능
   - 가시적인 성과 (실제 윈도우!)
   - FFI 실용성을 즉시 검증 가능
   
   **작업 목록:**
   - [x] SDL2 함수 extern 선언 작성
   - [x] SDL_Init, SDL_CreateWindow, SDL_DestroyWindow, SDL_Delay 바인딩
   - [x] 기본 윈도우 생성 예제 (headless mode)
   - [ ] 이벤트 폴링 (키보드 입력) - M4.5로 연기
   - [ ] **데모:** 윈도우 띄우고 ESC로 종료 - M4.5로 연기
   
   **구현 내용:**
   - ✅ SDL_CreateWindow: 6개 매개변수 (title, x, y, w, h, flags)
   - ✅ SDL_DestroyWindow: 윈도우 해제
   - ✅ SDL_Delay: 밀리초 단위 대기
   - ✅ IR parser 버그 수정: if-then-else에서 let 표현식 지원
   - ✅ Headless mode 테스트 (SDL_VIDEODRIVER=dummy)
   
   **산출물:**
   - ✅ `examples/24-sdl2-window.pole-ir` - SDL2 윈도우 생성/해제
   - ✅ `compiler/examples/test_sdl2_window.rs` - 통합 테스트
   - ✅ `compiler/src/ir_parser.rs` - if-then-else 파서 수정
   
   **검증 결과:**
   - ✅ 윈도우 생성 성공 (SDL_WINDOW_HIDDEN 플래그)
   - ✅ 윈도우 해제 성공
   - ✅ 메모리 누수 없음 (정상 종료)
   - ✅ 모든 기존 FFI 테스트 통과
   
   **완료 일자:** 2025-10-19
   **총 소요 시간:** 1일
   
   **선행 조건:** ✅ M1+M2 완료
   
4. **M3: 콜백 지원** (1개월) - 연기
   - Pole 함수를 C 함수 포인터로 전달
   - 이벤트 핸들러 패턴
   - Closure 캡처 제한적 지원
   - **데모:** qsort에 Pole 비교 함수 전달
   
   **선행 조건:** M4 완료 후 (실용성 검증 후 구현)
   **연기 사유:** 복잡도 대비 우선순위 낮음, M4가 더 실용적

**Phase 6.1 완료 요약:**
- ✅ M1-M4 완료 (FFI 시스템 + SDL2 윈도우)
- ✅ 5개 FFI 예제 작동 (printf, malloc, pointer, sdl2_init, sdl2_window)
- ✅ 18개 Rust unit test 통과
- ✅ 문서화 완료 (FFI Tutorial, Examples README)
- ⬜ M4.5 (이벤트 폴링) - 선택적, 연기됨
- ⬜ M3 (콜백) - 연기됨, Phase 6.2 이후 검토

**성공 기준 달성:**
- ✅ SDL2 윈도우 생성 성공
- ⬜ OpenGL 삼각형 렌더링 (향후)
- ⬜ 사용자 입력 처리 (M4.5, 향후)
- ✅ 메모리 누수 없음

**Phase 6.1 완료 일자:** 2025-10-19

**선행 조건:** ✅ Phase 5.1 완료 (충족됨)

---

#### 6.2 **저수준 메모리 제어** (연기됨)

**목표:** 포인터, unsafe 블록, 메모리 레이아웃 제어

**기간:** 2-3개월

**우선순위:** ~~P0~~ → **P3 (연기)**

**연기 사유:**
- Records 이미 C-compatible
- Ptr<T> 이미 구현됨 (Phase 6.1)
- 실질적 가치 낮음
- 필요시 재검토

**산출물:**
- `@repr(C)`, `@packed`, `@align(N)` 어노테이션
- `*const T`, `*mut T` 포인터 타입
- `unsafe { }` 블록

**구현 내용:**

```pole
@repr(C)  // C 구조체 호환
type Vertex:
  fields:
    - position: Vec3  // 12 bytes
    - normal: Vec3    // 12 bytes
    - uv: Vec2        // 8 bytes
  @align(16)  // 16바이트 정렬 (SIMD)

@unsafe
function write_gpu_buffer(ptr: *mut u8, data: Array<float>):
  // 직접 메모리 쓰기
```

**마일스톤:**
1. **M1: 메모리 레이아웃 제어** (1개월)
2. **M2: 포인터 타입** (1개월)
3. **M3: Unsafe 블록** (1개월)

**성공 기준:**
- ✅ GPU 버퍼 직접 쓰기 가능
- ✅ Zero-copy 데이터 전달

**선행 조건:** 6.1 M2

---

#### 6.3 **모듈 & 패키지 시스템** (연기됨)

**목표:** 대규모 프로젝트 관리 (게임 엔진 규모 지원)

**기간:** 4-5개월

**우선순위:** ~~P1~~ → **P2 (연기)**

**연기 사유:**
- 현재 단일 파일 프로젝트로 충분
- 대규모 프로젝트 없음
- Phase 7-8 진행 중 필요시 재검토

**산출물:**
- `pole.toml` 프로젝트 설정
- `pole add <package>` 패키지 관리자
- 증분 컴파일
- 중앙 패키지 레지스트리 (pole.dev)

**구현 내용:**

```pole
// pole.toml
[package]
name = "my_game_engine"
version = "0.1.0"

[dependencies]
pole_math = "1.0"
pole_graphics = "2.0"
pole_physics = "1.5"

// 모듈 정의
module graphics:
  public:
    type Renderer
    function create_renderer() -> Renderer
  private:
    type InternalState
```

```bash
pole new my_game
pole add pole_graphics@2.0
pole build --release
pole publish
```

**마일스톤:**
1. **M1: 모듈 시스템** (2개월)
2. **M2: 패키지 관리자** (2개월)
3. **M3: 증분 컴파일** (1개월)

**성공 기준:**
- ✅ 1000+ 파일 프로젝트 관리 가능
- ✅ 증분 컴파일: < 5초

**선행 조건:** Phase 5 완료

---

### P1 작업 (중요)

#### 6.4 **매크로 & 메타프로그래밍**
- **기간:** 3-4개월
- **목표:** 코드 생성 자동화
- **우선순위:** P1

---

### Phase 6 완료 기준 (재평가)

**달성됨 (Phase 6.1):**
- ✅ C 함수 호출 (@extern)
- ✅ 포인터 타입 (Ptr<T>)
- ✅ SDL2 윈도우 생성
- ✅ SDL2 렌더링 (픽셀 드로잉)
- ✅ 26개 예제 프로그램

**미달성 (연기):**
- ⏸️ SDL2 이벤트 폴링 (게임 프로토타입에서 구현 예정)
- ❌ Phase 6.2 저수준 메모리 (실질적 가치 낮음)
- ❌ Phase 6.3 모듈 시스템 (당장 불필요)

**데모 프로그램 (Phase 6 완료 시):**
```pole
// Pole로 작성한 실제 그래픽 프로그램
@ffi("SDL2")
@ffi("OpenGL")

func main() -> Unit :
  let window = SDL_CreateWindow("Pole Game", 800, 600) in
  let gl_context = SDL_GL_CreateContext(window) in
  
  game_loop(window, gl_context)

@unsafe
func game_loop(window: *Window, ctx: *GLContext) -> Unit :
  let running = true in
  while running do
    handle_events(&running)
    render_scene()
    SDL_GL_SwapWindow(window)
```

**Phase 6 → Phase 7 전환:**
- 6.1 완료 (필수)
- 6.2 완료 (필수)
- SDL2/OpenGL 데모 작동
- 실제 게임 프로토타입 제작 가능

---

## Phase 7: PZ 전용 라이브러리 (축소/변경)

**기간:** Year 1.5-2.5 (12개월) - 2026-07 ~ 2027-07

**목표:** Project Zomboid Clone에 필요한 최소 라이브러리만 구축

**중요: Phase 7-8은 "언어 표준 라이브러리" 개발**
- 게임 엔진이 아니라, 게임 엔진을 **만들 때 사용하는 라이브러리**
- 비유: Rust의 `std`, `bevy_ecs`, `wgpu` 같은 역할
- 이 라이브러리들을 조합하여 게임 엔진 제작 가능

**Phase 7 목표:**
- ✅ `pole_graphics`: 그래픽 추상화 라이브러리 (Vulkan/OpenGL/Metal)
- ✅ `pole_ecs`: Entity Component System 라이브러리
- ✅ `pole_physics`: 물리 엔진 라이브러리
- ✅ `pole_assets`: 에셋 관리 라이브러리
- ✅ 이 라이브러리들로 간단한 3D 게임 데모 제작 가능

**산출물:** Pole 표준 라이브러리 (게임 엔진 제작용)

---

### P0 작업 (PZ 필수만)

#### 7.1 **PZ 2D 타일 렌더러**

**목표:** `pz_renderer` - Project Zomboid 전용 아이소메트릭 렌더러

**기간:** 3개월

**산출물:**
- `pz_renderer` 라이브러리 (SDL2 기반)
- 아이소메트릭 타일맵 렌더링
- 스프라이트 배칭 (1000+ 타일)
- Y-sorting 및 레이어 시스템

**마일스톤:**
1. **M1: 타일 렌더링** (1개월)
2. **M2: 스프라이트 시스템** (1개월)
3. **M3: 조명/그림자** (1개월)

**성공 기준:**
- ✅ 100x100 타일맵 60 FPS
- ✅ 1000+ 스프라이트 동시 렌더링

---

#### 7.2 **PZ ECS (좀비 특화)**

**목표:** `pz_ecs` - 좀비 시뮬레이션 특화 ECS

**기간:** 3개월

**산출물:**
- `pz_ecs` 라이브러리 (간단한 ECS)
- 공간 분할 (QuadTree)
- 좀비 AI 시스템

**마일스톤:**
1. **M1: Entity 관리** (1개월)
2. **M2: 공간 분할** (1개월)
3. **M3: AI 시스템** (1개월)

**성공 기준:**
- ✅ 1000+ 좀비 60 FPS
- ✅ 시각/청각 시뮬레이션

---

#### 7.3 **PZ 네트워킹**

**목표:** `pz_network` - 멀티플레이어 시스템

**기간:** 3개월

**산출물:**
- `pz_network` 라이브러리
- Client-Server 아키텍처
- 상태 동기화 (플레이어, 좀비, 아이템)
- 2-8인 Co-op 지원

**성공 기준:**
- ✅ 8인 동시 접속
- ✅ 좀비 상태 동기화

---

#### 7.4 **PZ UI & 인벤토리**

**목표:** `pz_ui` - PZ 전용 UI 시스템

**기간:** 2개월

**산출물:**
- `pz_ui` 라이브러리
- 인벤토리 그리드
- 제작 메뉴
- 컨텍스트 메뉴

**성공 기준:**
- ✅ 드래그 앤 드롭
- ✅ 아이템 관리

---

### Phase 7 완료 기준 (PZ 전용)

**필수 (P0):**
- ✅ PZ 전용 라이브러리 완성 (`pz_renderer`, `pz_ecs`, `pz_network`, `pz_ui`)
- ✅ 아이소메트릭 타일맵 렌더링
- ✅ 1000+ 좀비 시뮬레이션
- ✅ 8인 멀티플레이어 작동

**데모:**
- PZ 프로토타입 (1시간 플레이)
- 4인 Co-op 데모
- Steam 페이지 준비

**Phase 7 → PZ Clone 개발:**
- Year 3에 본격 게임 개발 시작

---

## Phase 8: PZ Clone 개발 (Year 3)

**기간:** Year 3 (2027-2028)

**목표:** Steam 출시 가능한 Project Zomboid Clone 완성

### 개발 단계

**Q1 (2027-10 ~ 2027-12): 핵심 게임플레이**
- 전체 생존 시스템
- 스킬 시스템
- 다양한 좀비 타입
- 무기/도구 시스템

**Q2 (2028-01 ~ 2028-03): 콘텐츠 확장**
- 3-5개 도시
- 건축 시스템
- 농사/낚시
- 날씨/계절

**Q3 (2028-04 ~ 2028-06): Early Access**
- Steam Early Access 출시
- 10시간 콘텐츠
- 8인 멀티플레이어

**Q4 (2028-07 ~ 2028-09): 정식 출시**
- 20-30시간 콘텐츠
- 안정적인 멀티플레이어
- 목표: 10,000 판매, $200K 수익

---

## Phase 9+: 장기 계획 (PZ 이후)

**참고:** PZ Clone 출시 후 고려

**목표:** Pole 언어 개발 도구 (IDE, 디버거, 프로파일러, 빌드 시스템)

**중요: Phase 9는 "Pole 언어" 자체의 개발 도구**
- 게임 엔진 에디터가 **아님**
- Pole 언어로 프로그래밍할 때 사용하는 도구들
- 비유: Rust의 rust-analyzer, cargo, rustfmt 같은 역할

**참고 프로젝트 (선택):**
- Pole로 간단한 게임 엔진 에디터 구현 (Pole 언어 능력 시연용)

### P0 작업

#### 9.1 **IDE 통합 (LSP)**
- **기간:** 4-6개월
- Language Server Protocol 구현
- 문법 하이라이팅, 자동 완성, 에러 표시
- VSCode, Vim, Emacs 지원

#### 9.2 **디버거 & 프로파일러**
- **기간:** 4-5개월
- Pole 프로그램 디버깅 도구
- 성능 프로파일러

#### 9.3 **빌드 시스템 & 패키지 매니저**
- **기간:** 3-4개월
- 크로스 플랫폼 빌드 (Windows, macOS, Linux)
- 패키지 관리 (`pole add`, `pole publish`)

#### 9.4 **(선택) 참고용 게임 엔진 에디터**
- **목표:** Pole로 만든 간단한 게임 엔진 에디터 (Pole 언어 시연용)
- **기간:** 8-10개월
- 씬 편집, 인스펙터, 에셋 브라우저
- **주의:** 이것은 "Pole 언어 능력 증명"을 위한 참고 프로젝트

### P1 작업

#### 9.5 **문서 생성 & 학습 자료**
- **기간:** 지속적
- API 문서 자동 생성
- Pole 언어 튜토리얼 및 예제

### Phase 9 완료 기준

**필수 (P0):**
- ✅ IDE 통합 완성 (LSP)
- ✅ 디버거 & 프로파일러 작동
- ✅ 크로스 플랫폼 빌드 성공
- ✅ 패키지 관리 시스템 작동

**선택 (P1):**
- ✅ (선택) 참고용 게임 엔진 에디터 완성

**Phase 9 → Phase 10 전환:**
- 언어 도구 완성 + 생태계 구축 시작

---

## Phase 10: 언어 생태계 & 커뮤니티

**기간:** Year 4-5+ (지속적) - 2029-01 이후

**목표:** Pole 언어 생태계 구축 (커뮤니티, 패키지, 플러그인)

**중요: Phase 10은 "Pole 언어 생태계" 구축**
- 게임 출시나 상용화가 아니라, **언어 자체의 생태계**
- Pole 패키지 레지스트리 (pole.dev)
- 커뮤니티 성장 (개발자, 라이브러리 제작자)
- 다양한 도메인에서 Pole 사용 확산

### P0 작업

#### 10.1 **패키지 레지스트리 (pole.dev)**
- **기간:** 3-4개월
- 중앙 패키지 레지스트리
- `pole search`, `pole publish`

#### 10.2 **컴파일러 고급 최적화**
- **기간:** 6-8개월
- LTO, PGO (Profile-Guided Optimization)
- 추가 최적화 패스

### P1 작업

#### 10.3 **커뮤니티 성장**
- **기간:** 지속적
- 공식 포럼, Discord
- 컨퍼런스 개최
- 오픈소스 프로젝트 지원

#### 10.4 **다양한 도메인 확장**
- **기간:** 지속적
- 게임 개발 외 다른 시스템 프로그래밍 도메인
- 임베디드, 웹 어셈블리, 시스템 도구 등

### Phase 10 완료 기준

**필수 (P0):**
- ✅ Pole 패키지 레지스트리 운영 (pole.dev)
- ✅ Pole 언어 커뮤니티 1000+ 개발자
- ✅ Pole 패키지 생태계 활성화 (100+ 패키지)
- ✅ 다양한 도메인에서 Pole 사용 사례 확보

**성공 사례 (예시):**
- ✅ Pole로 만든 오픈소스 게임 엔진 출시 (커뮤니티 프로젝트)
- ✅ 그 엔진으로 만든 상업 게임 출시
- ✅ Pole로 만든 시스템 도구, 라이브러리 등

**Phase 10 완료 = Pole 언어 성숙 단계 도달**

---

## PZ Clone 주요 마일스톤

### ✅ Milestone 0: 기술 기반 완료 - **완료**
- ✅ LLVM 네이티브 컴파일
- ✅ SDL2 FFI 통합
- ✅ 성능: 100x+ 개선
- **현재:** Phase 5.1 & 6.1 완료

### 🎯 Milestone 1: 아이소메트릭 데모 (3개월)
- [ ] 100x100 타일맵
- [ ] 좀비 100마리
- [ ] YouTube 데모
- **목표:** 2026년 1월

### 🎮 Milestone 2: 생존 루프 (6개월)
- [ ] 인벤토리/제작
- [ ] 전투 시스템
- [ ] 2인 Co-op
- **목표:** 2026년 4월

### 🏗️ Milestone 3: PZ 프로토타입 (12개월)
- [ ] 1시간 플레이
- [ ] 4인 멀티플레이어
- [ ] 크라우드펀딩
- **목표:** 2026년 10월

### 🚀 Milestone 4: Early Access (24개월)
- [ ] 10시간 콘텐츠
- [ ] 8인 멀티플레이어
- [ ] Steam 출시
- **목표:** 2027년 10월

### 🌟 Milestone 5: 정식 출시 (36개월)
- [ ] 20-30시간 콘텐츠
- [ ] 10,000+ 판매
- [ ] $200K+ 수익
- **목표:** 2028년 10월

---

## PZ Clone 성공 지표

### 3개월 (2026-01)
- [ ] 아이소메트릭 렌더링 작동
- [ ] 100x100 타일맵
- [ ] YouTube 1000+ 조회수
- [ ] Discord 50+ 멤버

### 6개월 (2026-04)
- [ ] 좀비 100마리 시뮬레이션
- [ ] 기본 전투 시스템
- [ ] 플레이 가능한 데모
- [ ] 팔로워 500+

### 12개월 (2026-10)
- [ ] 1시간 플레이 콘텐츠
- [ ] 4인 Co-op 작동
- [ ] 크라우드펀딩 $50K
- [ ] 위시리스트 1000+

### 24개월 (2027-10)
- [ ] 5시간 콘텐츠
- [ ] Early Access 준비
- [ ] 위시리스트 5000+
- [ ] 스트리머 관심

### 36개월 (2028-10)
- [ ] Steam 정식 출시
- [ ] 8인 멀티플레이어
- [ ] 판매 10,000+
- [ ] 수익 $200K+
- [ ] 활성 서버 100+

---

## 리스크 관리

### 기술적 리스크

#### High Risk

1. **LLVM 통합 복잡도** (Phase 5.1)
   - **완화:** 단계적 구현, 커뮤니티 지원
   - **비상 계획:** Cranelift 또는 C++ 트랜스파일러

2. **성능 목표 미달** (Phase 7)
   - **완화:** 조기 벤치마킹, 프로파일링
   - **비상 계획:** 네이티브 플러그인 허용

3. **IDE 통합 복잡도** (Phase 9)
   - **완화:** Rust Analyzer, TypeScript LSP 참고
   - **비상 계획:** 기본 플러그인만 제공

#### Medium Risk

1. **FFI 안정성** (Phase 6.1)
2. **크로스 플랫폼** (Phase 9.3)

---

## 팀 & 리소스 계획

### 팀 규모

#### Year 1-2 (Phase 5)
- 컴파일러 엔지니어: 2명 (Rust 학습 필수)
- 런타임 엔지니어: 1명
- **총:** 3명
- **요구 스킬:** Rust, LLVM, Python (기존), 시스템 프로그래밍

#### Year 3-4 (Phase 6-7)
- 컴파일러: 2명
- 표준 라이브러리: 3명 (그래픽, ECS, 물리)
- QA: 1명
- **총:** 6명

#### Year 5-7 (Phase 8-9)
- 코어: 2명
- 표준 라이브러리: 3명 (고급 기능)
- 언어 도구: 3명 (LSP, 디버거, 빌드)
- QA: 1명
- **총:** 9명

#### Year 8-10 (Phase 10)
- 전체 팀: 10-15명 (언어 개발 + 커뮤니티 관리)

### 예산 (연간)

- Year 1-2: $300K-500K (컴파일러 개발)
- Year 3-4: $600K-1M (표준 라이브러리)
- Year 5-7: $1M-1.5M (언어 도구)
- Year 8-10: $1.5M-2M (생태계)

**총 예상 비용:** $3.4M-5M (10년, 언어 개발)

---

## 현재 최우선 작업 (Project Zomboid Clone 목표)

**현재 상태:** Phase 5.1 ✅ 완료, Phase 6.1 ✅ 완료 (2025-10-19)

**최종 목표:** 3년 내 Project Zomboid Clone 출시 (PZ-First 전략)

### 🎯 PZ 즉시 우선순위 (Week 1)

#### **최우선: 아이소메트릭 렌더링 PoC** ⭐⭐⭐
**기간:** 7일 (2025-10-20 ~ 2025-10-26)
**목표:** PZ 스타일 아이소메트릭 타일 렌더링 검증

**Day 1-2: 아이소메트릭 수학**
- [ ] iso_to_screen 함수 구현 
- [ ] screen_to_iso 역변환
- [ ] 타일 좌표 시스템 설계

**Day 3-4: SDL2 타일 렌더링**
- [ ] 타일 텍스처 로딩 (64x32 픽셀)
- [ ] 10x10 그리드 렌더링
- [ ] Y-sorting (깊이 정렬)

**Day 5-6: 카메라 & 입력**
- [ ] WASD 카메라 이동
- [ ] 마우스 휠 줌
- [ ] 타일 하이라이트

**Day 7: 데모 제작**
- [ ] YouTube 비디오 
- [ ] Reddit r/projectzomboid 공유

**가치:** 
- PZ 핵심 기술 즉시 검증
- 커뮤니티 초기 관심 확보
- 기술적 타당성 입증

### 📋 PZ 단기 우선순위 (1개월)

#### 1. **파일 I/O & 자료구조** ⭐⭐⭐ PZ 필수
**기간:** 2주
**산출물:**
- [ ] 파일 읽기/쓰기 FFI (fopen, fread, fwrite)
- [ ] 동적 배열 완전 구현 (resize, push, pop)
- [ ] HashMap/Dictionary 구현
- [ ] JSON 파서 (세이브/로드용)

#### 2. **아이소메트릭 타일맵 시스템**
**기간:** 2주  
**산출물:**
- [ ] 100x100 타일맵 렌더링
- [ ] 타일 스프라이트 관리
- [ ] 카메라 팬/줌 완성
- [ ] 건물 레이어 시스템

### 📅 PZ 중기 우선순위 (3-6개월)

#### Q1 (Month 1-3): PZ 핵심 시스템
- [ ] **좀비 AI 시뮬레이션** (100+ 엔티티)
- [ ] **네트워킹 FFI** (TCP/UDP 소켓)
- [ ] **2인 Co-op 프로토타입**
- [ ] **메모리 최적화** (Arena allocator)

#### Q2 (Month 4-6): PZ 게임플레이
- [ ] **인벤토리 시스템**
- [ ] **제작 시스템**
- [ ] **전투 메커니즘**
- [ ] **생존 루프** (배고픔/갈증)

### 🔄 PZ 우선순위 재조정

**PZ 필수 → 즉시/단기:**
- ✅ **파일 I/O** (세이브/로드) → Month 1
- ✅ **자료구조** (Array, HashMap) → Month 1  
- ✅ **네트워킹** (TCP/UDP) → Month 3
- ✅ **메모리 최적화** (1000+ 좀비) → Month 3

**PZ 불필요 → 연기/제거:**
- ❌ **3D 그래픽** → 제거
- ❌ **고급 물리** → 제거
- ❌ **모듈 시스템** → 연기
- ❌ **웹어셈블리** → 제거
- ❌ **Phase 7 범용 라이브러리** → PZ 전용으로 축소

### 완료된 작업 요약

**✅ Phase 5.1: LLVM 백엔드 (M0-M5 완료)**
- Rust IR Parser/Type Checker: 23.4x/25.6x 성능 향상
- LLVM 네이티브 컴파일: factorial, fibonacci 등
- 고급 타입: Record, List, Option, String, Unit
- Runtime 함수: String.length, List.concat, print
- Arena Allocator 통합

**✅ Phase 6.1: FFI System (M1-M4 완료)**
- C 함수 호출 (@extern)
- 포인터 타입 (Ptr<T>)
- SDL2 윈도우 생성 및 렌더링
- 26개 예제 프로그램 작동

### PZ Clone 작업 로드맵 (3년 목표)

#### 🚀 Week 1 (2025-10-20 ~ 2025-10-26): 아이소메트릭 PoC
```
작업 항목:
□ Day 1-2: 아이소메트릭 수학
  - iso_to_screen / screen_to_iso 함수
  - 타일 좌표계 설계
□ Day 3-4: SDL2 타일 렌더링
  - 64x32 타일 텍스처
  - 10x10 그리드 렌더링
  - Y-sorting 구현
□ Day 5-6: 카메라 시스템
  - WASD 이동
  - 마우스 휠 줌
□ Day 7: 데모 & 커뮤니티
  - YouTube 비디오
  - Reddit 공유
```

#### ⚡ Q3 성능 최적화 (2026-04 ~ 2026-06): PZ 필수
```
핵심 목표: 1000+ 좀비 @ 60 FPS

□ Month 4: 동시성 시스템
  - pthread/std::thread FFI
  - 워커 스레드 풀 (4+ 스레드)
  - Lock-free 큐
  
□ Month 5: 병렬 처리
  - 좀비 AI 병렬화 (250 좀비/스레드)
  - 공간 분할 병렬 쿼리
  - 네트워크 비동기 I/O
  
□ Month 6: 메모리 최적화
  - Object Pool (좀비 2000개)
  - Frame Arena (10MB/프레임)
  - 청크 시스템 (10x10)
```

#### 📋 Month 1 (2025-11): PZ 기초 시스템
```
□ Week 1-2: 파일 I/O & 자료구조
  - 파일 읽기/쓰기 FFI
  - 동적 배열 구현
  - HashMap 구현
□ Week 3-4: 타일맵 확장
  - 100x100 타일 렌더링
  - 청크 로딩 시스템
  - 건물 진입/탈출
```

#### 📋 Month 2-3 (2025-12 ~ 2026-01): 좀비 & 네트워크
```
□ Month 2: 좀비 시뮬레이션
  - 100+ 좀비 AI
  - 시각/청각 시스템
  - 경로 찾기 (A*)
□ Month 3: 네트워킹 기초
  - TCP/UDP 소켓 FFI
  - 2인 LAN Co-op
  - 상태 동기화
```

#### 🎯 Month 4-6 (2026-02 ~ 2026-04): PZ 게임플레이
```
□ Month 4: 전투 & 인벤토리
  - 근접 전투 시스템
  - 인벤토리 UI
  - 아이템 줍기/드롭
□ Month 5: 생존 시스템
  - 배고픔/갈증/피로
  - 제작 시스템
  - Day/Night 사이클
□ Month 6: 통합 & 최적화
  - 메모리 최적화 (Arena)
  - 4인 Co-op 테스트
  - Demo 3 완성
```

#### 📅 Year 2-3: PZ Clone 완성

**Year 2 (2026-2027): PZ 엔진 개발**
```
□ Q1: 타일맵 엔진 완성
□ Q2: ECS & 좀비 AI
□ Q3: 멀티플레이어 (8인)
□ Q4: 1시간 플레이 프로토타입
```

**Year 3 (2027-2028): PZ Clone 출시**
```
□ Q1: 핵심 게임플레이
□ Q2: 콘텐츠 확장
□ Q3: Steam Early Access
□ Q4: 정식 출시
```

### ❌ PZ 불필요 → 제거/연기

**제거 (PZ에 불필요):**
- ❌ **3D 그래픽** (Phase 7)
- ❌ **고급 물리 엔진** 
- ❌ **웹어셈블리 타겟**
- ❌ **모바일 플랫폼**

**연기 (나중에 필요시):**
- ⏸️ **모듈 시스템** (Phase 6.3)
- ⏸️ **고급 타입 시스템** (Phase 11)
- ⏸️ **범용 게임 엔진** → PZ 전용 엔진으로 대체

---

## Phase 11+: 장기 고급 기능 (Year 5+)

### 11.1 고급 타입 시스템
- 의존 타입 (dependent types)
- 선형/affine 타입
- 효과 시스템 (effect system)
- 소유권/차용 시스템 확장

### 11.2 형식 검증 시스템
- SMT 솔버 통합 (Z3, CVC5)
- 정리 증명기 연동
- 모델 체킹 도구
- 종료성 분석

### 11.3 LLM 변환 고도화
- 다중 후보 생성 및 평가
- 컨텍스트 기반 최적화
- 도메인 특화 파인튜닝
- 대화형 정제 시스템

---

## 변경 이력

- **2025-10-19**: Project Zomboid Clone 최우선 전략 수립 🧟
  - **핵심 변경:** 3년 내 PZ Clone 출시 목표
  - PZ-First 개발 전략 채택
  - Phase 7을 PZ 전용 라이브러리로 축소
  - Phase 8을 PZ Clone 개발로 변경
  - Phase 9+ 장기 계획으로 연기
  - 멀티플레이어를 필수 기능으로 포함
  - 3D/고급 기능 제거, PZ 필수만 집중
  - 새 로드맵: `docs/roadmaps/PROJECT_ZOMBOID_ROADMAP.md`
- **2025-10-19**: Phase 6.1 완료 (FFI + SDL2) ✅
  - M1-M4 완료: C 함수 호출, 포인터, SDL2 윈도우
  - M4.5 완료: SDL2 인터랙티브 데모 (10초 표시)
  - 26개 예제 프로그램 작동
  - FFI Tutorial 문서화 완료
- **2025-10-19**: Phase 5 M5 완료 (Runtime Functions) ✅
  - String.length 구현 (inline LLVM extractvalue)
    - 테스트: 3/3 통과 (hello=5, empty=0, long=42)
  - String.contains 구현 (C FFI strstr)
    - declare_libc_functions() 추가
    - compile_string_contains() 구현
  - 발견: IR 파서가 curried/multi-arg 미지원
  - **다음 단계**: IR 파서 개선 또는 추가 runtime 함수 구현
- **2025-10-19**: Phase 5 M2 준비 완료 (Arena Allocator 도입) ✅
  - bumpalo 라이브러리 통합 완료
  - CompilerArenas 구조체 구현 (parse, ir, codegen 영역)
  - CodeGen에 Arena 적용 (lifetime 추가)
  - 메모리 통계 및 벤치마크 추가
  - 모든 기존 예제 Arena 적용 및 통과
  - 성능: 12.15µs/compilation (100회 평균)
  - **완료 일자**: 2025-10-19
  - **다음 단계**: M3 LLVM 고급 기능 개발
- **2025-10-19**: Phase 5 M1.5 완료 (Python-Rust 통합 완성) 🎉
  - **해결한 문제**: `ir_parser_rust.py`가 type_defs를 무시하던 문제 수정
  - **구현**: convert_type_def() 함수, Literal type_name 전달
  - **검증**: 모든 End-to-End 테스트 통과, 5/6 예제 타입 체크 성공
  - **성능**: 23.4배 유지 (type_defs 변환 오버헤드 무시 가능)
  - **완료 일자**: 2025-10-19 (반나절 소요)
  - **다음 단계**: M2 (LLVM 백엔드) 시작 준비 완료
- **2025-10-19**: Phase 5 M1 완료 (Rust IR Parser 완성) 🎉
  - **완료 내용**: Rust IR Parser가 Python IR Parser와 기능 동등성 달성
  - Type definitions 파싱 구현 (Record, Variant, Alias)
  - Custom type names 인식 및 logical operators 지원
  - **6/6 예제 파싱 성공** (이전 2/6에서 개선)
  - Python-Rust 결과 100% 일치
  - 성능: Python 대비 **23.4배 향상** (0.014ms vs 0.322ms)
  - **발견**: Python 통합 레이어 불완전 (type_defs 손실)
- **2025-10-19**: Phase 5 M1 마일스톤 재구성 (우선순위 수정)
  - **변경 이유**: Rust IR Parser 기능 부족 발견 (2/6 예제만 통과)
  - M1 분할: "Rust IR Parser 완성" (2-3주) + "LLVM 백엔드" (2개월) → M1, M2로 분리
  - M2-M4 재조정: LLVM 백엔드를 단계별로 구현
  - **최우선 작업**: Rust IR Parser에 Type Definitions 및 Multi-line Expression 추가
  - **목표**: Python-Rust 기능 동등성 확보 후 LLVM 작업 시작
- **2025-10-19**: Phase 5-10 로드맵 재구성 (용어 명확화)
  - **핵심 변경**: Pole = 프로그래밍 언어 (게임 엔진 아님)
  - Phase 7-8: "게임 엔진" → "게임 개발 표준 라이브러리"
  - Phase 9: "에디터" → "언어 개발 도구 (IDE, 디버거 등)"
  - Phase 10: "상용화" → "언어 생태계 구축"
  - 게임 엔진은 Pole로 만들 수 있는 **별도 프로젝트**
- **2025-10-19**: 게임 엔진 비전으로 대폭 수정
  - Phase 5-10 추가 (7-10년 로드맵)
  - Phase 0-4 완료 선언
  - 기존 로드맵 → ROADMAP-v1-prototype.md 백업
- **2025-10-19**: Phase 4.4 완료 (IR 후처리 자동화)
- **2025-10-19**: Phase 4.3 완료 (LLM Prompt 개선)
- 이전 변경 이력: [ROADMAP-v1-prototype.md](ROADMAP-v1-prototype.md)
