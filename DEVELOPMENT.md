# Pole Development Guide

> 개발 환경 설정 및 개발 가이드

## 요구사항

- Python 3.11 이상
- pip

## 개발 환경 설정

### 1. 저장소 클론

```bash
git clone <repository-url>
cd pole
```

### 2. 가상 환경 생성 (권장)

```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
# or
venv\Scripts\activate  # Windows
```

### 3. 개발 의존성 설치

```bash
make dev-install
```

## 개발 명령어

### 테스트

```bash
make test
```

### 린팅

```bash
make lint
```

### 포매팅

```bash
make format
```

### 타입 체크

```bash
make typecheck
```

### 모든 검사 실행

```bash
make lint && make typecheck && make test
```

## 프로젝트 구조

```
pole/
├── src/pole/           # 소스 코드
│   ├── parser/         # .pole 파일 파서
│   ├── validator/      # 명세 검증기
│   ├── transformer/    # LLM 변환기
│   ├── runtime/        # IR 인터프리터
│   ├── verifier/       # 검증 시스템 (타입 체커, 테스트 실행기)
│   └── cli/            # CLI 도구
├── tests/              # 테스트 코드
├── specs/              # 언어 사양 문서
├── examples/           # 예제 프로그램
└── pyproject.toml      # 프로젝트 설정
```

## 개발 워크플로우

### 1. 기능 개발

1. 새 브랜치 생성
2. 코드 작성
3. 테스트 작성
4. 린팅 및 타입 체크 통과
5. 커밋 및 PR 생성

### 2. 코드 스타일

- **포매터**: Black (line-length=100)
- **린터**: Ruff
- **타입 체크**: mypy (strict mode)

### 3. 커밋 메시지

명확하고 간결한 커밋 메시지 작성:

```
Add type checker for basic types

- Implement type checking for Int, Nat, Bool, String
- Add test cases for type inference
```

## 의존성 관리

### 프로덕션 의존성

- `anthropic`: Claude API 연동
- `openai`: GPT API 연동

### 개발 의존성

- `pytest`: 테스트 프레임워크
- `pytest-cov`: 코드 커버리지
- `mypy`: 타입 체커
- `ruff`: 린터
- `black`: 코드 포매터

## 문제 해결

### 가상 환경이 활성화되지 않음

```bash
source venv/bin/activate
```

### 의존성 설치 실패

```bash
pip install --upgrade pip
make dev-install
```

### 테스트 실패

```bash
pytest tests/ -v  # 자세한 출력
pytest tests/ -x  # 첫 실패 시 중단
```

## 참고 문서

- [README.md](README.md) - 프로젝트 개요 및 설계 원칙
- [ARCHITECTURE.md](ARCHITECTURE.md) - 시스템 아키텍처
- [ROADMAP.md](ROADMAP.md) - 개발 로드맵
- [AGENTS.md](AGENTS.md) - AI 에이전트 개발 가이드라인
- [specs/](specs/) - 언어 사양 문서
