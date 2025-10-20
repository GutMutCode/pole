# 현대적 LLM 도구 활용

> 2025년 현재 LLM 코딩 도구 생태계와 Pole에서의 활용

**최종 업데이트:** 2025-10-20

---

## 목차

1. [현대 LLM 도구 개요](#현대-llm-도구-개요)
2. [도구별 비교](#도구별-비교)
3. [Pole 프로젝트 적용 전략](#pole-프로젝트-적용-전략)
4. [권장 워크플로우](#권장-워크플로우)

---

## 현대 LLM 도구 개요

### LLM 도구의 진화

**과거 (2023):**
- LLM API 직접 호출
- 단순 프롬프트 → 응답
- 컨텍스트 제한
- 도구 통합 없음

**현재 (2025):**
- **Agent 기반 코딩**
- **도구 통합** (파일 시스템, Git, 웹 검색)
- **멀티 모델 지원**
- **IDE/터미널 통합**
- **자동 실행 & 검증**

### 핵심 개념

#### 1. Agentic Coding
LLM이 단순 생성을 넘어 자율적으로:
- 파일 읽기/쓰기
- 명령어 실행
- 에러 확인 & 재시도
- Git 커밋

#### 2. Tool Use (Function Calling)
LLM이 외부 도구 호출:
- 파일 시스템 접근
- 웹 검색 (최신 정보)
- 터미널 명령 실행
- API 호출

#### 3. Codebase Indexing
전체 코드베이스 이해:
- AST 분석
- 의존성 그래프
- 벡터 임베딩

---

## 도구별 비교

### 1. GitHub Copilot (IDE 통합)

**핵심 기능:**
- **Agent Mode:** 백그라운드에서 이슈 처리 → PR 생성
- **Code Review:** AI가 코드 리뷰 수행
- **Multi-model:** GPT-5, Claude Opus 4.1, Gemini 2.0 Flash
- **Spaces:** 팀별 컨텍스트 구성

**장점:**
- IDE 네이티브 통합 (VS Code, JetBrains)
- GitHub 생태계 (Issues, PR, Actions)
- 엔터프라이즈 지원
- 무료 티어 (월 2000 completions)

**단점:**
- GitHub 종속적
- 비교적 비쌈 (Pro: $10/월, Pro+: $39/월)
- 터미널 경험은 제한적

**Pole 적용:**
- ❌ 현재 Pole CLI 중심이라 불필요
- ⚠️ 향후 VS Code 확장 고려 시 검토

### 2. Cursor (IDE 재발명)

**핵심 기능:**
- **Agent Mode:** 아이디어 → 코드 자동 생성
- **Tab Completion:** 커스텀 모델로 정확한 자동완성
- **Codebase Indexing:** 전체 프로젝트 이해
- **Multi-file Editing:** 여러 파일 동시 수정

**장점:**
- VS Code 기반 (익숙한 UX)
- 최신 모델 지원 (GPT-5, Claude Sonnet 4.5)
- Tab 모델 성능 우수 (21% fewer suggestions, 28% higher accept)
- Fortune 500 절반이 사용

**단점:**
- IDE 교체 필요 (Lock-in)
- 비쌈 ($20/월)
- 터미널 워크플로우 제한

**Pole 적용:**
- ❌ Pole은 명세 기반 개발 (IDE 자동완성과 방향 다름)
- ⚠️ 게임 개발자용 보조 도구로는 가능

### 3. Aider (터미널 AI 페어 프로그래밍) ⭐ 추천

**핵심 기능:**
- **터미널 네이티브:** CLI 기반 워크플로우
- **Codebase Map:** 전체 프로젝트 자동 매핑
- **Git 통합:** 자동 커밋 + 의미 있는 메시지
- **Lint & Test:** 자동 린팅 + 테스트 실행
- **100+ 언어 지원**
- **로컬/클라우드 LLM 지원**

**사용 예:**
```bash
aider --model deepseek
> Add multi-argument function support to ir_parser.rs
# → 코드 분석 → 생성 → 테스트 → 커밋
```

**장점:**
- 터미널 중심 (Pole 워크플로우와 일치)
- 모델 독립적 (DeepSeek, Claude, GPT 등)
- Git 통합 (자동 커밋)
- 로컬 모델 지원
- 무료 오픈소스

**Pole 적용:**
- ✅ **최적 후보!**
- 명세 작성 → Aider로 구현 생성
- Git 자동 커밋
- 테스트 자동 실행

### 4. OpenCode (터미널 AI Agent) ⭐ 추천

**핵심 기능:**
- **100% 오픈소스**
- **터미널 TUI:** 터미널 UI에 특화
- **LSP 지원:** Language Server Protocol 내장
- **Client-Server 아키텍처:** 원격 제어 가능
- **Provider 독립적:** Anthropic, OpenAI, Google, Local

**장점:**
- 완전 오픈소스 (MIT)
- 터미널 네이티브
- 확장 가능한 아키텍처
- 프라이버시 우선
- 28.5k GitHub stars

**단점:**
- 아직 베타 (0.15.x)
- Aider보다 덜 성숙

**Pole 적용:**
- ✅ **유망 후보**
- 아키텍처 철학이 Pole과 유사 (Provider 독립적)
- 터미널 중심
- 향후 통합 검토 가능

### 5. Claude Code (공식 제품 없음)

**상태:** Anthropic은 "Claude Code"라는 독립 제품을 출시하지 않았습니다.
- Claude API를 사용한 도구들이 "Claude Code"로 불림
- GitHub Copilot, Cursor 등이 Claude 모델 사용

---

## Pole 프로젝트 적용 전략

### 현재 Pole 워크플로우

```
1. 개발자: .pole 명세 작성 (자연어 + 타입)
   ↓
2. pole build: LLM API 호출 → .pole-ir 생성
   ↓
3. Rust: 타입 체크
   ↓
4. LLVM: 네이티브 컴파일
```

### 문제점

1. **웹 검색 없음:** 최신 API 정보 부족
2. **도구 통합 제한:** 파일 읽기만 가능, 실행 불가
3. **단일 생성:** 재시도 수동
4. **검증 수동:** 타입 체크 별도 실행

### 개선 방향 (2025 현대적 접근)

#### 전략 1: Aider 통합 (단기, 추천) ⭐

**구조:**
```bash
# 1. 명세 작성
vim player.pole

# 2. Aider로 IR 생성
aider player.pole --model deepseek
> Generate Pole IR implementation from this spec

# 3. Aider가 자동:
- IR 파서 호출
- 타입 체크
- 에러 시 재시도
- Git 커밋
```

**장점:**
- 즉시 사용 가능 (통합 최소)
- Git 자동 커밋
- 테스트 자동화
- 웹 검색 가능 (--web-search 플래그)

**구현:** `scripts/aider_pole.sh` - Aider 래퍼 스크립트로 자동 검증 & 커밋

#### 전략 2: Pole CLI에 Tool Use 추가 (중기)

**기능:** `pole build --web-search` - LLM이 최신 API 문서 자동 검색

**장점:** 최신 정보 자동 검색, 예제 자동 참조, 검증 자동화  
**단점:** 개발 시간 2-3주, 보안 고려 필요

#### 전략 3: OpenCode 통합 (장기)

**비전:** `pole agent build` - 다중 파일 자동 분석 → IR 생성 → 통합 테스트 → 커밋

**장점:** 완전 자동화, 다중 파일 통합, 복잡한 워크플로우 처리  
**단점:** OpenCode 아직 베타, 깊은 통합 필요 (3-6개월)

---

## 권장 워크플로우

### Phase 1: Aider 보조 사용 (현재~3개월)

```bash
# 기존 워크플로우 유지 + Aider 보조

# 1. 명세 작성 (기존)
vim games/zomboid/player.pole

# 2. LLM 생성 (기존)
pole build games/zomboid/player.pole

# 3. 실패 시 Aider 사용 (신규)
aider games/zomboid/player.pole-ir \
  --message "Fix type errors. See specs/ir-syntax.md" \
  --lint "pole check player.pole-ir"

# 4. 성공 시 자동 커밋 (Aider)
```

**이점:**
- 점진적 도입
- 기존 워크플로우 유지
- 수동 재시도 제거

### Phase 2: Tool Use 통합 (3-6개월)

```bash
pole build player.pole --web-search
# LLM이 SDL2 최신 문서 검색 → IR 생성
```

### Phase 3: 완전 자동화 (6-12개월)

```bash
pole agent build games/zomboid/specs/*.pole
# 다중 파일 분석 → 의존성 해결 → IR 생성 → 통합 테스트 → 커밋
```

---

## 실전 예제

### 예제: Aider 워크플로우

```bash
# IR 생성 + 자동 검증
aider --model deepseek \
  --lint "pole check {file}" \
  games/zomboid/specs/player.pole

> Generate Pole IR from spec. Follow specs/ir-syntax.md

# Aider가 자동으로: spec 읽기 → IR 생성 → 검증 → 재시도 → 커밋
```

---

## 도구별 사용 시나리오

| 작업 | 추천 도구 | 이유 |
|------|-----------|------|
| .pole → .pole-ir 생성 | **Aider** | 터미널 중심, Git 통합 |
| Rust 컴파일러 수정 | **Aider** | 복잡한 리팩토링 |
| 빠른 프로토타입 | **pole build** (기존) | 단순 생성 |
| 대규모 리팩토링 | **OpenCode** (미래) | Multi-file 지원 |
| 명세 작성 보조 | **pole build** | 명세 언어는 단순 |

---

## 현재 상태 요약

### 2025년 현재

**LLM 도구 생태계:**
- Agent 기반 코딩이 표준
- Tool Use로 최신 정보 접근
- Git/IDE/터미널 깊은 통합
- 멀티 모델 지원

**Pole 현재:**
- 단순 LLM API 호출
- 웹 검색 없음
- 수동 재시도
- Git 통합 없음

### 즉시 적용 가능

1. **Aider 통합** (1주)
   - `pole-aider` 래퍼 스크립트
   - 자동 커밋
   - 자동 재시도

2. **웹 검색 추가** (2주)
   - `--web-search` 플래그
   - 최신 API 문서 검색

3. **Tool Use** (1개월)
   - Claude/GPT Function Calling
   - 파일 읽기, 명령 실행

---

## 결론

### 핵심 교훈

1. **LLM만으로는 부족** (2023 방식)
   - 최신 정보 없음
   - 도구 통합 없음
   - 수동 작업 많음

2. **Agent + Tools = 현대적 접근** (2025)
   - 웹 검색으로 최신 정보
   - 자동 검증 & 재시도
   - Git 자동화

3. **Pole의 방향성**
   - 단기: Aider 보조 사용
   - 중기: Tool Use 통합
   - 장기: 완전 자동화

### 실용적 조언

```
현재: pole build (수동)
   ↓
단기: pole build + Aider (반자동)
   ↓
중기: pole build --web-search (자동)
   ↓
장기: pole agent (완전 자동)

= 점진적 진화
```

---

## 관련 문서

- [LLM 활용법](USAGE.md)
- [LLM 한계](LIMITATIONS.md)
- [최신 정보 문제](LATEST_INFO.md)
- [README](README.md)

---

**교훈:** LLM의 한계는 도구 통합으로 극복한다. 🛠️
