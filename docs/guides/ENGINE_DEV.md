# Pole Engine 개발 가이드

> 재사용 가능한 2D 게임 엔진 모듈 개발

**대상:** Pole Engine 기여자  
**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [Engine 개요](#engine-개요)
2. [모듈 구조](#모듈-구조)
3. [개발 워크플로우](#개발-워크플로우)
4. [모듈별 가이드](#모듈별-가이드)
5. [테스트 및 문서화](#테스트-및-문서화)

---

## Engine 개요

### 목표

Pole Engine은 **검증된 게임 코드에서 추출한 재사용 가능한 모듈**입니다.

**설계 원칙:**
1. **Spec-First:** `.pole` 명세로 작성, LLM이 `.pole-ir` 생성
2. **실전 검증:** Pole Zomboid에서 실제 사용 후 추출
3. **모듈화:** 독립적으로 사용 가능한 작은 단위
4. **성능:** LLVM 네이티브 컴파일 (60+ FPS)

### Engine vs Game

| 구분 | Pole Engine | Pole Zomboid |
|------|-------------|--------------|
| 목적 | 재사용 가능한 라이브러리 | 구체적인 게임 |
| 작성 | 목-금 | 월-화 |
| 형태 | `.pole` 모듈 | `.pole` 게임 로직 |
| 추출 | 게임 → 엔진 | 엔진 사용 |

---

## 모듈 구조

```
pole_engine/
  ├── render/           # 렌더링
  │   ├── sprite.pole
  │   ├── tilemap.pole
  │   └── window.pole
  │
  ├── input/            # 입력 처리
  │   ├── keyboard.pole
  │   └── mouse.pole
  │
  ├── core/             # 기본 타입 및 유틸
  │   ├── types.pole    # Position, Direction 등
  │   ├── math.pole     # distance, clamp 등
  │   └── time.pole     # FPS, delta time
  │
  ├── physics/          # 물리 및 충돌
  │   ├── collision.pole
  │   └── movement.pole
  │
  ├── ai/               # 인공지능
  │   ├── pathfinding.pole
  │   └── behavior.pole
  │
  └── network/          # 멀티플레이어
      ├── tcp.pole
      └── sync.pole
```

---

## 개발 워크플로우

### 주간 사이클 (목-금)

#### 목요일: 게임 코드 분석 및 추출

1. **월-화 게임 개발 리뷰**
   ```bash
   # 작성된 게임 코드 확인
   ls games/zomboid/specs/*.pole
   ```

2. **재사용 가능한 패턴 식별**
   - 여러 곳에서 사용된 함수
   - 게임 로직과 독립적인 코드
   - 범용적으로 유용한 기능

3. **엔진 모듈로 추출**
   ```pole
   // games/zomboid/specs/player.pole (게임 코드)
   function distance(pos1: Position, pos2: Position) -> Int:
     purpose: "맨해튼 거리 계산"
     ...
   
   ↓ 추출
   
   // pole_engine/core/math.pole (엔진 코드)
   function manhattan_distance(pos1: Position, pos2: Position) -> Int:
     purpose: "두 점 사이의 맨해튼 거리"
     examples:
       - manhattan_distance((0,0), (3,4)) → 7
       - manhattan_distance((5,5), (5,5)) → 0
   ```

4. **명세 개선**
   - 더 명확한 함수명
   - 더 많은 예제
   - 제약조건 명시
   - 엣지 케이스 추가

#### 금요일: LLM 생성, 테스트, 문서화

1. **LLM으로 IR 생성**
   ```bash
   pole build pole_engine/core/math.pole
   ```

2. **유닛 테스트 작성**
   ```bash
   # 테스트 예제 작성
   examples/XX-test-math.pole-ir
   
   # 실행
   pole test examples/XX-test-math.pole-ir
   ```

3. **통합 테스트**
   - 게임에서 엔진 모듈 사용
   - 기존 게임 코드 대체
   - 동작 확인

4. **문서화**
   ```markdown
   # pole_engine/core/README.md
   
   ## math.pole
   
   수학 유틸리티 함수
   
   ### Functions
   
   #### manhattan_distance
   - **Input:** pos1, pos2 (Position)
   - **Output:** Int (거리)
   - **용도:** AI 경로 찾기, 충돌 감지
   ```

---

## 모듈별 가이드

### 1. render 모듈

**목적:** SDL2 기반 2D 렌더링

#### sprite.pole

```pole
type Sprite = {
  x: Int,
  y: Int,
  width: Int,
  height: Int,
  color: Color
}

type Color = { r: Int, g: Int, b: Int }

function draw_sprite(renderer: Ptr<Unit>, sprite: Sprite) -> Unit:
  purpose: "스프라이트를 화면에 그리기"
  constraints:
    - "0 <= r,g,b <= 255"
  examples:
    - draw_sprite(renderer, {x=10, y=20, w=32, h=32, color={r=255,g=0,b=0}})
```

#### tilemap.pole

```pole
type Tilemap = {
  width: Int,
  height: Int,
  tiles: List<Int>  // tile IDs
}

function draw_tilemap(renderer: Ptr<Unit>, tilemap: Tilemap, 
                     camera_x: Int, camera_y: Int) -> Unit:
  purpose: "타일맵을 카메라 기준으로 렌더링"
  constraints:
    - "화면에 보이는 타일만 렌더링 (최적화)"
  examples:
    - draw_tilemap(renderer, map_20x20, 0, 0)  // 카메라 (0,0)
```

### 2. input 모듈

**목적:** 키보드, 마우스 입력 처리

#### keyboard.pole

```pole
type KeyboardState = {
  w_pressed: Bool,
  a_pressed: Bool,
  s_pressed: Bool,
  d_pressed: Bool,
  escape_pressed: Bool
}

function poll_keyboard() -> KeyboardState:
  purpose: "현재 프레임의 키보드 상태 읽기"
  
function is_key_pressed(state: KeyboardState, key: String) -> Bool:
  purpose: "특정 키가 눌렸는지 확인"
  examples:
    - is_key_pressed(state, "w") → true
```

### 3. core 모듈

**목적:** 기본 타입 및 유틸리티

#### types.pole

```pole
type Position = { x: Int, y: Int }

type Direction = North | South | East | West

function position_add(pos: Position, dir: Direction) -> Position:
  purpose: "위치에 방향을 더해 새 위치 계산"
  examples:
    - position_add({x=10, y=10}, North) → {x=10, y=9}
    - position_add({x=5, y=5}, East) → {x=6, y=5}
```

#### math.pole

```pole
function manhattan_distance(pos1: Position, pos2: Position) -> Int:
  purpose: "맨해튼 거리 계산"
  examples:
    - manhattan_distance((0,0), (3,4)) → 7

function clamp(value: Int, min: Int, max: Int) -> Int:
  purpose: "값을 범위 내로 제한"
  examples:
    - clamp(150, 0, 100) → 100
    - clamp(-10, 0, 100) → 0
    - clamp(50, 0, 100) → 50
```

### 4. physics 모듈

**목적:** 충돌 감지 및 물리

#### collision.pole

```pole
type AABB = {
  x: Int,
  y: Int,
  width: Int,
  height: Int
}

function aabb_intersect(a: AABB, b: AABB) -> Bool:
  purpose: "두 직사각형이 겹치는지 확인"
  examples:
    - aabb_intersect({x=0,y=0,w=10,h=10}, {x=5,y=5,w=10,h=10}) → true
    - aabb_intersect({x=0,y=0,w=10,h=10}, {x=20,y=20,w=10,h=10}) → false
```

### 5. ai 모듈

**목적:** 게임 AI

#### pathfinding.pole

```pole
function find_path(start: Position, goal: Position, tilemap: Tilemap) -> List<Position>:
  purpose: "A* 알고리즘으로 경로 찾기"
  constraints:
    - "벽 타일 회피"
    - "최대 경로 길이 100"
  examples:
    - find_path((0,0), (5,5), map) → [(0,0), (1,0), ..., (5,5)]
```

---

## 테스트 및 문서화

### 유닛 테스트

모든 엔진 모듈은 테스트 예제가 있어야 합니다.

```pole
// examples/XX-test-math.pole-ir

def test_manhattan_distance() -> Int =
  let d1 = manhattan_distance({x=0, y=0}, {x=3, y=4}) in
  let d2 = manhattan_distance({x=5, y=5}, {x=5, y=5}) in
  if d1 == 7 && d2 == 0 then
    0  // success
  else
    1  // failure
```

### 문서화 체크리스트

각 모듈은 다음을 포함해야 합니다:

- [ ] 모듈 README.md
  - 목적 및 용도
  - 함수 목록
  - 예제 코드
  
- [ ] 함수별 명세
  - purpose (목적)
  - input/output 타입
  - constraints (제약조건)
  - examples (예제 2개 이상)

- [ ] 테스트 예제
  - 정상 케이스
  - 엣지 케이스
  - 에러 케이스

### 성능 기준

엔진 모듈은 다음 성능을 만족해야 합니다:

- **렌더링:** 60 FPS (16.6ms/frame)
- **물리:** 100+ 엔티티 처리
- **AI:** 10개 경로 찾기 동시 실행
- **네트워크:** 4인 동기화 < 50ms latency

---

## 예제: 모듈 추출 과정

### 1. 게임 코드 (Before)

```pole
// games/zomboid/specs/zombie.pole

function distance(pos1: Position, pos2: Position) -> Int:
  purpose: "거리 계산"
  let dx = abs(pos1.x - pos2.x) in
  let dy = abs(pos1.y - pos2.y) in
  dx + dy
```

### 2. 엔진 모듈 (After)

```pole
// pole_engine/core/math.pole

function manhattan_distance(pos1: Position, pos2: Position) -> Int:
  purpose: "두 점 사이의 맨해튼 거리 계산"
  
  input:
    - pos1: 첫 번째 위치
    - pos2: 두 번째 위치
  
  output: 맨해튼 거리 (|dx| + |dy|)
  
  constraints:
    - "항상 양수 반환"
    - "대칭: distance(a,b) == distance(b,a)"
  
  examples:
    - manhattan_distance({x=0, y=0}, {x=3, y=4}) → 7
    - manhattan_distance({x=5, y=5}, {x=5, y=5}) → 0
    - manhattan_distance({x=10, y=10}, {x=5, y=8}) → 7
    
  performance: O(1)
```

### 3. 게임에서 사용

```pole
// games/zomboid/specs/zombie.pole

import pole_engine.core.math (manhattan_distance)

function update_zombie(zombie: Zombie, player: Player) -> Zombie:
  let dist = manhattan_distance(zombie.position, player.position) in
  if dist < 10 then
    // Chase player
  else
    // Idle
```

---

## 관련 문서

- [Engine README](../../pole_engine/README.md) - 엔진 구조 개요
- [게임 개발 가이드](GAME_DEV.md) - 엔진 사용 예제
- [언어 개발 가이드](LANGUAGE_DEV.md) - Pole 언어 기능
- [로드맵](../../ROADMAP.md) - 엔진 개발 일정

---

**목표:** 2년 내 완전한 2D 게임 엔진 (15개 모듈)  
**현재:** 초기 단계 (72개 예제 코드 보유)
