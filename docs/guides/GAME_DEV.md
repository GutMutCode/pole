# Pole Zomboid 게임 개발 가이드

> Project Zomboid 클론 게임 개발 with Pole 언어

**대상:** 게임 컨텐츠 개발자  
**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [게임 개요](#게임-개요)
2. [개발 워크플로우](#개발-워크플로우)
3. [명세 작성 가이드](#명세-작성-가이드)
4. [시스템별 가이드](#시스템별-가이드)
5. [LLM 활용 팁](#llm-활용-팁)

---

## 게임 개요

### Pole Zomboid란?

**Project Zomboid** 스타일의 좀비 생존 게임을 **Pole 언어**로 개발하는 프로젝트입니다.

**핵심 특징:**
- **명세 우선:** 자연어로 기능 설명 → LLM이 코드 생성
- **점진적 개발:** 매주 작은 기능 추가
- **실시간 피드백:** 매주 플레이 가능한 빌드

### 개발 목표

| 기간 | 목표 |
|------|------|
| Week 1 | 1분 데모 (플레이어 이동 + 좀비 1마리) |
| 3개월 | 10분 플레이 (전투, 인벤토리, 건물) |
| 6개월 | 1시간 플레이 (멀티플레이어, 퀘스트) |
| 1년 | 플레이어블 데모, 크라우드펀딩 |
| 2년 | Steam Early Access |

---

## 개발 워크플로우

### 주간 사이클 (월-화)

#### 월요일: 새 기능 명세 작성

1. **기능 선택**
   - Week Plan에서 우선순위 확인
   - 작고 완성 가능한 기능 선택

2. **명세 파일 작성**
   ```pole
   // games/zomboid/specs/player.pole
   
   type Player = { ... }
   
   function create_player(...) -> Player:
     purpose: "새 플레이어 생성"
     examples:
       - create_player(10, 10) → Player{...}
   ```

3. **LLM으로 IR 생성**
   ```bash
   pole build games/zomboid/specs/player.pole
   ```

4. **컴파일 & 테스트**
   ```bash
   pole test games/zomboid/specs/player.pole-ir
   ```

#### 화요일: 통합 & 플레이 테스트

1. **메인 루프 통합**
   ```pole
   // games/zomboid/main.pole
   
   function game_loop() -> Int:
     // 새 기능 사용
   ```

2. **네이티브 컴파일**
   ```bash
   pole compile games/zomboid/main.pole-ir -o build/zomboid
   ```

3. **플레이 테스트**
   - 실제로 게임 실행
   - 버그 발견 시 기록
   - 성능 측정

4. **언어 이슈 기록**
   - 발견된 문제를 수요일에 수정
   - `docs/ISSUES.md`에 기록

---

## 명세 작성 가이드

### 좋은 명세의 조건

1. **명확한 목적 (purpose)**
2. **구체적인 예제 (examples)**
3. **제약조건 (constraints)**
4. **타입 명시 (types)**

### 예제: Player 명세

```pole
type Position = { x: Int, y: Int }

type Player = {
  position: Position,
  health: Int,        // 0-100
  hunger: Int,        // 0-100
  inventory: List<Item>,
  facing: Direction
}

type Direction = North | South | East | West

function create_player(x: Int, y: Int) -> Player:
  purpose: "초기 위치에 새 플레이어 생성"
  
  input:
    - x: 초기 X 좌표
    - y: 초기 Y 좌표
  
  output: 기본 상태의 플레이어
    - 체력 100
    - 배고픔 100
    - 빈 인벤토리
    - 남쪽 바라봄
  
  examples:
    - create_player(10, 10) → Player{pos=(10,10), hp=100, hunger=100, inv=[], facing=South}
    - create_player(0, 0) → Player{pos=(0,0), hp=100, hunger=100, inv=[], facing=South}

function move_player(player: Player, direction: Direction, tilemap: Tilemap) -> Player:
  purpose: "플레이어를 한 타일 이동시킴 (충돌 체크)"
  
  input:
    - player: 이동할 플레이어
    - direction: 이동 방향
    - tilemap: 충돌 체크용 맵
  
  output: 이동된 플레이어 (이동 불가 시 원래 위치)
  
  constraints:
    - "맵 경계 체크: 0 <= x < map_width, 0 <= y < map_height"
    - "벽 타일(tile_id=1)은 이동 불가"
    - "facing 방향도 업데이트"
  
  examples:
    - move_player(player_at_10_10, North, map) → player_at_10_9 (이동 성공)
    - move_player(player_at_0_0, West, map) → player_at_0_0 (경계 밖)
    - move_player(player_at_5_5, South, map_with_wall) → player_at_5_5 (벽 충돌)
```

### 명세 작성 체크리스트

- [ ] 타입 정의 (모든 필드 명시)
- [ ] purpose 작성 (1-2줄 설명)
- [ ] input/output 설명
- [ ] constraints (제약조건) 명시
- [ ] examples 3개 이상
  - [ ] 정상 케이스
  - [ ] 엣지 케이스
  - [ ] 에러 케이스

---

## 시스템별 가이드

### 1. Player 시스템

**파일:** `games/zomboid/specs/player.pole`

**필수 기능:**
- 생성 (create_player)
- 이동 (move_player)
- 상태 업데이트 (update_player)
  - 배고픔 감소
  - 체력 회복/감소

**예제:**
```pole
function update_player(player: Player, dt: Float64) -> Player:
  purpose: "프레임마다 플레이어 상태 업데이트"
  
  constraints:
    - "배고픔: 실시간 1시간 = 게임 1일 → 10 감소"
    - "배고픔 0이면 체력 1/초 감소"
    - "체력 0이면 사망 상태"
  
  examples:
    - update_player(player_100hp_50hunger, 0.016) → (배고픔 약간 감소)
```

### 2. Zombie 시스템

**파일:** `games/zomboid/specs/zombie.pole`

**필수 기능:**
- 생성 (create_zombie)
- AI 업데이트 (update_zombie)
  - Idle: 정지
  - Chase: 플레이어 추적
  - Attack: 공격
- 이동 (move_towards)

**예제:**
```pole
function update_zombie(zombie: Zombie, player: Player, dt: Float64) -> Zombie:
  purpose: "좀비 AI 업데이트 (상태 전환 + 이동)"
  
  constraints:
    - "플레이어 10타일 이내 → Chase 상태"
    - "플레이어 1타일 이내 → Attack 상태"
    - "Chase 상태: 플레이어 방향으로 1타일 이동 (1초당)"
  
  examples:
    - update_zombie(zombie_at_10_10, player_at_20_20, 0.016) → (Idle, 이동 없음)
    - update_zombie(zombie_at_10_10, player_at_12_10, 0.016) → (Chase, x+1 이동)
    - update_zombie(zombie_at_10_10, player_at_10_11, 0.016) → (Attack)
```

### 3. Combat 시스템

**파일:** `games/zomboid/specs/combat.pole`

**필수 기능:**
- 공격 (attack)
- 데미지 계산 (calculate_damage)
- 사망 처리 (handle_death)

**예제:**
```pole
type Weapon = {
  name: String,
  damage: Int,      // 10-100
  range: Int,       // 1-5 tiles
  cooldown: Float64 // 초
}

function attack(attacker: Player, target: Zombie, weapon: Weapon) -> (Player, Zombie):
  purpose: "플레이어가 좀비를 무기로 공격"
  
  constraints:
    - "거리 체크: manhattan_distance <= weapon.range"
    - "쿨다운 체크: last_attack + cooldown < current_time"
    - "데미지: weapon.damage ± 랜덤(20%)"
  
  examples:
    - attack(player, zombie_100hp, {name="Bat", damage=20, range=1, cd=1.0})
      → (player_with_cooldown, zombie_80hp)
```

### 4. Inventory 시스템

**파일:** `games/zomboid/specs/inventory.pole`

**필수 기능:**
- 아이템 추가 (add_item)
- 아이템 사용 (use_item)
- 아이템 제거 (remove_item)

**예제:**
```pole
type Item = Food | Weapon | Medicine

type Food = { name: String, hunger_restore: Int }
type Medicine = { name: String, health_restore: Int }

function use_item(player: Player, item_index: Int) -> Player:
  purpose: "인벤토리 아이템 사용"
  
  constraints:
    - "0 <= item_index < inventory.length"
    - "Food: hunger 회복 (최대 100)"
    - "Medicine: health 회복 (최대 100)"
    - "사용 후 아이템 제거"
  
  examples:
    - use_item(player_50hunger, 0) → player_70hunger (Food +20 사용)
```

### 5. Survival 시스템

**파일:** `games/zomboid/specs/survival.pole`

**필수 기능:**
- 시간 경과 (update_time)
- 생존 상태 체크 (check_survival_status)
- 환경 효과 (apply_environment_effects)

---

## LLM 활용 팁

### 1. 명세는 구체적으로

**나쁜 예:**
```pole
function move_player(player: Player) -> Player:
  purpose: "플레이어 이동"
```

**좋은 예:**
```pole
function move_player(player: Player, direction: Direction, tilemap: Tilemap) -> Player:
  purpose: "플레이어를 한 타일 이동 (충돌 체크 포함)"
  
  constraints:
    - "맵 경계: 0 <= x < map_width"
    - "벽 타일 충돌 체크"
  
  examples:
    - move_player(player_at_10_10, North, map) → player_at_10_9
    - move_player(player_at_0_0, West, map) → player_at_0_0 (경계)
```

### 2. 예제는 3개 이상

- 정상 케이스 1개
- 엣지 케이스 1개
- 에러 케이스 1개

### 3. 타입을 명확히

```pole
// 나쁨
function foo(x, y) -> z

// 좋음
function manhattan_distance(pos1: Position, pos2: Position) -> Int
```

### 4. LLM 생성이 이상하면 재생성

```bash
# 첫 시도
pole build player.pole

# 결과 확인
pole test player.pole-ir

# 이상하면 명세 수정 후 재생성
vim player.pole  # 예제 추가
pole build player.pole --force
```

### 5. System Prompt 활용

LLM에게 더 나은 코드를 생성하도록 유도:

```markdown
당신은 Pole IR 코드 생성 전문가입니다.

규칙:
1. 모든 if 분기는 else 포함
2. 변수명은 명확하게 (x1 대신 player_new_x)
3. 함수 호출은 tuple로 (f(a, b) → f((a, b)))
4. 타입 추론 가능하게 작성
```

---

## 디버깅 가이드

### 컴파일 에러

```bash
# 타입 체크
pole check player.pole-ir

# 상세 에러 메시지
pole check player.pole-ir --verbose
```

### 런타임 에러

```bash
# Python 인터프리터로 테스트 (느리지만 디버깅 쉬움)
pole run player.pole-ir create_player 10 10

# printf 디버깅
# player.pole-ir에 print 추가
let _ = print("Player position: ", player.position.x) in
...
```

### 성능 문제

```bash
# 프로파일링
pole profile zomboid.pole-ir

# LLVM 최적화 레벨 조정
pole compile zomboid.pole-ir -O2
pole compile zomboid.pole-ir -O3
```

---

## 관련 문서

- [Week 1 계획](../WEEK1_PLAN.md) - 첫 주 상세 일정
- [게임 시스템 명세](../../games/zomboid/specs/) - 전체 시스템
- [엔진 사용법](ENGINE_DEV.md) - Pole Engine 모듈 활용
- [LLM 가이드](LLM_USAGE.md) - LLM 효과적 활용법

---

**목표:** 매주 플레이 가능한 빌드, 2년 내 Steam Early Access 🎮
