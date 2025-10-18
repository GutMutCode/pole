# Pole Development Guide

> 개발 환경 설정 및 개발 가이드

## 요구사항

### Phase 0-4 (Python - 현재)
- Python 3.11 이상
- pip

### Phase 5+ (Rust 컴파일러)
- Rust 1.75+ (2024 Edition)
- Cargo (Rust 패키지 매니저)
- LLVM 17.0+ (시스템 설치)
- Python 3.11+ (CLI 및 바인딩용)

## 개발 환경 설정

### Phase 0-4: Python 개발 환경

#### 1. 저장소 클론

```bash
git clone <repository-url>
cd pole
```

#### 2. 가상 환경 생성 (권장)

```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
# or
venv\Scripts\activate  # Windows
```

#### 3. 개발 의존성 설치

```bash
make dev-install
```

---

### Phase 5+: Rust 컴파일러 개발 환경

#### 1. Rust 설치

```bash
# rustup 설치 (공식 설치 방법)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rust 버전 확인
rustc --version  # 1.75 이상 필요
cargo --version

# 최신 안정 버전으로 업데이트
rustup update stable
```

#### 2. LLVM 설치

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install llvm-17 llvm-17-dev libpolly-17-dev
```

**macOS (Homebrew):**
```bash
brew install llvm@17
export LLVM_SYS_170_PREFIX="$(brew --prefix llvm@17)"
```

**Arch Linux:**
```bash
sudo pacman -S llvm
```

#### 3. Rust 개발 도구 설치

```bash
# rust-analyzer (IDE 지원)
rustup component add rust-analyzer

# clippy (린터)
rustup component add clippy

# rustfmt (포매터)
rustup component add rustfmt
```

#### 4. 컴파일러 빌드

```bash
# Rust 컴파일러 빌드
cd compiler
cargo build --release

# Python 바인딩 빌드 (PyO3 + maturin)
pip install maturin
maturin develop  # 개발 모드
# 또는
maturin build --release  # 릴리스 빌드
```

---

## 개발 명령어

### Python (Phase 0-4)

**테스트:**
```bash
make test
```

**린팅:**
```bash
make lint
```

**포매팅:**
```bash
make format
```

**타입 체크:**
```bash
make typecheck
```

**모든 검사 실행:**
```bash
make lint && make typecheck && make test
```

---

### Rust (Phase 5+)

**빌드:**
```bash
cd compiler
cargo build          # 디버그 빌드
cargo build --release  # 릴리스 빌드 (최적화)
```

**테스트:**
```bash
cd compiler
cargo test           # 모든 테스트 실행
cargo test --release  # 릴리스 모드 테스트
```

**린팅:**
```bash
cd compiler
cargo clippy         # Rust 린터
cargo clippy -- -D warnings  # 경고를 에러로
```

**포매팅:**
```bash
cd compiler
cargo fmt            # 코드 포매팅
cargo fmt -- --check  # 포맷 체크만
```

**문서 생성:**
```bash
cd compiler
cargo doc --open     # 문서 생성 및 브라우저 열기
```

**벤치마크:**
```bash
cd compiler
cargo bench          # 성능 벤치마크
```

---

## 프로젝트 구조

```
pole/
├── src/pole/               # Python 소스 코드 (Phase 0-4)
│   ├── parser/             # .pole 파일 파서
│   ├── validator/          # 명세 검증기
│   ├── transformer/        # LLM 변환기
│   ├── runtime/            # IR 인터프리터
│   ├── verifier/           # 검증 시스템 (타입 체커, 테스트 실행기)
│   ├── cli/                # CLI 도구
│   └── compiler/           # Rust 바인딩 (Phase 5+)
│       └── bindings.py     # PyO3 Python 바인딩
├── compiler/               # Rust 컴파일러 (Phase 5+, 별도 crate)
│   ├── src/
│   │   ├── lib.rs          # 라이브러리 루트
│   │   ├── ir_to_llvm.rs   # IR → LLVM IR 변환
│   │   ├── codegen.rs      # 코드 생성
│   │   ├── optimization.rs # 최적화
│   │   └── memory/         # 메모리 관리
│   │       ├── mod.rs
│   │       ├── gc.rs       # 가비지 컬렉션
│   │       └── allocator.rs # 커스텀 할당자
│   ├── Cargo.toml          # Rust 프로젝트 설정
│   ├── build.rs            # 빌드 스크립트
│   └── tests/              # Rust 통합 테스트
├── tests/                  # Python 테스트 코드
├── specs/                  # 언어 사양 문서
├── examples/               # 예제 프로그램
├── pyproject.toml          # Python 프로젝트 설정
└── Makefile                # 빌드 스크립트
```

---

## 개발 워크플로우

### 1. 기능 개발

1. 새 브랜치 생성
2. 코드 작성
3. 테스트 작성
4. 린팅 및 타입 체크 통과
5. 커밋 및 PR 생성

### 2. 코드 스타일

**Python:**
- **포매터**: Black (line-length=100)
- **린터**: Ruff
- **타입 체크**: mypy (strict mode)

**Rust:**
- **포매터**: rustfmt (Rust 표준)
- **린터**: clippy (Rust 표준)
- **네이밍**: Rust 컨벤션 (snake_case, CamelCase)

### 3. 커밋 메시지

명확하고 간결한 커밋 메시지 작성:

```
Add type checker for basic types

- Implement type checking for Int, Nat, Bool, String
- Add test cases for type inference
```

---

## 의존성 관리

### Python (Phase 0-4)

**프로덕션 의존성:**
- `anthropic`: Claude API 연동
- `openai`: GPT API 연동

**개발 의존성:**
- `pytest`: 테스트 프레임워크
- `pytest-cov`: 코드 커버리지
- `mypy`: 타입 체커
- `ruff`: 린터
- `black`: 코드 포매터

---

### Rust (Phase 5+)

**주요 의존성 (Cargo.toml):**
- `llvm-sys = "170"` 또는 `inkwell = "0.4"`: LLVM 바인딩
- `pyo3 = "0.20"`: Python 연동
- `anyhow = "1.0"`: 에러 처리
- `thiserror = "1.0"`: 커스텀 에러 타입
- `clap = "4.0"`: CLI 파싱 (선택사항)

**개발 의존성:**
- `criterion = "0.5"`: 벤치마킹
- `proptest = "1.0"`: 속성 기반 테스트
- `pretty_assertions = "1.0"`: 테스트 출력 개선

**빌드 도구:**
- `maturin = "1.4"`: Python wheel 빌드
- `cc = "1.0"`: C 코드 빌드 (LLVM 연동 시)

---

## Rust 학습 리소스 (Phase 5 준비)

### 필수 학습 자료

**1. Rust 기초 (4-6주)**
- [The Rust Programming Language](https://doc.rust-lang.org/book/) (The Book)
  - Chapter 1-10: 기본 문법 및 소유권
  - Chapter 15-17: 스마트 포인터, 동시성, OOP
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - 실습 위주 학습

**2. 고급 Rust (2-4주)**
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (Unsafe Rust)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

**3. LLVM 학습 (4-6주)**
- [LLVM Tutorial](https://llvm.org/docs/tutorial/)
- [Kaleidoscope Tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/)
- [llvm-sys Rust Crate](https://docs.rs/llvm-sys/)
- [inkwell - 고수준 LLVM 래퍼](https://github.com/TheDan64/inkwell)

**4. PyO3 학습 (1-2주)**
- [PyO3 User Guide](https://pyo3.rs/)
- [maturin 빌드 도구](https://www.maturin.rs/)

### 추천 학습 순서

```
Week 1-6:   Rust 기초 (The Book + Rust by Example)
Week 7-10:  고급 Rust (Unsafe, 매크로, 트레이트)
Week 11-16: LLVM 튜토리얼 (간단한 컴파일러 구현)
Week 17-18: PyO3 통합 (Python-Rust 바인딩)
Week 19+:   Pole 컴파일러 개발 시작
```

### 실습 프로젝트 (학습용)

1. **간단한 계산기 인터프리터** (Rust로 구현)
2. **LLVM "Hello World" 컴파일러** (LLVM IR 생성)
3. **간단한 함수형 언어 컴파일러** (Kaleidoscope 따라하기)
4. **Python-Rust 통합 예제** (PyO3로 Rust 함수 노출)

---

## 문제 해결

### Python 환경

**가상 환경이 활성화되지 않음:**
```bash
source venv/bin/activate
```

**의존성 설치 실패:**
```bash
pip install --upgrade pip
make dev-install
```

**테스트 실패:**
```bash
pytest tests/ -v  # 자세한 출력
pytest tests/ -x  # 첫 실패 시 중단
```

---

### Rust 환경

**LLVM 찾을 수 없음:**
```bash
# LLVM 경로 설정
export LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
# 또는 macOS
export LLVM_SYS_170_PREFIX="$(brew --prefix llvm@17)"
```

**링크 에러:**
```bash
# LLVM 라이브러리 경로 추가
export LD_LIBRARY_PATH=/usr/lib/llvm-17/lib:$LD_LIBRARY_PATH
```

**maturin 빌드 실패:**
```bash
# Python 헤더 설치 (Ubuntu)
sudo apt-get install python3-dev

# maturin 재설치
pip install --upgrade maturin
```

---

## 참고 문서

- [README.md](README.md) - 프로젝트 개요 및 설계 원칙
- [ARCHITECTURE.md](ARCHITECTURE.md) - 시스템 아키텍처
- [ROADMAP.md](ROADMAP.md) - 개발 로드맵
- [AGENTS.md](AGENTS.md) - AI 에이전트 개발 가이드라인
- [specs/](specs/) - 언어 사양 문서
