# Pole 개발 로드맵

> **통합 개발 전략**: 언어-엔진-게임 동시 개발
>
> Pole 언어를 개선하면서 Pole Engine과 Pole Zomboid를 함께 만듭니다.

**최종 업데이트:** 2025-10-20

---

## 🎯 핵심 전략

### 3-Track 병렬 개발

**Track 1: Pole 언어** (수요일, 주 1일)
- 컴파일러, 런타임, LLM 통합
- 게임 개발 중 발견한 문제 즉시 수정

**Track 2: Pole Engine** (목-금, 주 2일)  
- 재사용 가능한 2D 게임 엔진 코드
- 게임에서 추출한 검증된 패턴

**Track 3: Pole Zomboid** (월-화, 주 2일)
- Project Zomboid 클론 게임
- 명세(.pole) 작성 → LLM 생성(.pole-ir)

### 피드백 루프

```
게임 개발 → 언어 문제 발견 → 즉시 수정 → 
엔진 패턴 추출 → 게임에 재적용 → 반복
```

---

## 📍 현재 상태 (2025-10-20)

### ✅ 완료된 것

**Pole 언어 (Phase 0-6):**
- ✅ 명세 언어(.pole) + IR(.pole-ir) + LLM 변환
- ✅ Rust 컴파일러 (23x 성능 향상)
- ✅ LLVM 네이티브 컴파일 (100x 성능)
- ✅ FFI 시스템 (SDL2 통합)
- ✅ Multi-arg 함수 지원
- ✅ 고급 타입 (Record, List, Option, Ptr<T>)

**Pole Engine (초기):**
- ✅ 72개 예제 코드
- ✅ 타일맵 렌더링 (100x100)
- ✅ 좀비 AI (100+ 엔티티)
- ✅ 네트워킹 (TCP)

**완료된 작업 상세:**
- [Phase 5.1 LLVM 백엔드](docs/reports/PHASE_5.1_COMPLETION_REPORT.md)
- [Phase 6.1 FFI 시스템](docs/reports/PHASE_6.1_SDL2_COMPLETION.md)
- [IR Parser Multi-arg 버그 수정](docs/reports/IR_PARSER_MULTIARG_FIX.md)

### 🎯 이번 주 목표 (Week 1)

**목표:** 1분 플레이 가능한 데모
- [ ] player.pole, zombie.pole 명세 작성
- [ ] LLM 생성 & 컴파일
- [ ] 플레이어 WASD 이동 + 좀비 1마리 추적
- [ ] YouTube 데모 영상

**자세한 계획:** [Week 1 계획](docs/WEEK1_PLAN.md)

---

## 🗓️ 타임라인 (2년)

### 3개월 목표 (2026-01)
- Pole 언어: 10개 기능 개선 (루프, 배열, 메모리)
- Pole Engine: 10개 모듈 (render, input, physics)
- Pole Zomboid: 10분 플레이 (전투, 인벤토리)

### 6개월 목표 (2026-04)
- Pole 언어: 멀티스레드, LSP 기초
- Pole Engine: 완전한 2D 엔진 (15개 모듈)
- Pole Zomboid: 1시간 콘텐츠, 2-4인 Co-op

### 1년 목표 (2026-10)
- Pole 언어: 1.0 RC, 디버거
- Pole Engine: 1.0 릴리스, 문서화 완료
- Pole Zomboid: 플레이어블 데모, 크라우드펀딩

### 2년 목표 (2027-10)
- Pole 언어: 1.0 릴리스, 프로덕션 레디
- Pole Engine: 다른 게임 제작 가능
- Pole Zomboid: Steam Early Access

**상세 마일스톤:** [마일스톤 추적](docs/roadmaps/MILESTONES.md)

---

## 📚 주요 문서

### 시작하기
- [빠른 시작](QUICKSTART.md) - 5분 안에 Pole 시작
- [개발 가이드](DEVELOPMENT.md) - 개발자용 Index
- [아키텍처](ARCHITECTURE.md) - 시스템 구조

### 개발 가이드
- [언어 개발](docs/guides/LANGUAGE_DEV.md) - Pole 언어 개발
- [엔진 개발](docs/guides/ENGINE_DEV.md) - Pole Engine 개발  
- [게임 개발](docs/guides/GAME_DEV.md) - Pole Zomboid 개발
- [LLM 활용](docs/guides/LLM_USAGE.md) - LLM 효과적 사용

### 로드맵 상세
- [주간 계획](docs/roadmaps/WEEKLY_PLANS.md) - 주간 작업 모음
- [언어 로드맵](docs/roadmaps/LANGUAGE_ROADMAP.md) - Pole 언어 상세
- [엔진 로드맵](docs/roadmaps/ENGINE_ROADMAP.md) - Pole Engine 상세
- [게임 로드맵](docs/roadmaps/GAME_ROADMAP.md) - Pole Zomboid 상세

### 기술 명세
- [명세 언어 문법](specs/syntax-v0.md)
- [IR 문법](specs/ir-syntax.md)
- [FFI 명세](specs/ffi.md)

---

## 🔄 주간 개발 사이클

### 월-화: 게임 개발 (Track 3)
1. 새 기능 명세 작성 (.pole)
2. LLM으로 구현 생성 (`pole build`)
3. 컴파일 & 테스트
4. 통합 & 플레이

### 수: 언어 개선 (Track 1)
1. 발견된 문제 리뷰
2. 언어 기능 추가/수정
3. LLM 프롬프트 개선
4. 테스트 & 검증

### 목-금: 엔진 리팩토링 (Track 2)
1. 게임 코드 분석
2. Pole Engine 모듈 작성
3. 문서화
4. 다음 주 사용 준비

### 토: 통합 테스트
- 전체 파이프라인 검증
- 성능 측정
- 주간 데모 제작

### 일: 계획 & 홍보
- 다음 주 우선순위
- YouTube/블로그
- 커뮤니티 피드백

**상세 사이클 설명:** [통합 개발 전략](docs/UNIFIED_DEVELOPMENT_STRATEGY.md)

---

## 📊 Pole Engine 구조

```
pole_engine/
  ├── render/      # SDL2 렌더링
  ├── input/       # 키보드, 마우스
  ├── core/        # 수학, 시간, 타입
  ├── physics/     # 충돌 감지
  ├── ai/          # 경로 찾기
  └── network/     # 멀티플레이어
```

**자세한 구조:** [Pole Engine 문서](pole_engine/README.md)

---

## 🎮 Pole Zomboid 시스템

**핵심 시스템:**
- Player (이동, 상태)
- Zombie AI (추적, 공격)
- Combat (전투, 데미지)
- Inventory (아이템 관리)
- Survival (배고픔, 갈증)
- Network (멀티플레이어)

**설계 문서:** [게임 시스템](games/zomboid/docs/SYSTEMS.md)

---

## ✅ 핵심 장점

1. **빠른 피드백** - 매주 플레이 가능한 결과물
2. **실전 검증** - 실제 필요한 기능만 개발
3. **재미** - 지루하지 않음, 계속 새로운 기능
4. **리스크 분산** - 하나 막혀도 다른 부분 진행

---

## 🔗 관련 링크

### 문서
- [문서 전략](docs/DOCUMENTATION_STRATEGY.md) - 문서 구조 설명
- [변경 이력](CHANGELOG.md) - 전체 변경 기록

### 프로젝트
- [Pole Language](README.md) - 메인 프로젝트
- [Pole Engine](pole_engine/README.md) - 2D 엔진
- [Pole Zomboid](games/zomboid/README.md) - 게임 프로젝트

### 커뮤니티
- GitHub Issues - 버그 리포트
- Discord - 토론 (예정)
- YouTube - 주간 데모

---

**마지막 업데이트:** 2025-10-20
**다음 마일스톤:** Week 1 완료 (2025-10-26)
**현재 작업:** player.pole 명세 작성

**Let's build something amazing! 🎮✨**
