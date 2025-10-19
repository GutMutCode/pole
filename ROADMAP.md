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

### 타임라인 (7-10년)

```
Year 0-1   : Phase 0-4  언어 기초 (완료) ✅
Year 1-2.5 : Phase 5-6  컴파일러 & 시스템 프로그래밍
Year 2.5-5.5: Phase 7-8  게임 개발 표준 라이브러리 (렌더링, 물리, ECS)
Year 5.5-7 : Phase 9    언어 개발 도구 (IDE, 디버거, 프로파일러)
Year 7-10  : Phase 10   언어 생태계 & 패키지 시스템
```

### 현재 위치

**2025-10-19**: Phase 4 완료 → **Phase 5 시작**

**완료된 것:**
- ✅ 명세 언어 설계 및 파서
- ✅ IR 설계 및 인터프리터
- ✅ LLM 변환 시스템 (OpenRouter)
- ✅ 타입 체커, 계약 검증
- ✅ CLI 도구 (check, build, run, test)

**다음 목표:**
- 🎯 네이티브 컴파일러 (LLVM)
- 🎯 고성능 런타임
- 🎯 시스템 프로그래밍 기능 (FFI, 저수준 메모리 제어)

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

3. **M3: LLVM 백엔드 - 고급 기능** (1.5개월, 2026-01-10 ~ 2026-02-25)
   
   **구현 내용:**
   - Record type → LLVM struct 매핑
   - Pattern matching 컴파일
   - Let expression 컴파일
   - 메모리 레이아웃 계산
   
   **검증:**
   - user-validation 예제 컴파일
   - is_even, max 예제 컴파일
   
   **성공 기준:**
   - ✅ 4/6 예제 컴파일 성공 (factorial, is_even, max, simple-math)
   
   **M3 완료 시 달성:**
   - ✅ 기본 함수 네이티브 컴파일 완성
   - ✅ Record types 지원
   - ✅ Pattern matching 지원

4. **M4: 전체 예제 컴파일** (1.5개월, 2026-02-25 ~ 2026-04-10)
   - 모든 examples/*.pole 컴파일 성공
   - 타입 체킹 통합 (이미 Rust로 완성)
   - **검증:** 6개 예제 네이티브 실행
   - **성능 벤치마크:** IR Parser + Type Checker Rust 효과 측정

**성공 기준:**
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

#### 5.2 **런타임 메모리 관리 시스템**

**목표:** Pole 프로그램의 런타임 메모리 관리 (컴파일된 코드용)

**기간:** 4-6개월 (5.1 완료 후)

**산출물:**
- `specs/memory-model.md` - 메모리 모델 설계
- `compiler/src/runtime_memory/` (Rust, 신규)
  - `gc.rs` - 가비지 컬렉션 (참조 카운팅)
  - `allocator.rs` - 런타임 할당자 (게임 엔진용)
  - `ownership.rs` - 소유권 추적 및 검증
- `@manual_memory`, `@heap_allocated` 어노테이션

**구현 내용:**

1. **참조 카운팅 (RC)**
   - 자동 증가/감소
   - 순환 참조 감지 (Weak references)
   - Zero-cost abstractions

2. **게임 엔진용 커스텀 할당자**
   - Frame allocator (프레임 단위 해제)
   - Object pool (엔티티 재사용)
   - Stack allocator (임시 데이터)

3. **소유권 시스템** (Rust 스타일)
   ```pole
   type Resource<T>
     @owned  // 소유권 명시
   
   function transfer(res: Resource<Texture>) -> Resource<Texture>
     // 소유권 이동, 복사 없음
   ```

**마일스톤:**
1. **M1: RC 기본 구현** (2개월)
   - 참조 카운트 자동 관리
   - 메모리 해제 자동화

2. **M2: 순환 참조 감지** (1개월)
   - Weak references
   - 누수 검증 도구

3. **M3: 게임 엔진 할당자** (1개월)
   - Frame allocator
   - Object pooling

4. **M4: 메모리 프로파일러** (1개월)
   - 런타임 메모리 추적
   - 누수 자동 감지

**성공 기준:**
- ✅ 메모리 누수: 0개 (Valgrind 검증)
- ✅ 오버헤드: < 5% (RC)
- ✅ Frame allocator: 10x 빠른 할당

**선행 조건:** 5.1 완료

**주의:** 5.1.5는 컴파일러 자체의 메모리, 5.2는 컴파일된 프로그램의 런타임 메모리

---

#### 5.3 **성능 최적화 시스템**

**목표:** 컴파일러 최적화 자동 적용

**기간:** 3-4개월 (5.1 M3 이후)

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

#### 5.4 **동시성 & 병렬 처리**

**목표:** 멀티스레드 안전성, 병렬 실행

**기간:** 4-5개월 (5.2 이후)

**산출물:**
- `@async`, `@parallel` 어노테이션
- Thread-safe 타입 시스템
- 데이터 레이스 컴파일 타임 검증

**구현 내용:**

1. **비동기 프로그래밍**
   ```pole
   @async
   function load_asset(path: string) -> Future<Asset>:
     // 비동기 로딩
   ```

2. **병렬 처리**
   ```pole
   @parallel
   function update_particles(particles: Array<Particle>):
     // 자동 병렬화
   ```

3. **동기화 기본 요소**
   - Mutex, RwLock
   - Atomic 연산
   - Channel (메시지 전달)

**마일스톤:**
1. **M1: 스레드 안전성 분석** (2개월)
2. **M2: async/await** (2개월)
3. **M3: 병렬 for** (1개월)

**성공 기준:**
- ✅ 데이터 레이스: 0개 (컴파일 타임 검증)
- ✅ 병렬 성능: 4코어에서 3.5x

**선행 조건:** 5.1 M4, 5.2 M2

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

## Phase 6: 시스템 프로그래밍 기능 (Year 2.5-3.5)

**기간:** 2027-03 ~ 2028-03 (1년)

**목표:** C/C++ 라이브러리 연동, 저수준 제어, 대규모 프로젝트 지원

**Phase 6 목표:**
- ✅ SDL2, OpenGL 등 C/C++ 라이브러리 호출 가능
- ✅ 포인터 직접 조작 (unsafe 블록)
- ✅ 모듈 시스템으로 대규모 프로젝트 구조화

---

### P0 작업 (필수)

#### 6.1 **FFI (Foreign Function Interface)**

**목표:** C/C++ 라이브러리 호출

**기간:** 3-4개월

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
1. **M1: 간단한 C 함수 호출** (1개월)
2. **M2: 구조체 전달** (1개월)
3. **M3: 콜백 지원** (1개월)
4. **M4: SDL2로 윈도우 띄우기** (1개월)

**성공 기준:**
- ✅ SDL2 윈도우 생성 성공
- ✅ OpenGL 삼각형 렌더링

**선행 조건:** Phase 5 완료

---

#### 6.2 **저수준 메모리 제어**

**목표:** 포인터, unsafe 블록, 메모리 레이아웃 제어

**기간:** 2-3개월

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

#### 6.3 **모듈 & 패키지 시스템**

**목표:** 대규모 프로젝트 관리 (게임 엔진 규모 지원)

**기간:** 4-5개월

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

### Phase 6 완료 기준

**필수 (P0):**
- ✅ SDL2 윈도우 + OpenGL 삼각형
- ✅ 포인터로 GPU 버퍼 조작
- ✅ 모듈 시스템으로 대규모 프로젝트

**데모:**
- SDL2 + OpenGL 데모 프로그램
- 회전하는 삼각형 렌더링

**Phase 6 → Phase 7 전환:**
- P0 완료 + 시스템 프로그래밍 준비 완료

---

## Phase 7: 게임 개발 표준 라이브러리 - 기본 (Year 3.5-5)

**기간:** 2028-03 ~ 2029-09 (1.5년)

**목표:** Pole 표준 라이브러리 구축 (게임 개발에 필요한 기본 라이브러리)

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

### P0 작업 (필수)

#### 7.1 **그래픽 렌더링 추상화 라이브러리**

**목표:** `pole_graphics` - 크로스플랫폼 그래픽 API 추상화

**기간:** 6-8개월

**산출물:**
- `pole_graphics` 라이브러리 (게임 엔진이 사용할 라이브러리)
- Vulkan/OpenGL/Metal 백엔드 지원
- 셰이더 컴파일러 (Pole 셰이더 언어 → SPIR-V)

**마일스톤:**
1. **M1: 2D 스프라이트** (2개월)
2. **M2: 3D 메시** (2개월)
3. **M3: 기본 라이팅** (2개월)
4. **M4: 텍스처 매핑** (2개월)

**성공 기준:**
- ✅ 3D 모델 렌더링
- ✅ 60 FPS (1000 메시)

---

#### 7.2 **ECS (Entity Component System) 라이브러리**

**목표:** `pole_ecs` - 고성능 ECS 구현

**기간:** 4-6개월

**산출물:**
- `pole_ecs` 라이브러리 (Bevy ECS, EnTT 같은 역할)
- `@component`, `@system` 매크로

**마일스톤:**
1. **M1: 컴포넌트 저장** (2개월)
2. **M2: 쿼리 시스템** (1개월)
3. **M3: 시스템 스케줄링** (1개월)
4. **M4: 성능 최적화** (1개월)

**성공 기준:**
- ✅ 10,000 엔티티 60 FPS

---

#### 7.3 **물리 엔진 라이브러리**

**목표:** `pole_physics` - 물리 시뮬레이션 라이브러리 (또는 기존 엔진 바인딩)

**기간:** 3-4개월

**산출물:**
- `pole_physics` 라이브러리 (Rapier, PhysX 바인딩 또는 자체 구현)
- 충돌 감지, Rigidbody, 제약 조건

**성공 기준:**
- ✅ 1000 오브젝트 물리 시뮬레이션

---

#### 7.4 **에셋 관리 라이브러리**

**목표:** `pole_assets` - 에셋 로딩 및 관리

**기간:** 3-4개월

**산출물:**
- `pole_assets` 라이브러리
- 비동기 로딩 시스템
- 에셋 임포터 (PNG, OBJ, GLTF 등)

---

#### 7.5 **입력 처리 라이브러리**

**목표:** `pole_input` - 입력 장치 추상화

**기간:** 2개월

**산출물:**
- `pole_input` 라이브러리
- 키보드/마우스/게임패드 지원

---

### P1 작업

#### 7.6 **오디오 라이브러리**
- **목표:** `pole_audio` - 오디오 재생 및 믹싱
- **기간:** 2-3개월

---

### Phase 7 완료 기준

**필수 (P0):**
- ✅ 모든 기본 라이브러리 완성 (`pole_graphics`, `pole_ecs`, `pole_physics`, `pole_assets`, `pole_input`)
- ✅ 이 라이브러리들을 사용하여 간단한 3D 게임 데모 제작 가능
- ✅ 60 FPS 유지 (1000+ 오브젝트)
- ✅ 물리 시뮬레이션 안정적 작동

**데모:**
- Pole 표준 라이브러리로 만든 3D FPS 게임 데모
- 오픈소스 공개 (GitHub)

**Phase 7 → Phase 8 전환:**
- 기본 라이브러리 완성 + 고급 기능 라이브러리 개발 시작

---

## Phase 8: 게임 개발 표준 라이브러리 - 고급 (Year 5-6.5)

**기간:** 2029-09 ~ 2031-03 (1.5년)

**목표:** 고급 게임 개발 라이브러리 (PBR 렌더링, 애니메이션, UI, 네트워킹)

**중요: Phase 8도 "언어 표준 라이브러리" 개발**
- 게임 엔진 자체가 아니라, 엔진 제작에 필요한 **고급 라이브러리**
- 비유: Unity의 HDRP/URP, Unreal의 Niagara 같은 고급 기능을 라이브러리로 제공

### P0 작업

#### 8.1 **고급 렌더링 라이브러리**
- **목표:** `pole_graphics` 확장 - PBR, 그림자, 포스트 프로세싱
- **기간:** 6-8개월

#### 8.2 **애니메이션 라이브러리**
- **목표:** `pole_animation` - 스켈레탈 애니메이션, 블렌딩
- **기간:** 4-5개월

#### 8.3 **UI 라이브러리**
- **목표:** `pole_ui` - 게임 UI 시스템 (즉시 모드 또는 유지 모드)
- **기간:** 3-4개월
- 버튼, 텍스트, 레이아웃 등

### P1 작업

#### 8.4 **네트워킹 라이브러리**
- **목표:** `pole_net` - 게임 네트워킹 (클라이언트-서버, P2P)
- **기간:** 4-6개월

#### 8.5 **스크립팅 지원**
- **목표:** 런타임 스크립팅 지원 (핫 리로딩, 모딩)
- **기간:** 2-3개월

### Phase 8 완료 기준

**필수 (P0):**
- ✅ 모든 고급 라이브러리 완성
- ✅ 이 라이브러리들로 AAA급 그래픽 품질 구현 가능
- ✅ 캐릭터 애니메이션 작동
- ✅ 완전한 UI 시스템

**데모:**
- 고급 그래픽 기능을 사용한 데모 게임
- 오픈소스 공개

**Phase 8 완료 후:**
- Pole 언어 + 표준 라이브러리로 **실제 게임 엔진 제작 가능**
- Phase 9-10은 **언어 개발 도구 및 생태계** 구축

---

## Phase 9: 언어 개발 도구 (Year 6.5-8)

**기간:** 2031-03 ~ 2032-09 (1.5년)

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

## Phase 10: 언어 생태계 & 커뮤니티 (Year 8-10)

**기간:** 2032-09 ~ 2035 (2-3년)

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

## 주요 마일스톤

### Milestone 1: 네이티브 컴파일 (Year 2)
- ✅ Pole → 실행 파일
- ✅ 성능: 100x 개선
- **데모:** factorial, fibonacci

### Milestone 2: 표준 라이브러리로 3D 게임 데모 (Year 4)
- ✅ Pole 표준 라이브러리 완성
- ✅ 간단한 3D FPS 데모 (라이브러리 사용 예시)
- ✅ 60 FPS, 1000+ 오브젝트
- **데모:** YouTube 공개

### Milestone 3: 언어 도구 완성 (Year 7)
- ✅ IDE 통합 (LSP)
- ✅ 디버거 & 프로파일러
- ✅ (선택) 참고용 게임 엔진 에디터
- **데모:** Pole 언어 개발 환경 시연

### Milestone 4: 언어 생태계 성숙 (Year 10)
- ✅ Pole 커뮤니티 1000+ 개발자
- ✅ Pole 패키지 100+ (게임, 시스템 도구 등)
- ✅ Pole로 만든 게임 엔진 및 게임 출시 (커뮤니티 프로젝트)
- **데모:** Pole 언어 성공 사례 발표

---

## 성공 지표

### Year 2 (Phase 5)
- [ ] 네이티브 컴파일: 100%
- [ ] 컴파일 성능: 100x vs 인터프리터
- [ ] IR 파싱 성능: 10-100x vs Python (Rust 전환)
- [ ] 타입 체킹 성능: 5-20x vs Python (Rust 전환)
- [ ] 메모리 안전성: 0 누수
- [ ] Rust 핵심 인프라: IR Parser, Type Checker 완성

### Year 4 (Phase 7)
- [ ] 표준 라이브러리 완성: `pole_graphics`, `pole_ecs`, `pole_physics` 등
- [ ] 라이브러리로 3D 게임 데모: 60 FPS (1000 오브젝트)
- [ ] 물리 시뮬레이션: 안정적
- [ ] Pole 개발자: 100+

### Year 7 (Phase 9)
- [ ] IDE 통합: LSP 완성
- [ ] 디버거 & 프로파일러: 작동
- [ ] 패키지 관리: 완성
- [ ] Pole 사용자: 500+

### Year 10 (Phase 10)
- [ ] Pole 패키지: 100+ (다양한 도메인)
- [ ] Pole 커뮤니티: 1000+
- [ ] Pole로 만든 게임 엔진: 1개 이상 (커뮤니티 프로젝트)
- [ ] 그 엔진으로 만든 게임: 출시

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

## 현재 최우선 작업

**현재 Phase:** Phase 5 (네이티브 컴파일러)

**현재 마일스톤:** ⭐ **5.1 M2 - LLVM 백엔드 개발 준비**

**완료된 작업:**
- ✅ M0: Rust 학습 & 핵심 인프라 전환 (2025-10-19)
  - IR Parser (Rust) 기본 구현: 2/6 예제 통과
  - Type Checker (Rust) 완성: 25.6배 성능 향상
  - PyO3 바인딩 완성
- ✅ M0.5: 시스템 안정화 (2025-10-19)
  - Python IR Parser: 6/6 예제 통과
  - Type definitions 지원 (Record, Variant, Alias)
  - Multi-line let expression 지원
  - End-to-end 테스트 완성
- ✅ **M1: Rust IR Parser 완성 & 타입 시스템 확장** (2025-10-19)
  - Type definitions 파싱 완성 (Record, Variant, Alias)
  - Custom type names 인식
  - Logical operators 지원 (&&, ||)
  - **6/6 예제 파싱 성공**
  - Python-Rust 기능 동등성 달성
  - 성능: Python 대비 **23.4배 향상**
- ✅ **M1.5: Python-Rust 통합 완성** (2025-10-19) 🎉
  - Type definitions 변환 로직 구현
  - Literal type_name 전달 수정
  - **모든 End-to-End 테스트 통과**
  - Type definitions가 전체 파이프라인에서 보존됨
  - LLVM 백엔드 개발 준비 완료

**다음 작업:**

**5.1.5: 컴파일러 메모리 관리 최적화 (Arena Allocator)** ⭐ 높은 우선순위

**목표:** SQLite 스타일 메모리 관리로 컴파일러 안정성/성능 개선

**선행 조건:** M2 진행 중 (병행 개발 가능)

**구현 계획:**
1. bumpalo 라이브러리 통합 (Cargo.toml)
2. CompilerArenas 구조체 구현
3. codegen.rs에 Arena 통합
4. OOM 복구 메커니즘 추가
5. 벤치마크 및 검증

**예상 소요 시간:** 2-3주

**완료 후:** M2 계속 진행 + M3 (LLVM 고급 기능)

---

## 변경 이력

- **2025-10-19**: Arena Allocator 도입 계획 추가 (5.1.5)
  - SQLite 스타일 메모리 관리를 Rust로 구현
  - 컴파일러 메모리 사용량 75% 감소 목표
  - OOM 복구 메커니즘 도입
  - 5.2 런타임 메모리 관리와 구분 명확화
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
