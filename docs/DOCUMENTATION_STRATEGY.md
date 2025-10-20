# Pole 문서 전략

> **목표:** LLM context 한계 극복 + 빠른 정보 접근
>
> 작은 문서들을 링크로 연결하여 필요한 정보만 읽기

---

## 🎯 설계 원칙

### 1. 작은 문서 (Small Documents)
- **각 문서 최대 500줄** (LLM이 한 번에 읽기 적합)
- 단일 주제에 집중
- 명확한 목적

### 2. 링크 중심 (Link-Driven)
- 문서 간 링크로 연결
- 계층적 구조 (Index → Category → Detail)
- 순환 참조 금지

### 3. 즉시 사용 가능 (Action-Oriented)
- 추상적 설명보다 구체적 예제
- 체크리스트 제공
- 복사 가능한 코드 스니펫

---

## 📚 문서 계층 구조

```
README.md (프로젝트 소개 + Index)
  ├─ QUICKSTART.md (5분 안에 시작)
  ├─ ROADMAP.md (전체 계획 요약)
  │   ├─ docs/roadmaps/WEEKLY_PLANS.md
  │   └─ docs/roadmaps/MILESTONES.md
  │
  ├─ ARCHITECTURE.md (시스템 구조)
  │
  ├─ DEVELOPMENT.md (개발 가이드 Index)
  │   ├─ docs/guides/LANGUAGE_DEV.md
  │   ├─ docs/guides/ENGINE_DEV.md
  │   └─ docs/guides/GAME_DEV.md
  │
  └─ docs/ (상세 문서)
      ├─ guides/ (가이드)
      ├─ roadmaps/ (로드맵 상세)
      ├─ specs/ (언어 명세)
      └─ reports/ (완료 보고서)
```

---

## 📝 문서 유형별 가이드

### Type 1: Index 문서
**목적:** 빠른 네비게이션
**크기:** 100-200줄
**예시:** README.md, DEVELOPMENT.md

**구조:**
```markdown
# 문서 제목

> 한 줄 설명

## Quick Links
- [시작하기](link1)
- [가이드](link2)

## 주요 섹션
간단한 설명 + 링크
```

### Type 2: Guide 문서
**목적:** 특정 작업 수행 방법
**크기:** 300-500줄
**예시:** WEEK1_PLAN.md

**구조:**
```markdown
# 가이드 제목

## 목표
명확한 목표 1-2줄

## 체크리스트
- [ ] 항목 1
- [ ] 항목 2

## 단계별 가이드
구체적 명령어 + 예제
```

### Type 3: Spec 문서
**목적:** 기술 명세
**크기:** 200-400줄
**예시:** specs/ir-syntax.md

**구조:**
```markdown
# 명세 제목

## 문법
BNF 또는 예제

## 예시
실제 코드
```

### Type 4: Report 문서
**목적:** 완료된 작업 기록
**크기:** 300-500줄
**예시:** IR_PARSER_MULTIARG_FIX.md

**구조:**
```markdown
# 작업 제목

## 요약
3-5줄

## 문제
구체적 설명

## 해결
코드 + 결과
```

---

## 🔗 링크 전략

### 상대 링크 사용
```markdown
✅ 좋음: [가이드](docs/guides/LANGUAGE_DEV.md)
❌ 나쁨: [가이드](/home/user/pole/docs/...)
```

### 섹션 링크
```markdown
[특정 섹션](#섹션-제목)
```

### 외부 링크 최소화
```markdown
# 필요시만 외부 링크
[Rust Book](https://doc.rust-lang.org/book/)
```

---

## 📂 새로운 파일 구조

```
pole/
  ├─ README.md                    # 프로젝트 소개 + Index
  ├─ QUICKSTART.md                # 5분 시작 가이드
  ├─ ROADMAP.md                   # 전체 로드맵 요약 (200줄)
  ├─ ARCHITECTURE.md              # 시스템 아키텍처
  ├─ DEVELOPMENT.md               # 개발 가이드 Index
  │
  ├─ docs/
  │   ├─ guides/                  # 개발 가이드
  │   │   ├─ LANGUAGE_DEV.md      # 언어 개발 가이드
  │   │   ├─ ENGINE_DEV.md        # 엔진 개발 가이드
  │   │   ├─ GAME_DEV.md          # 게임 개발 가이드
  │   │   └─ LLM_USAGE.md         # LLM 활용 가이드
  │   │
  │   ├─ roadmaps/                # 로드맵 상세
  │   │   ├─ WEEKLY_PLANS.md      # 주간 계획 모음
  │   │   ├─ MILESTONES.md        # 마일스톤 추적
  │   │   └─ LANGUAGE_ROADMAP.md  # 언어 로드맵 상세
  │   │
  │   ├─ specs/                   # 기술 명세
  │   │   ├─ syntax-v0.md
  │   │   ├─ ir-syntax.md
  │   │   └─ ffi.md
  │   │
  │   ├─ reports/                 # 완료 보고서
  │   │   ├─ IR_PARSER_FIX.md
  │   │   └─ WEEK1_REPORT.md
  │   │
  │   └─ archive/                 # 구 문서 보관
  │       └─ ROADMAP-v1.md
  │
  ├─ games/zomboid/
  │   └─ docs/
  │       ├─ DESIGN.md            # 게임 디자인
  │       └─ SYSTEMS.md           # 시스템 설계
  │
  └─ pole_engine/
      └─ docs/
          ├─ API.md               # API 레퍼런스
          └─ MODULES.md           # 모듈 가이드
```

---

## 🎯 LLM Context 최적화 전략

### 1. 계층적 읽기
```
LLM이 필요한 정보 찾기:
1. README.md 읽기 → 전체 구조 파악
2. Index 문서 읽기 → 관련 섹션 찾기
3. 상세 문서 읽기 → 구체적 정보
```

### 2. 문서당 1개 주제
```
✅ 좋음:
- LANGUAGE_DEV.md: 언어 개발만
- ENGINE_DEV.md: 엔진 개발만

❌ 나쁨:
- EVERYTHING.md: 모든 내용
```

### 3. 중복 최소화, 링크 최대화
```
✅ 좋음:
"자세한 내용은 [FFI 가이드](../specs/ffi.md) 참고"

❌ 나쁨:
FFI 내용을 여러 문서에 복사
```

---

## 📏 문서 크기 가이드라인

| 문서 유형 | 최대 줄 수 | 예상 단어 수 |
|----------|----------|------------|
| Index    | 200줄    | 1,000 단어 |
| Guide    | 500줄    | 2,500 단어 |
| Spec     | 400줄    | 2,000 단어 |
| Report   | 500줄    | 2,500 단어 |

**측정 방법:**
```bash
wc -l docs/guides/LANGUAGE_DEV.md
# 출력: 350 lines (✅ 500줄 이하)
```

---

## ✅ 문서 작성 체크리스트

### 새 문서 작성 시
- [ ] 제목이 명확한가?
- [ ] 한 줄 설명이 있는가?
- [ ] 500줄 이하인가?
- [ ] 관련 문서 링크가 있는가?
- [ ] 예제가 포함되어 있는가?
- [ ] 작성 일자가 있는가?

### 기존 문서 수정 시
- [ ] 500줄 넘으면 분리했는가?
- [ ] 링크가 깨지지 않았는가?
- [ ] 수정 일자를 업데이트했는가?

---

## 🔄 문서 유지보수

### 주간 점검 (일요일)
```bash
# 1. 깨진 링크 확인
grep -r "]\(" docs/ | grep -v "http"

# 2. 500줄 넘는 문서 찾기
find docs/ -name "*.md" -exec wc -l {} \; | awk '$1 > 500'

# 3. 오래된 문서 찾기 (6개월 이상 수정 없음)
find docs/ -name "*.md" -mtime +180
```

### 분기별 정리 (3개월마다)
- [ ] 사용하지 않는 문서 → archive/
- [ ] 중복 내용 통합
- [ ] 링크 구조 최적화

---

## 💡 실전 예시

### Before (나쁜 예)
```
ROADMAP.md (2000줄)
  - 전체 로드맵
  - 주간 계획
  - 언어 상세
  - 엔진 상세
  - 게임 상세
  - 변경 이력
```
→ LLM이 읽기 힘듦, 정보 찾기 어려움

### After (좋은 예)
```
ROADMAP.md (200줄)
  - 전체 개요
  - 현재 상태
  - 다음 단계
  - 링크:
    - [주간 계획](docs/roadmaps/WEEKLY_PLANS.md)
    - [언어 로드맵](docs/roadmaps/LANGUAGE_ROADMAP.md)
    - [엔진 로드맵](docs/roadmaps/ENGINE_ROADMAP.md)
```
→ LLM이 필요한 것만 읽기 가능

---

## 🚀 적용 계획

### Phase 1: 현재 문서 분석 (완료)
- [x] 기존 문서 크기 측정
- [x] 중복 내용 파악

### Phase 2: 구조 재설계 (이번 작업)
- [x] 새 파일 구조 설계
- [ ] Index 문서 작성
- [ ] 기존 문서 분리

### Phase 3: 마이그레이션 (다음 작업)
- [ ] ROADMAP.md 분리
- [ ] 상세 문서 이동
- [ ] 링크 업데이트

### Phase 4: 검증
- [ ] 모든 링크 작동 확인
- [ ] LLM으로 테스트 (정보 찾기)
- [ ] 문서 크기 확인

---

이 전략으로 LLM이 필요한 정보만 효율적으로 읽을 수 있습니다! 📚✨
