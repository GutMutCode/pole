# 문서 재구성 계획

**작성일:** 2025-10-20

---

## 📊 현재 문서 현황 분석

### 루트 디렉토리 문서 (9개)
```
✅ README.md              - 프로젝트 소개 (유지)
✅ ROADMAP.md             - 전체 로드맵 (압축 완료, 218줄)
✅ DEVELOPMENT.md         - 개발 가이드 Index (새로 생성, 243줄)
✅ QUICKSTART.md          - 빠른 시작 (유지)
✅ ARCHITECTURE.md        - 시스템 아키텍처 (유지)
✅ CHANGELOG.md           - 변경 이력 (유지)

🔧 AGENTS.md              - LLM 에이전트 가이드 (검토 필요)
📦 ROADMAP-v1-prototype.md - 구 로드맵 (archive로 이동)
🔧 TEST_ISOMETRIC.md      - 테스트 노트 (삭제 또는 examples로)
```

### docs/ 디렉토리 (18개)
```
📋 완료 보고서 (reports/로 이동):
- PHASE_5.1_COMPLETION_REPORT.md
- PHASE_6.1_SDL2_COMPLETION.md
- M0.5_PROGRESS.md
- M0.5_WEEK2_COMPLETION.md
- M4_ADVANCED_TYPES_PROGRESS.md
- M5_RUNTIME_FUNCTIONS.md
- WEEK1_COMPLETION_REPORT.md
- WEEK2_COMPLETION_REPORT.md
- IR_PARSER_MULTIARG_FIX.md

🗺️ 로드맵 관련 (정리 필요):
- roadmaps/PROJECT_ZOMBOID_ROADMAP.md (중복, 삭제)
- roadmaps/game-development-general.md (archive)
- roadmaps/game-engine-vision.md (archive)

📚 기술 문서 (적절한 위치로):
- RUST_PARSER_MIGRATION.md (reports/)
- RUST_TYPE_CHECKER_MIGRATION.md (reports/)
- COMPILER_MEMORY_MANAGEMENT.md (guides/로 변환)
- ARENA_ALLOCATOR_STATUS.md (reports/)
- ARENA_ALLOCATOR_IMPACT.md (reports/)
- SDL2_RENDERING_DEMO.md (reports/)
- HYBRID_STRATEGY.md (archive, 구 전략)

✅ 새로 생성된 문서:
- DOCUMENTATION_STRATEGY.md (유지)
- DOCUMENTATION_COMPLETION_REPORT.md (유지)
- UNIFIED_DEVELOPMENT_STRATEGY.md (유지)
- WEEK1_PLAN.md (유지)
```

### specs/ 디렉토리 (6개)
```
✅ 모두 유지:
- syntax-v0.md           - 명세 언어 문법
- ir-syntax.md           - IR 문법
- ffi.md                 - FFI 명세
- implementation-lang.md - 구현 언어 명세
- verification.md        - 검증 시스템
- workflow.md            - LLM 워크플로
```

### 하위 프로젝트 문서 (4개)
```
✅ 유지:
- compiler/README.md
- compiler/docs/IR_TO_LLVM_MAPPING.md
- examples/README.md
- games/zomboid/README.md
- pole_engine/README.md
- docs/tutorials/FFI_TUTORIAL.md
```

---

## 🎯 재구성 계획

### Phase 1: 완료 보고서 이동
```bash
docs/reports/ 생성 및 이동:
- PHASE_5.1_COMPLETION_REPORT.md
- PHASE_6.1_SDL2_COMPLETION.md
- IR_PARSER_MULTIARG_FIX.md
- RUST_PARSER_MIGRATION.md
- RUST_TYPE_CHECKER_MIGRATION.md
- M0.5_PROGRESS.md
- M0.5_WEEK2_COMPLETION.md
- M4_ADVANCED_TYPES_PROGRESS.md
- M5_RUNTIME_FUNCTIONS.md
- WEEK1_COMPLETION_REPORT.md
- WEEK2_COMPLETION_REPORT.md
- ARENA_ALLOCATOR_STATUS.md
- ARENA_ALLOCATOR_IMPACT.md
- SDL2_RENDERING_DEMO.md
```

### Phase 2: Archive 이동
```bash
docs/archive/ 이동:
- ROADMAP-v1-prototype.md (이미 이동)
- HYBRID_STRATEGY.md (구 전략)
- roadmaps/game-development-general.md
- roadmaps/game-engine-vision.md
- roadmaps/examples/project-zomboid-clone.md
```

### Phase 3: 삭제
```bash
중복/불필요 문서:
- roadmaps/PROJECT_ZOMBOID_ROADMAP.md (ROADMAP.md와 중복)
- TEST_ISOMETRIC.md (일회성 테스트 노트)
```

### Phase 4: 새 가이드 생성
```bash
docs/guides/ 생성:
- LANGUAGE_DEV.md (언어 개발 가이드)
- ENGINE_DEV.md (엔진 개발 가이드)
- GAME_DEV.md (게임 개발 가이드)
- LLM_USAGE.md (LLM 활용 가이드)
- MEMORY_MANAGEMENT.md (메모리 관리, 기존 문서에서 추출)
```

### Phase 5: 로드맵 상세 생성
```bash
docs/roadmaps/ 생성:
- WEEKLY_PLANS.md (주간 계획 모음)
- LANGUAGE_ROADMAP.md (언어 로드맵 상세)
- ENGINE_ROADMAP.md (엔진 로드맵 상세)
- GAME_ROADMAP.md (게임 로드맵 상세)
- MILESTONES.md (마일스톤 추적)
```

### Phase 6: 특수 문서 처리
```bash
AGENTS.md 검토:
- LLM 에이전트 가이드
- docs/guides/AGENTS_GUIDE.md로 이름 변경 또는
- DEVELOPMENT.md에 통합
```

---

## 📂 최종 디렉토리 구조

```
pole/
  ├─ README.md                      # 프로젝트 소개
  ├─ QUICKSTART.md                  # 5분 시작
  ├─ ROADMAP.md                     # 로드맵 요약 (218줄)
  ├─ DEVELOPMENT.md                 # 개발 Index (243줄)
  ├─ ARCHITECTURE.md                # 아키텍처
  ├─ CHANGELOG.md                   # 변경 이력
  │
  ├─ docs/
  │   ├─ guides/                    # 개발 가이드
  │   │   ├─ LANGUAGE_DEV.md
  │   │   ├─ ENGINE_DEV.md
  │   │   ├─ GAME_DEV.md
  │   │   ├─ LLM_USAGE.md
  │   │   ├─ MEMORY_MANAGEMENT.md
  │   │   └─ AGENTS_GUIDE.md
  │   │
  │   ├─ roadmaps/                  # 로드맵 상세
  │   │   ├─ WEEKLY_PLANS.md
  │   │   ├─ LANGUAGE_ROADMAP.md
  │   │   ├─ ENGINE_ROADMAP.md
  │   │   ├─ GAME_ROADMAP.md
  │   │   └─ MILESTONES.md
  │   │
  │   ├─ reports/                   # 완료 보고서
  │   │   ├─ PHASE_5.1_COMPLETION.md
  │   │   ├─ PHASE_6.1_COMPLETION.md
  │   │   ├─ IR_PARSER_FIX.md
  │   │   └─ ... (14개 파일)
  │   │
  │   ├─ tutorials/                 # 튜토리얼
  │   │   └─ FFI_TUTORIAL.md
  │   │
  │   ├─ archive/                   # 구 문서
  │   │   ├─ ROADMAP-v1-prototype.md
  │   │   ├─ ROADMAP-v2-detailed.md
  │   │   └─ ... (구 전략 문서들)
  │   │
  │   ├─ DOCUMENTATION_STRATEGY.md  # 문서 전략
  │   ├─ UNIFIED_DEVELOPMENT_STRATEGY.md
  │   └─ WEEK1_PLAN.md
  │
  ├─ specs/                         # 언어 명세
  │   ├─ syntax-v0.md
  │   ├─ ir-syntax.md
  │   ├─ ffi.md
  │   └─ ...
  │
  ├─ compiler/
  │   ├─ README.md
  │   └─ docs/
  │       └─ IR_TO_LLVM_MAPPING.md
  │
  ├─ examples/
  │   └─ README.md
  │
  ├─ games/zomboid/
  │   ├─ README.md
  │   ├─ specs/
  │   └─ docs/
  │
  └─ pole_engine/
      ├─ README.md
      └─ docs/
```

---

## ✅ 작업 체크리스트

### Phase 1: 완료 보고서 이동
- [ ] reports/ 디렉토리 생성
- [ ] 14개 보고서 파일 이동
- [ ] 링크 업데이트

### Phase 2: Archive 정리
- [ ] 구 문서 5개 archive로 이동
- [ ] roadmaps/examples/ 삭제

### Phase 3: 불필요 문서 삭제
- [ ] PROJECT_ZOMBOID_ROADMAP.md 삭제
- [ ] TEST_ISOMETRIC.md 삭제

### Phase 4: 가이드 생성
- [ ] LANGUAGE_DEV.md 작성
- [ ] ENGINE_DEV.md 작성
- [ ] GAME_DEV.md 작성
- [ ] LLM_USAGE.md 작성
- [ ] MEMORY_MANAGEMENT.md 작성

### Phase 5: 로드맵 상세
- [ ] WEEKLY_PLANS.md 작성
- [ ] LANGUAGE_ROADMAP.md 작성
- [ ] ENGINE_ROADMAP.md 작성
- [ ] GAME_ROADMAP.md 작성
- [ ] MILESTONES.md 작성

### Phase 6: AGENTS.md 처리
- [ ] AGENTS_GUIDE.md로 이동 또는
- [ ] DEVELOPMENT.md에 통합

### Phase 7: 링크 검증
- [ ] 모든 상대 링크 확인
- [ ] 깨진 링크 수정
- [ ] 문서 크기 검증

---

## 📏 예상 결과

### 문서 수
- Before: 40개 (산재)
- After: 35개 (정리됨)
  - guides/: 6개
  - roadmaps/: 5개
  - reports/: 14개
  - tutorials/: 1개
  - archive/: 9개

### 평균 문서 크기
- 목표: 300-500줄
- Index 문서: 200줄
- 가이드: 400줄
- 보고서: 300줄

---

이 계획대로 진행하시겠습니까?
