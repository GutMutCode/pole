# Pole 통합 개발 로드맵

> **새로운 전략 (2025-10-20)**: 언어-엔진-게임 동시 개발
>
> Pole 언어를 개선하면서 동시에 Pole Engine과 Pole Zomboid를 만듭니다.
> 실전 개발에서 발견된 문제를 즉시 언어에 반영하여 빠르게 진화시킵니다.

**이전 로드맵 (Phase 0-4 프로토타입)**: [ROADMAP-v1-prototype.md](ROADMAP-v1-prototype.md)

---

## 🎯 핵심 전략: 3-Track 병렬 개발

### Track 1: Pole 언어 (기반 기술)
**역할:** 컴파일러, 런타임, LLM 통합
**산출물:** 작동하는 프로그래밍 언어

### Track 2: Pole Engine (엔진 라이브러리)
**역할:** 2D 게임 엔진 (렌더링, 물리, 네트워킹)
**산출물:** Pole로 작성된 재사용 가능한 엔진 코드

### Track 3: Pole Zomboid (실전 프로젝트)
**역할:** Project Zomboid 클론 게임
**산출물:** 플레이 가능한 생존 게임

### 🔄 피드백 루프

```
Pole Zomboid 개발
    ↓ (언어 기능 필요)
Pole 언어 개선
    ↓ (새 기능 추가)
Pole Engine 확장
    ↓ (검증된 패턴)
Pole Zomboid에 적용
    ↓ (반복)
```

---

## 📊 현재 상태 (2025-10-20)

### ✅ 완료된 것

**Pole 언어 (Phase 0-6):**
- ✅ 명세 언어 (.pole) 파서
- ✅ IR (.pole-ir) 인터프리터
- ✅ LLM 변환기 (OpenRouter 통합)
- ✅ Rust IR Parser/Type Checker (23x 성능 향상)
- ✅ LLVM 네이티브 컴파일 (100x 성능)
- ✅ FFI 시스템 (C 함수 호출)
- ✅ Multi-arg 함수 지원
- ✅ 고급 타입 (Record, List, Option, Ptr<T>)
- ✅ SDL2 렌더링

**Pole Engine (초기):**
- ✅ 72개 예제 코드 (수동 작성)
- ✅ 타일맵 렌더링 (100x100)
- ✅ 좀비 AI (100+ 엔티티)
- ✅ 네트워킹 (TCP 소켓)
- ✅ 파일 I/O

### 🚀 즉시 목표 (1주일)

**Track 1 (언어):**
- [ ] LLM 생성 코드 품질 개선 (null, 타입 캐스팅)
- [ ] System prompt 개선 (더 많은 예제)

**Track 2 (엔진):**
- [ ] 기존 코드를 .pole 명세로 변환 (3-5개 핵심 모듈)
- [ ] Pole Engine 아키텍처 설계

**Track 3 (게임):**
- [ ] 인벤토리 시스템 명세 작성
- [ ] 전투 시스템 명세 작성
- [ ] LLM 생성 → 컴파일 → 테스트

---

## 🗓️ 통합 개발 타임라인 (2년)

### Year 1: 기반 구축 (2025-2026)

#### Q1 (2025-11 ~ 2026-01): 엔진 핵심
**Pole 언어:**
- [ ] 루프 구문 (for, while)
- [ ] 배열/HashMap 완전 지원
- [ ] 에러 처리 개선

**Pole Engine:**
- [ ] `pole_render` 모듈 (SDL2 래퍼)
- [ ] `pole_ecs` 모듈 (Entity-Component)
- [ ] `pole_input` 모듈 (키보드/마우스)

**Pole Zomboid:**
- [ ] 플레이어 이동 (WASD)
- [ ] 타일맵 렌더링
- [ ] 기본 UI

#### Q2 (2026-02 ~ 2026-04): 게임플레이
**Pole 언어:**
- [ ] 메모리 최적화 (Arena allocator)
- [ ] 동시성 (pthread FFI)
- [ ] 디버깅 도구

**Pole Engine:**
- [ ] `pole_physics` 모듈 (충돌 감지)
- [ ] `pole_ai` 모듈 (경로 찾기)
- [ ] `pole_sound` 모듈 (오디오)

**Pole Zomboid:**
- [ ] 좀비 AI (100+ 좀비)
- [ ] 전투 시스템
- [ ] 인벤토리/제작
- [ ] 생존 시스템

#### Q3 (2026-05 ~ 2026-07): 멀티플레이어
**Pole 언어:**
- [ ] 비동기 I/O
- [ ] 직렬화/역직렬화
- [ ] 네트워크 프로토콜 지원

**Pole Engine:**
- [ ] `pole_network` 모듈 (서버/클라이언트)
- [ ] 상태 동기화
- [ ] 패킷 압축

**Pole Zomboid:**
- [ ] 2-4인 Co-op
- [ ] 서버 호스팅
- [ ] 상태 동기화

#### Q4 (2026-08 ~ 2026-10): 완성도
**Pole 언어:**
- [ ] 성능 최적화 (SIMD)
- [ ] 크로스 플랫폼 빌드
- [ ] LSP 기초

**Pole Engine:**
- [ ] 프로파일링 도구
- [ ] 에셋 파이프라인
- [ ] 저장/로드 시스템

**Pole Zomboid:**
- [ ] 3개 맵
- [ ] 50+ 아이템
- [ ] 퀘스트 시스템
- [ ] 첫 플레이어블 데모

### Year 2: 출시 준비 (2027)

#### Q1-Q2: 콘텐츠 확장
- Pole Zomboid: 10시간 플레이 콘텐츠
- Pole Engine: 안정화 및 최적화
- Pole 언어: 버그 수정

#### Q3: Early Access
- Steam 출시
- 커뮤니티 피드백 수집
- 언어/엔진/게임 모두 공개

#### Q4: 정식 출시
- 20-30시간 콘텐츠
- Pole Engine 1.0 릴리스
- Pole 언어 1.0 릴리스

---

## 📋 주간 개발 사이클

### 매주 반복되는 패턴

**월요일-화요일: Track 3 (게임 개발)**
- 새로운 기능 명세 작성 (.pole)
- LLM으로 구현 생성 (.pole-ir)
- 컴파일 및 테스트

**수요일: Track 1 (언어 개선)**
- 게임 개발 중 발견한 문제 수정
- 필요한 언어 기능 추가
- LLM 프롬프트 개선

**목요일-금요일: Track 2 (엔진 리팩토링)**
- 게임 코드에서 재사용 가능한 부분 추출
- Pole Engine 모듈로 정리
- 문서화

**토요일: 통합 테스트**
- 전체 파이프라인 검증
- 성능 측정
- 주간 데모 제작

**일요일: 계획 및 문서**
- 다음 주 작업 우선순위 결정
- 로드맵 업데이트
- 블로그/YouTube 컨텐츠

---

## 🎮 Pole Zomboid 개발 로드맵

### Phase 1: 핵심 게임플레이 (Month 1-3)

**Week 1-2: 플레이어 & 맵**
```pole
// specs/player.pole
type Player:
  purpose: "플레이어 캐릭터 상태"
  fields:
    - position: (Int, Int)  // 타일 좌표
    - health: Int           // 0-100
    - hunger: Int           // 0-100
    - inventory: Inventory

function move_player(player: Player, direction: Direction) -> Player:
  purpose: "플레이어를 특정 방향으로 이동"
  input: "플레이어와 이동 방향"
  output: "업데이트된 플레이어 상태"
  constraints:
    - "맵 경계 체크"
    - "충돌 감지"
  examples:
    - move_player(player_at_5_5, North) → player_at_5_4
```

**Week 3-4: 좀비 AI**
```pole
// specs/zombie.pole
type Zombie:
  fields:
    - position: (Int, Int)
    - health: Int
    - state: ZombieState  // Idle, Chase, Attack

function update_zombie(zombie: Zombie, player: Player, dt: Float64) -> Zombie:
  purpose: "좀비 AI 업데이트"
  constraints:
    - "플레이어가 10타일 이내면 추적"
    - "1타일 이내면 공격"
```

**Week 5-6: 전투**
```pole
// specs/combat.pole
function attack(attacker: Entity, target: Entity, weapon: Weapon) -> CombatResult:
  purpose: "공격 판정 및 데미지 계산"
  constraints:
    - "명중률은 무기와 거리에 비례"
    - "크리티컬 10% 확률"
```

**Week 7-8: 인벤토리**
```pole
// specs/inventory.pole (이미 작성됨!)
// LLM 생성 완료, 테스트 중
```

**Week 9-12: 생존 시스템**
```pole
// specs/survival.pole
function update_hunger(player: Player, dt: Float64) -> Player:
  purpose: "시간에 따른 배고픔 증가"
  constraints:
    - "1시간마다 10 감소"
    - "0이 되면 체력 감소 시작"
```

### Phase 2: 멀티플레이어 (Month 4-6)

**Network Protocol**
```pole
// specs/network.pole
type GameState:
  fields:
    - players: List<Player>
    - zombies: List<Zombie>
    - items: List<Item>

function serialize_state(state: GameState) -> String:
  purpose: "게임 상태를 JSON으로 직렬화"

function sync_to_clients(server: Server, state: GameState) -> Unit:
  purpose: "모든 클라이언트에 상태 브로드캐스트"
```

### Phase 3: 콘텐츠 (Month 7-12)

- 다양한 맵 (도시, 숲, 농장)
- 100+ 아이템
- 건축 시스템
- 날씨/시간

---

## 🔧 Pole Engine 아키텍처

### 모듈 구조

```
pole_engine/
  ├── render/          # SDL2 렌더링 래퍼
  │   ├── window.pole
  │   ├── texture.pole
  │   ├── sprite.pole
  │   └── tilemap.pole
  │
  ├── ecs/             # Entity-Component-System
  │   ├── entity.pole
  │   ├── component.pole
  │   └── system.pole
  │
  ├── physics/         # 2D 물리
  │   ├── collision.pole
  │   └── raycast.pole
  │
  ├── input/           # 입력 처리
  │   ├── keyboard.pole
  │   └── mouse.pole
  │
  ├── network/         # 네트워킹
  │   ├── server.pole
  │   ├── client.pole
  │   └── protocol.pole
  │
  ├── ai/              # AI 시스템
  │   ├── pathfinding.pole
  │   └── behavior.pole
  │
  └── core/            # 핵심 유틸리티
      ├── math.pole
      ├── memory.pole
      └── time.pole
```

### 개발 원칙

1. **명세 우선**: 모든 모듈은 .pole 명세로 시작
2. **LLM 생성**: 구현은 LLM이 생성 (.pole-ir)
3. **검증 필수**: 생성된 코드는 타입 체크 + 테스트 필수
4. **재사용성**: 게임 특화 코드는 나중에 추출

---

## 🛠️ Pole 언어 개선 로드맵

### Priority 0: 게임 개발 즉시 필요

**Week 1-2:**
- [ ] 루프 구문 (for i in 0..10)
- [ ] 배열 인덱싱 최적화
- [ ] null/nil 대신 Option<T> 가이드

**Week 3-4:**
- [ ] 메모리 관리 (Object Pool)
- [ ] 에러 처리 개선 (Result<T,E>)

### Priority 1: 성능 최적화

**Month 2-3:**
- [ ] SIMD 벡터화
- [ ] 멀티스레드 지원
- [ ] 인라인 최적화

### Priority 2: 개발 경험

**Month 4-6:**
- [ ] LSP (VSCode 통합)
- [ ] 디버거 (breakpoint)
- [ ] 프로파일러

---

## 📈 성공 지표

### 3개월 목표 (2026-01)
- [ ] Pole Zomboid: 1분 플레이 가능 (이동, 좀비 1마리)
- [ ] Pole Engine: 5개 모듈 완성
- [ ] Pole 언어: 루프, 배열, 메모리 관리

### 6개월 목표 (2026-04)
- [ ] Pole Zomboid: 10분 게임플레이 (전투, 인벤토리)
- [ ] Pole Engine: 10개 모듈, 문서화
- [ ] Pole 언어: 멀티스레드, 성능 최적화

### 1년 목표 (2026-10)
- [ ] Pole Zomboid: 1시간 콘텐츠, 2-4인 Co-op
- [ ] Pole Engine: 완전한 2D 엔진 (15개 모듈)
- [ ] Pole 언어: LSP, 디버거, 안정화

### 2년 목표 (2027-10)
- [ ] Pole Zomboid: Steam Early Access
- [ ] Pole Engine: 1.0 릴리스, 다른 게임 제작 가능
- [ ] Pole 언어: 1.0 릴리스, 프로덕션 레디

---

## 🚀 당장 시작할 작업 (이번 주)

### Day 1-2: 명세 작성
```bash
# 핵심 게임 시스템 명세 작성
games/zomboid/specs/
  ├── player.pole          # 플레이어 이동, 상태
  ├── zombie.pole          # 좀비 AI
  ├── combat.pole          # 전투 시스템 (이미 있음)
  ├── inventory.pole       # 인벤토리 (이미 있음)
  └── survival.pole        # 생존 시스템 (이미 있음)
```

### Day 3: LLM 생성
```bash
# 각 명세를 IR로 변환
pole build games/zomboid/specs/player.pole
pole build games/zomboid/specs/zombie.pole

# 타입 체크
pole check games/zomboid/specs/player.pole-ir
```

### Day 4-5: 통합 테스트
```bash
# 간단한 데모 만들기
# 플레이어가 WASD로 이동
# 좀비 1마리가 플레이어 추적
# 타일맵 렌더링

examples/67-simple-game.pole-ir
```

### Day 6: 엔진 추출
```bash
# 재사용 가능한 부분 추출
pole_engine/
  └── render/
      └── sprite.pole  # 스프라이트 렌더링
```

### Day 7: 주간 리뷰
- YouTube 데모 영상
- 블로그 포스트
- 다음 주 계획

---

## 📝 개발 원칙

### Pole 언어 핵심 가치 지키기

1. **명세 우선 (Spec-First)**
   - 항상 .pole 명세로 시작
   - LLM이 .pole-ir 구현 생성
   - 사람은 "무엇"을, LLM은 "어떻게"를

2. **타입 안전성**
   - 컴파일 타임 에러 감지
   - null 대신 Option<T>
   - Result<T,E>로 에러 처리

3. **네이티브 성능**
   - LLVM 컴파일
   - Zero-cost abstractions
   - 메모리 최적화

4. **LLM 친화적**
   - 자연어 명세
   - 명확한 제약조건
   - 예제 기반 학습

### 실전 개발 피드백 루프

1. **문제 발견**: 게임 개발 중 언어 한계 발견
2. **즉시 수정**: 1-2일 내 언어 기능 추가/개선
3. **검증**: 게임에 즉시 적용하여 검증
4. **정리**: Pole Engine으로 재사용 가능하게 추출
5. **문서화**: 다른 개발자도 사용 가능하도록

---

## 🎯 핵심 차별점

### 기존 방식 (실패 패턴)
```
1. 언어 완성 (5년)
2. 엔진 개발 (3년)
3. 게임 제작 (2년)
---
총 10년, 높은 실패 확률
```

### 새로운 방식 (Pole 방식)
```
1주차: 게임 명세 → LLM 생성 → 플레이 가능
2주차: 언어 문제 발견 → 즉시 수정
3주차: 엔진 패턴 추출 → 재사용
4주차: 더 복잡한 기능 추가
---
반복하며 동시에 성장
```

**장점:**
- ✅ 빠른 피드백 (1주 단위)
- ✅ 실전 검증된 기능
- ✅ 지루하지 않음 (매주 플레이 가능한 결과물)
- ✅ 언어-엔진-게임 모두 진화

---

## 📚 관련 문서

- [ARCHITECTURE.md](ARCHITECTURE.md) - Pole 시스템 아키텍처
- [QUICKSTART.md](QUICKSTART.md) - 빠른 시작 가이드
- [specs/](specs/) - 언어 명세
- [games/zomboid/](games/zomboid/) - PZ Clone 프로젝트
- [docs/IR_PARSER_MULTIARG_FIX.md](docs/IR_PARSER_MULTIARG_FIX.md) - 최근 버그 수정

---

## 🔄 변경 이력

- **2025-10-20**: 통합 개발 전략 수립 🎮
  - **핵심 변경**: 3-Track 병렬 개발 (언어-엔진-게임)
  - 기존 순차 개발 → 동시 개발로 전환
  - 주간 사이클 확립 (게임→언어→엔진)
  - 실전 피드백 루프 강조
  - 2년 내 Early Access 목표
- **2025-10-20**: IR Parser Multi-arg 버그 수정 ✅
  - Multi-arg 함수 호출 완전 지원
  - LLM 변환 파이프라인 복원
  - Pole 핵심 원칙 복원
- **2025-10-19**: Phase 5.1 & 6.1 완료
  - LLVM 백엔드, FFI 시스템, SDL2 통합
