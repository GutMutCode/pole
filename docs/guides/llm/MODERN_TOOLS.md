# 현대적 LLM 도구 활용

> 2025년 현재 LLM 코딩 도구 생태계와 Pole에서의 활용

**최종 업데이트:** 2025-10-20  
**웹 검색 완료:** 2025-10-20 (Anthropic, OpenAI, GitHub)

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

### 7. Factory AI Droids ⭐⭐ Enterprise (2024-2025)

**최신 정보 (2025-10-20 확인):**
- **Series A $15M** (Sequoia 주도, 2025)
- **Agent-Native Development** (IDE, Web, CLI, Slack, Linear 통합)
- **Droids:** Code, Reliability, Knowledge, Tutorial 전문 agents
- **Enterprise 중심:** Fortune 500 기업 고객 (Clari 등)

**핵심 기능:**
- **Droids everywhere:** IDE, Web, CLI, Slack, Teams, Project Manager
- **Context Management:** Jira, Notion, Slack 자동 연동
- **MCP 통합:** Model Context Protocol 지원
- **Git 통합:** 자동 커밋, PR 생성
- **Specification Mode:** 계획 → 구현 워크플로우

**Droids 종류:**
- **Code Droid:** 코딩, 리팩토링, 디버깅
- **Reliability Droid:** 인시던트 관리, RCA 문서
- **Knowledge Droid:** 문서화, 지식 관리
- **Tutorial Droid:** Factory 학습

**설치:**
```bash
# macOS/Linux
curl -fsSL https://app.factory.ai/cli | sh

# Windows
irm https://app.factory.ai/cli/windows | iex

# 실행
droid
```

**장점:**
- **Enterprise급 보안 & 컴플라이언스**
- 모든 개발 인터페이스 통합 (가장 광범위)
- Sequoia 투자 (안정적 백킹)
- 전문화된 Droids (용도별 최적화)

**단점:**
- Enterprise 중심 (개인 사용자는 비쌈)
- 비공개 소스
- 가격 정보 미공개 (Contact Sales)

**Pole 적용:**
- ⚠️ **Enterprise 팀에게 적합**
- 개인/오픈소스 프로젝트엔 과함
- Slack/Linear 통합은 매력적
- MCP로 Pole 도구 연동 가능

---

### 1. GitHub Copilot (IDE 통합)

**핵심 기능:**
- **Agent Mode:** 백그라운드에서 이슈 처리 → PR 생성
- **Code Review:** AI가 코드 리뷰 수행
- **Multi-model:** GPT-5*, Claude Opus 4.1, Gemini 2.0 Flash  
  (*GPT-5는 아직 미출시, 현재는 GPT-4o)
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
- 최신 모델 지원 (Claude Sonnet 4.5, GPT-4o)
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

### 4. OpenCode (by SST) ⭐ 추천

**최신 정보 (2025-10-20 확인):**
- **현재 버전:** 0.15.8 (2025-10-18)
- **28.5k GitHub stars** (활발한 커뮤니티)
- **211 contributors** (오픈소스 성장 중)

**핵심 기능:**
- **100% 오픈소스** (MIT 라이선스)
- **터미널 TUI:** 터미널 UI에 특화
- **LSP 지원:** Language Server Protocol 내장
- **Client-Server 아키텍처:** 원격 제어 가능
- **Provider 독립적:** Anthropic, OpenAI, Google, Local

**장점:**
- 완전 오픈소스 (MIT)
- 터미널 네이티브
- 확장 가능한 아키텍처
- 프라이버시 우선
- 활발한 개발 (주 단위 릴리즈)

**단점:**
- 아직 베타 (0.15.x)
- Aider/Codex보다 덜 성숙

**Pole 적용:**
- ✅ **유망 후보**
- 아키텍처 철학이 Pole과 유사 (Provider 독립적)
- 터미널 중심
- 향후 통합 검토 가능

### 5. OpenAI Codex CLI ⭐⭐ 최신 (2025년 4월)

**최신 정보 (2025-10-20 확인):**
- **2025년 4월 16일** OpenAI 공식 출시
- **48.3k GitHub stars** (OpenAI 공식 프로젝트)
- **현재 버전:** 0.47.0 (2025-10-17)
- **188 contributors**

**핵심 기능:**
- **터미널 Agent:** 로컬에서 실행되는 경량 agent
- **ChatGPT 통합:** Plus/Pro/Team/Enterprise 플랜 사용
- **MCP 지원:** Model Context Protocol 서버 연동
- **Sandbox 모드:** 안전한 코드 실행 환경
- **다중 설치 방법:** npm, brew, 바이너리

**사용 예:**
```bash
npm install -g @openai/codex  # 또는 brew install codex
codex  # ChatGPT 계정으로 로그인
```

**장점:**
- **OpenAI 공식** (가장 최신 GPT 모델 사용)
- 터미널 네이티브
- ChatGPT 플랜 통합 (별도 비용 없음)
- Rust로 작성 (빠른 성능)
- 활발한 개발 (주 단위 업데이트)

**단점:**
- 아직 초기 단계 (0.47.x)
- OpenAI 모델에 종속적
- MCP 설정이 복잡

**Pole 적용:**
- ✅✅ **최우선 검토 대상**
- OpenAI 공식이라 안정적
- ChatGPT 플랜 활용 가능
- 터미널 중심 (Pole과 완벽한 궁합)

**참고:** 과거 "OpenAI Codex" API 모델(2021-2023)은 deprecated됨.  
현재는 **Codex CLI**가 공식 제품.

### 6. Claude Code ⭐ 공식 제품 (2025년 2월 출시)

**최신 정보 (2025-10-20 확인):**
- **2025년 2월 24일** Anthropic이 공식 출시
- **Claude 3.7 Sonnet** 기반 코딩 전용 agent
- **Extended Thinking** 지원 (추론 과정 표시)
- **터미널/IDE 통합** 가능

**핵심 기능:**
- **Agent 모드:** 자율적 코드 생성 & 수정
- **Plugin 시스템:** 확장 가능 (2025-10-09 발표)
- **MCP 통합:** Model Context Protocol 지원
- **Team/Enterprise 플랜** 지원

**Pole 적용:**
- ✅ **공식 Agent 도구로 유망**
- Aider와 비슷하지만 Anthropic 공식
- 향후 통합 검토 필요

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
aider --model deepseek --lint "pole check {file}" player.pole
# spec 읽기 → IR 생성 → 검증 → 재시도 → 커밋
```

---

## 도구별 사용 시나리오 (2025-10 기준)

| 작업 | 1순위 | 2순위 | 이유 |
|------|-------|-------|------|
| .pole → .pole-ir 생성 | **Codex CLI** | Aider | OpenAI 공식, ChatGPT 통합 |
| Rust 컴파일러 수정 | **Codex CLI** | Aider | 복잡한 리팩토링, MCP 지원 |
| 빠른 프로토타입 | **pole build** | Codex | 단순 생성은 기존 방식 |
| 대규모 리팩토링 | **Codex CLI** | Claude Code | Multi-file, Sandbox 지원 |
| 명세 작성 보조 | **pole build** | - | 명세 언어는 단순 |
| ChatGPT 플랜 보유 | **Codex CLI** | - | 별도 비용 없음 |
| Claude 선호 | **Claude Code** | Aider | Anthropic 공식 |
| Enterprise 팀 | **Factory Droids** | - | Slack/Jira 통합, 보안 |
| 인시던트 대응 | **Factory Droids** | - | Reliability Droid 전문 |

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

### 즉시 적용 가능 (2025-10 업데이트)

1. **OpenAI Codex CLI 통합** (1주) ⭐ 최우선
   - ChatGPT 플랜 활용 (무료 또는 기존 플랜)
   - MCP 서버로 Pole 도구 연동
   - Sandbox 모드로 안전한 실행

2. **Claude Code 통합** (1주)
   - Anthropic 공식 agent
   - Plugin 시스템 활용
   - Extended Thinking

3. **웹 검색 추가** (2주)
   - `--web-search` 플래그
   - 최신 API 문서 검색

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

3. **Pole의 방향성 (2025-10 업데이트)**
   - **단기:** Codex CLI/Claude Code 통합 (공식 도구 활용)
   - **중기:** MCP로 Pole 도구 노출
   - **장기:** Pole 자체가 agent 플랫폼

### 실용적 조언

```
현재: pole build (수동)
   ↓
단기: Codex CLI / Claude Code (공식 agent) ⭐ 변경됨
   ↓
중기: pole build --mcp (MCP 서버로 노출)
   ↓
장기: pole agent (자체 agent 플랫폼)

= Aider 대신 공식 도구 우선
```

---

## 관련 문서

- [LLM 활용법](USAGE.md)
- [LLM 한계](LIMITATIONS.md)
- [최신 정보 문제](LATEST_INFO.md)
- [README](README.md)

---

**교훈:** LLM의 한계는 도구 통합으로 극복한다. 🛠️
