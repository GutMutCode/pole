# Pole 언어 개발 가이드

> Pole 컴파일러, 타입 체커, LLM 통합 개발 가이드

**대상:** Pole 언어 코어 개발자  
**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [개발 환경 설정](#개발-환경-설정)
2. [프로젝트 구조](#프로젝트-구조)
3. [빌드 및 테스트](#빌드-및-테스트)
4. [주요 컴포넌트](#주요-컴포넌트)
5. [개발 워크플로우](#개발-워크플로우)
6. [문제 해결](#문제-해결)

---

## 개발 환경 설정

### 필수 도구

```bash
# Python 3.10+
python --version

# Rust 1.75+
rustc --version

# LLVM 17+
llc --version

# SDL2 (FFI 테스트)
sdl2-config --version
```

### 환경 변수

```bash
# LLM API Key (transformer 테스트용)
export OPENROUTER_API_KEY="your-key"

# Python 경로 (테스트용)
export PYTHONPATH=/home/gmc/Devs/pole/src
```

### 초기 설정

```bash
cd /home/gmc/Devs/pole

# Python 의존성
pip install -r requirements.txt

# Rust 컴파일러 빌드
cd compiler && cargo build --release
```

---

## 프로젝트 구조

```
pole/
  ├── src/pole/              # Python 구현 (Phase 0-4)
  │   ├── parser/            # 명세(.pole) 파서
  │   ├── transformer/       # LLM 변환기
  │   ├── runtime/           # IR 파서, 타입 체커 (레거시)
  │   └── compiler/          # LLVM 코드젠 (레거시)
  │
  ├── compiler/              # Rust 구현 (Phase 5+)
  │   ├── src/
  │   │   ├── ir_parser.rs   # IR 파서 (23x 빠름)
  │   │   ├── type_checker.rs # 타입 체커 (25x 빠름)
  │   │   └── codegen.rs     # LLVM 백엔드
  │   └── Cargo.toml
  │
  ├── specs/                 # 언어 명세
  │   ├── syntax-v0.md       # 명세 언어 문법
  │   ├── ir-syntax.md       # IR 문법
  │   ├── ffi.md             # FFI 명세
  │   └── workflow.md        # LLM 워크플로우
  │
  └── tests/                 # Python 테스트 (13개)
```

---

## 빌드 및 테스트

### Python 테스트

```bash
# 개별 테스트
python tests/test_parser.py
python tests/test_transformer.py
python tests/test_type_checker.py

# 전체 테스트 (13개 모듈)
pytest tests/
```

### Rust 컴파일러

```bash
cd compiler

# 빌드
cargo build --release

# 테스트
cargo test

# 벤치마크
cargo bench

# 예제 실행
cargo run --example benchmark_factorial
```

### 통합 테스트

```bash
# End-to-end 파이프라인
python tests/test_e2e_integration.py

# LLM 변환 (API key 필요)
python tests/test_llm_pipeline.py
```

---

## 주요 컴포넌트

### 1. Parser (명세 언어)

**위치:** `src/pole/parser/`

**역할:** `.pole` 파일을 AST로 파싱

```python
from pole.parser import parse_spec

spec = """
function factorial(n: Int) -> Int:
  purpose: "계승 계산"
  examples:
    - factorial(5) → 120
"""

ast = parse_spec(spec)
```

### 2. Transformer (LLM 통합)

**위치:** `src/pole/transformer/llm_transformer.py`

**역할:** 명세 → IR 변환 (LLM 사용)

```python
from pole.transformer import LLMTransformer

transformer = LLMTransformer()
ir_code = transformer.transform(spec_ast)
```

**관련 문서:**
- [LLM 워크플로우](../../specs/workflow.md)
- [LLM 활용 가이드](LLM_USAGE.md)

### 3. IR Parser (Rust)

**위치:** `compiler/src/ir_parser.rs`

**역할:** `.pole-ir` 파일을 IR AST로 파싱

**성능:** Python 대비 23.4x 빠름

```rust
use pole_compiler::parse_ir;

let ir_code = "def factorial(n: Int) -> Int = ...";
let ast = parse_ir(ir_code)?;
```

### 4. Type Checker (Rust)

**위치:** `compiler/src/type_checker.rs`

**역할:** IR 타입 검증

**성능:** Python 대비 25.6x 빠름

```rust
use pole_compiler::type_check;

let errors = type_check(&ast)?;
if errors.is_empty() {
    println!("Type check passed!");
}
```

### 5. Codegen (LLVM)

**위치:** `compiler/src/codegen.rs`

**역할:** IR → LLVM IR → 네이티브 바이너리

**상태:** 기본 구현 완료 (factorial 예제)

```rust
use pole_compiler::codegen;

let llvm_ir = codegen(&ast)?;
// LLVM 컴파일 → 실행 파일
```

---

## 개발 워크플로우

### 주간 사이클

**수요일: 언어 개선 데이**

1. 게임 개발(월-화)에서 발견된 이슈 리뷰
2. 우선순위 결정 (ROADMAP.md 업데이트)
3. 이슈 수정 또는 기능 추가
4. 테스트 작성 및 검증
5. 문서 업데이트

### 새 기능 추가 프로세스

#### 1. 명세 작성

`specs/` 디렉토리에 명세 추가

```markdown
# specs/new-feature.md

## 기능: 루프 구문

### 명세 언어
...

### IR 문법
...
```

#### 2. 파서 수정

**Python 파서** (`src/pole/parser/`)
```python
def parse_loop_statement(tokens):
    # 구현
```

**Rust IR 파서** (`compiler/src/ir_parser.rs`)
```rust
fn parse_loop(&mut self) -> Result<Expr> {
    // 구현
}
```

#### 3. 타입 체커 수정

```rust
fn type_check_loop(&mut self, expr: &Loop) -> Result<Type> {
    // 타입 검증 로직
}
```

#### 4. 코드젠 수정

```rust
fn codegen_loop(&mut self, expr: &Loop) -> Result<LLVMValue> {
    // LLVM IR 생성
}
```

#### 5. 테스트 작성

```python
# tests/test_loop.py

def test_loop_parsing():
    code = "loop i from 0 to 10: ..."
    ast = parse(code)
    assert ast.type == "Loop"

def test_loop_type_check():
    # 타입 체크 테스트

def test_loop_codegen():
    # 코드 생성 테스트
```

#### 6. 예제 추가

```
examples/XX-loop-test.pole-ir
```

### 버그 수정 프로세스

1. **재현 예제 작성**
   - 최소한의 코드로 버그 재현
   - `examples/XX-bug-name.pole-ir`

2. **테스트 작성**
   - 버그를 검증하는 테스트 추가
   - 현재는 실패해야 함

3. **수정**
   - 관련 컴포넌트 수정
   - 테스트가 통과할 때까지 반복

4. **문서화**
   - `docs/reports/BUG_NAME_FIX.md` 작성
   - 원인, 해결책, 영향 범위 기록

---

## 문제 해결

### 자주 발생하는 문제

#### 1. PYTHONPATH 에러

```bash
ModuleNotFoundError: No module named 'pole'
```

**해결:**
```bash
export PYTHONPATH=/home/gmc/Devs/pole/src
# 또는
python -m pytest tests/  # 프로젝트 루트에서 실행
```

#### 2. Rust 컴파일 에러

```bash
error: linking with `cc` failed
```

**해결:**
```bash
# LLVM 재설치
sudo apt install llvm-17 llvm-17-dev

# 또는 macOS
brew install llvm@17
```

#### 3. LLM API 에러

```bash
OpenRouterError: API key not found
```

**해결:**
```bash
export OPENROUTER_API_KEY="sk-or-..."
```

#### 4. SDL2 링크 에러

```bash
undefined reference to `SDL_Init`
```

**해결:**
```bash
# SDL2 설치
sudo apt install libsdl2-dev

# 또는 macOS
brew install sdl2
```

### 디버깅 팁

1. **Python 인터프리터 사용**
   ```bash
   pole run examples/01-factorial.pole-ir factorial 5
   ```

2. **Rust 디버그 빌드**
   ```bash
   cargo build  # --release 없이
   RUST_BACKTRACE=1 cargo test
   ```

3. **LLVM IR 출력**
   ```bash
   pole compile example.pole-ir --emit-llvm
   cat output.ll  # LLVM IR 확인
   ```

4. **타입 체크 상세 출력**
   ```bash
   pole check example.pole-ir --verbose
   ```

---

## 관련 문서

- [아키텍처](../../ARCHITECTURE.md) - 전체 시스템 구조
- [IR 문법](../../specs/ir-syntax.md) - IR 상세 명세
- [FFI 명세](../../specs/ffi.md) - FFI 시스템
- [메모리 관리](MEMORY_MANAGEMENT.md) - 컴파일러 메모리 전략
- [완료 보고서](../reports/) - 이전 작업 기록

---

## 성능 지표

### Rust 컴파일러 (Phase 5 완료)

- **IR Parser:** 23.4x faster (1.2ms → 0.051ms)
- **Type Checker:** 25.6x faster (0.8ms → 0.031ms)
- **E2E Pipeline:** 20x faster (3.5ms → 0.175ms)

### 목표 (Phase 6)

- **LLVM Codegen:** 100x faster
- **전체 컴파일:** < 10ms (factorial 예제)
- **메모리:** < 50MB (Arena Allocator)

---

**문의:** GitHub Issues 또는 Discord
