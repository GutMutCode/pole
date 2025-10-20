# Pole 개발 가이드

> **개발자용 Index** - 필요한 가이드를 빠르게 찾기

---

## 🚀 빠른 시작

### 처음 시작하는 분
1. [빠른 시작 가이드](QUICKSTART.md) - 5분 안에 Pole 시작
2. [아키텍처 이해](ARCHITECTURE.md) - 시스템 구조 파악
3. [예제 실행](examples/README.md) - 예제 코드 실행

### 기여하고 싶은 분
1. [개발 환경 설정](#개발-환경-설정)
2. [작업할 분야 선택](#작업-분야)
3. [해당 가이드 읽기](#개발-가이드)

---

## 🔧 개발 환경 설정

### 필수 도구
```bash
# Pole CLI
pole --version

# Rust 컴파일러
rustc --version

# Python
python3 --version

# SDL2
sdl2-config --version

# OpenRouter API Key
echo $OPENROUTER_API_KEY
```

### 설치 가이드
- [Pole CLI 설치](QUICKSTART.md#설치)
- [개발 도구 설정](docs/guides/SETUP.md)

---

## 📂 작업 분야

### Track 1: Pole 언어 개발
**작업 내용:**
- 컴파일러 개선 (Rust)
- IR Parser / Type Checker
- LLM 통합
- 런타임 함수

**가이드:** [언어 개발 가이드](docs/guides/LANGUAGE_DEV.md)

**주요 파일:**
- `compiler/` - Rust 컴파일러
- `src/pole/` - Python 도구
- `specs/` - 언어 명세

### Track 2: Pole Engine 개발
**작업 내용:**
- 엔진 모듈 작성 (.pole 명세)
- 재사용 가능한 코드 추출
- API 설계
- 문서화

**가이드:** [엔진 개발 가이드](docs/guides/ENGINE_DEV.md)

**주요 파일:**
- `pole_engine/` - 엔진 코드
- `pole_engine/docs/` - API 문서

### Track 3: Pole Zomboid 개발
**작업 내용:**
- 게임 기능 명세 작성
- LLM으로 구현 생성
- 게임플레이 테스트
- 버그 수정

**가이드:** [게임 개발 가이드](docs/guides/GAME_DEV.md)

**주요 파일:**
- `games/zomboid/specs/` - 게임 명세
- `games/zomboid/docs/` - 설계 문서

---

## 📚 개발 가이드

### 언어 개발
- [Pole 언어 개발](docs/guides/LANGUAGE_DEV.md)
  - 컴파일러 수정
  - 새 언어 기능 추가
  - 타입 시스템 확장
  
- [LLM 활용 가이드](docs/guides/llm/)
  - [기본 활용법](docs/guides/llm/USAGE.md)
  - [LLM 한계](docs/guides/llm/LIMITATIONS.md)
  - [최신 정보 문제](docs/guides/llm/LATEST_INFO.md) ⭐

### 엔진 개발
- [Pole Engine 개발](docs/guides/ENGINE_DEV.md)
  - 모듈 구조
  - API 설계 원칙
  - 재사용 패턴

- [모듈별 가이드](pole_engine/docs/MODULES.md)
  - render/ 렌더링
  - input/ 입력 처리
  - physics/ 물리 시스템

### 게임 개발
- [Pole Zomboid 개발](docs/guides/GAME_DEV.md)
  - 명세 작성법
  - LLM 생성 워크플로
  - 디버깅 팁

- [시스템별 가이드](games/zomboid/docs/SYSTEMS.md)
  - Player 시스템
  - AI 시스템
  - 네트워크

---

## 🗓️ 개발 워크플로

### 주간 사이클
- **월-화**: 게임 개발 (명세 작성 → LLM 생성)
- **수요일**: 언어 개선 (발견된 문제 수정)
- **목-금**: 엔진 리팩토링 (재사용 코드 추출)
- **토요일**: 통합 테스트
- **일요일**: 계획 & 홍보

**자세한 사이클:** [통합 개발 전략](docs/UNIFIED_DEVELOPMENT_STRATEGY.md)

### 이번 주 계획
현재 주차 계획은 항상 여기 참고:
- [Week 1 계획](docs/WEEK1_PLAN.md)
- [주간 계획 모음](docs/roadmaps/WEEKLY_PLANS.md)

---

## 🛠️ 일반 작업

### 코드 작성
```bash
# 1. 명세 작성
vim games/zomboid/specs/player.pole

# 2. LLM 생성
pole build games/zomboid/specs/player.pole

# 3. 타입 체크
pole check games/zomboid/specs/player.pole-ir

# 4. 테스트
pole test games/zomboid/specs/player.pole-ir
```

### 디버깅
```bash
# 인터프리터 실행
pole run examples/01-factorial.pole-ir factorial 5

# 상세 로그
RUST_LOG=debug pole build ...

# Python 디버거
python -m pdb src/pole/cli/main.py build ...
```

### 테스트
```bash
# Python 테스트
python tests/test_parser.py

# Rust 테스트
cd compiler && cargo test

# 통합 테스트
./test_all_examples.py
```

---

## 📖 참고 자료

### 내부 문서
- [ROADMAP](ROADMAP.md) - 전체 로드맵
- [ARCHITECTURE](ARCHITECTURE.md) - 아키텍처
- [문서 전략](docs/DOCUMENTATION_STRATEGY.md) - 문서 구조

### 언어 명세
- [명세 언어](specs/syntax-v0.md) - .pole 문법
- [IR 문법](specs/ir-syntax.md) - .pole-ir 문법
- [FFI 명세](specs/ffi.md) - C 함수 호출

### 완료 보고서
- [Phase 5.1 완료](docs/reports/PHASE_5.1_COMPLETION.md)
- [Phase 6.1 완료](docs/reports/PHASE_6.1_COMPLETION.md)
- [Multi-arg 버그 수정](docs/IR_PARSER_MULTIARG_FIX.md)

---

## 💬 커뮤니티

### 질문하기
- GitHub Issues - 버그, 기능 요청
- Discord (예정) - 실시간 토론

### 기여하기
- Pull Request 환영
- 코드 리뷰 참여
- 문서 개선

---

## 🎯 다음 단계

### 언어 개발자
1. [언어 로드맵](docs/roadmaps/LANGUAGE_ROADMAP.md) 확인
2. 우선순위 높은 이슈 선택
3. [언어 개발 가이드](docs/guides/LANGUAGE_DEV.md) 참고
4. 개발 시작!

### 엔진 개발자
1. [엔진 로드맵](docs/roadmaps/ENGINE_ROADMAP.md) 확인
2. 모듈 선택 (render, physics 등)
3. [엔진 개발 가이드](docs/guides/ENGINE_DEV.md) 참고
4. 명세 작성!

### 게임 개발자
1. [게임 로드맵](docs/roadmaps/GAME_ROADMAP.md) 확인
2. 시스템 선택 (Player, AI 등)
3. [게임 개발 가이드](docs/guides/GAME_DEV.md) 참고
4. 명세 작성!

---

Happy Coding! 🚀
