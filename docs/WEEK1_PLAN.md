# Week 1 통합 개발 계획 (2025-10-20 ~ 2025-10-26)

> **목표:** Pole 언어-엔진-게임 동시 개발의 첫 주
> 
> 1분 플레이 가능한 Pole Zomboid 데모 완성

---

## 📋 주간 목표

### 최종 결과물
- [ ] **플레이 가능한 데모**: 플레이어가 WASD로 이동, 좀비 1마리가 추적
- [ ] **5개 명세 파일**: player, zombie, combat, inventory, survival
- [ ] **Pole Engine 첫 모듈**: `pole_render` (스프라이트 렌더링)
- [ ] **YouTube 데모 영상**: 1분 게임플레이

---

## 🗓️ 일별 계획

### Day 1 (월): Player 명세 & 구현

**오전: 명세 작성**
```pole
// games/zomboid/specs/player.pole

type Position = { x: Int, y: Int }

type Player = {
  position: Position,
  health: Int,        // 0-100
  hunger: Int,        // 0-100
  facing: Direction   // North, South, East, West
}

type Direction = North | South | East | West

function create_player(x: Int, y: Int) -> Player:
  purpose: "새 플레이어 생성"
  input: "초기 위치 (x, y)"
  output: "기본 상태의 플레이어"
  constraints:
    - "체력 100"
    - "배고픔 100"
  examples:
    - create_player(10, 10) → Player{pos=(10,10), hp=100, hunger=100}

function move_player(player: Player, direction: Direction, tilemap: Tilemap) -> Player:
  purpose: "플레이어를 한 타일 이동"
  input: "플레이어, 이동 방향, 타일맵"
  output: "이동된 플레이어 (또는 원래 위치)"
  constraints:
    - "맵 경계 체크 (0 <= x < map_width)"
    - "벽 타일은 이동 불가"
  examples:
    - move_player(player_at_10_10, North, map) → player_at_10_9
    - move_player(player_at_0_0, West, map) → player_at_0_0 (경계)

function update_player(player: Player, dt: Float64) -> Player:
  purpose: "프레임마다 플레이어 상태 업데이트"
  input: "플레이어, 델타타임 (초)"
  output: "업데이트된 플레이어"
  constraints:
    - "배고픔: 1시간마다 10 감소"
    - "배고픔 0이면 체력 감소 시작"
  examples:
    - update_player(player_100hp_50hunger, 0.016) → (약간 감소)
```

**오후: LLM 생성 & 테스트**
```bash
# LLM으로 구현 생성
pole build games/zomboid/specs/player.pole

# 타입 체크
pole test games/zomboid/specs/player.pole-ir

# 수동 테스트 작성
examples/67-test-player.pole-ir
```

**저녁: 언어 이슈 발견 시 수정**
- 발견된 문제를 ROADMAP.md에 기록
- 필요시 즉시 수정

---

### Day 2 (화): Zombie 명세 & 구현

**오전: 명세 작성**
```pole
// games/zomboid/specs/zombie.pole

type Zombie = {
  position: Position,
  health: Int,           // 0-100
  state: ZombieState,
  target_player: Int     // Player ID, -1 if no target
}

type ZombieState = Idle | Chase | Attack

function create_zombie(x: Int, y: Int) -> Zombie:
  purpose: "새 좀비 생성"
  examples:
    - create_zombie(5, 5) → Zombie{pos=(5,5), hp=50, state=Idle}

function distance(pos1: Position, pos2: Position) -> Int:
  purpose: "두 위치 사이의 맨해튼 거리"
  examples:
    - distance((0,0), (3,4)) → 7

function update_zombie(zombie: Zombie, player: Player, dt: Float64) -> Zombie:
  purpose: "좀비 AI 업데이트"
  constraints:
    - "플레이어가 10타일 이내: Chase 상태로 전환"
    - "플레이어가 1타일 이내: Attack 상태로 전환"
    - "Chase 상태: 플레이어 방향으로 이동"
  examples:
    - update_zombie(zombie_at_10_10, player_at_15_15, 0.016) → (Idle 유지)
    - update_zombie(zombie_at_10_10, player_at_12_10, 0.016) → (Chase, 한 칸 이동)

function move_towards(zombie: Zombie, target: Position) -> Zombie:
  purpose: "목표 위치로 한 칸 이동"
  constraints:
    - "X축 우선 이동"
    - "X축 같으면 Y축 이동"
```

**오후: LLM 생성 & 통합**
```bash
pole build games/zomboid/specs/zombie.pole

# Player + Zombie 통합 테스트
examples/68-player-zombie.pole-ir
```

---

### Day 3 (수): 언어 개선 & 렌더링

**오전: 언어 이슈 해결**
- Day 1-2에서 발견한 문제 수정
- LLM 프롬프트 개선
- 타입 시스템 개선

**오후: 렌더링 시스템**
```pole
// pole_engine/render/sprite.pole

type Sprite = {
  x: Int,
  y: Int,
  width: Int,
  height: Int,
  color: Color  // RGB
}

function draw_sprite(renderer: Ptr<Unit>, sprite: Sprite) -> Unit:
  purpose: "스프라이트를 화면에 렌더링"
  
function draw_tilemap(renderer: Ptr<Unit>, tilemap: Tilemap, camera_x: Int, camera_y: Int) -> Unit:
  purpose: "타일맵 렌더링 (카메라 기준)"
```

---

### Day 4 (목): 통합 데모 제작

**오전: 메인 게임 루프**
```pole
// games/zomboid/main.pole

function game_loop(window: Ptr<Unit>, renderer: Ptr<Unit>) -> Int:
  purpose: "메인 게임 루프"
  
  let player = create_player(10, 10) in
  let zombie = create_zombie(5, 5) in
  
  loop:
    // 입력 처리
    let input = poll_keyboard() in
    let player2 = handle_input(player, input) in
    
    // 업데이트
    let zombie2 = update_zombie(zombie, player2, 0.016) in
    
    // 렌더링
    let _ = SDL_RenderClear(renderer) in
    let _ = draw_tilemap(renderer, map, player2.position.x, player2.position.y) in
    let _ = draw_player(renderer, player2) in
    let _ = draw_zombie(renderer, zombie2) in
    let _ = SDL_RenderPresent(renderer) in
    
    if should_quit(input) then 0 else game_loop(...)
```

**오후: 컴파일 & 디버깅**
```bash
# 컴파일
pole build games/zomboid/main.pole

# 네이티브 실행
./build/pole_zomboid

# 디버깅 (필요시 Python 인터프리터)
pole run games/zomboid/main.pole-ir main
```

---

### Day 5 (금): Pole Engine 리팩토링

**오전: 재사용 코드 추출**
```bash
pole_engine/
  ├── render/
  │   ├── sprite.pole       # Day 3에서 작성
  │   ├── tilemap.pole      # 타일맵 렌더링
  │   └── window.pole       # SDL2 윈도우 래퍼
  │
  ├── input/
  │   └── keyboard.pole     # 키보드 입력
  │
  └── core/
      ├── math.pole         # distance, clamp 등
      └── types.pole        # Position, Direction 등
```

**오후: 문서화**
```markdown
# Pole Engine 0.1

## render 모듈

### sprite.pole
타일 기반 2D 스프라이트 렌더링

Functions:
- draw_sprite(renderer, sprite) → Unit
- draw_tilemap(renderer, tilemap, camera_x, camera_y) → Unit

## input 모듈

### keyboard.pole
키보드 입력 처리 (SDL2)

Functions:
- poll_keyboard() → KeyboardState
- is_key_pressed(state, key) → Bool
```

---

### Day 6 (토): 테스트 & 최적화

**오전: 전체 테스트**
```bash
# 유닛 테스트
pole test games/zomboid/specs/player.pole-ir
pole test games/zomboid/specs/zombie.pole-ir

# 통합 테스트
./build/pole_zomboid

# 성능 테스트
# 목표: 60 FPS, 1 플레이어 + 1 좀비
```

**오후: 최적화**
- 프로파일링
- 병목 지점 발견
- 필요시 코드 수정

---

### Day 7 (일): 데모 & 리뷰

**오전: YouTube 데모 영상**
1. 게임 플레이 녹화 (1분)
2. 편집 (자막, 음악)
3. 업로드

**내용:**
- 타이틀: "Pole 언어로 만든 좀비 게임 (1주차)"
- 보여줄 것:
  - WASD 플레이어 이동
  - 좀비가 플레이어 추적
  - 타일맵 렌더링
  - 코드 일부 (명세 예시)

**오후: 주간 리뷰 & 계획**
```markdown
# Week 1 완료 보고

## 달성한 것
- [ ] 5개 명세 파일 작성
- [ ] LLM 생성 코드 X개 함수
- [ ] 플레이 가능한 데모
- [ ] Pole Engine 3개 모듈

## 발견한 언어 이슈
1. ...
2. ...

## 다음 주 계획
- 전투 시스템 구현
- 인벤토리 통합
- 좀비 10마리로 확장
```

---

## ✅ 주간 체크리스트

### Track 3: Pole Zomboid
- [ ] player.pole 명세 작성 & LLM 생성
- [ ] zombie.pole 명세 작성 & LLM 생성
- [ ] main.pole 메인 루프 작성
- [ ] 플레이 가능한 실행 파일

### Track 2: Pole Engine
- [ ] render/sprite.pole
- [ ] render/tilemap.pole
- [ ] input/keyboard.pole
- [ ] core/math.pole
- [ ] core/types.pole

### Track 1: Pole 언어
- [ ] 발견된 이슈 X개 수정
- [ ] LLM 프롬프트 개선
- [ ] 문서 업데이트

### 홍보 & 커뮤니티
- [ ] YouTube 데모 영상
- [ ] 블로그 포스트
- [ ] Reddit/Discord 공유

---

## 🎯 성공 기준

### 필수 (P0)
- ✅ 플레이어 WASD 이동 작동
- ✅ 좀비 1마리 추적 작동
- ✅ 타일맵 렌더링
- ✅ 60 FPS 유지

### 선택 (P1)
- ⭐ 충돌 감지 (플레이어-벽)
- ⭐ 좀비 애니메이션
- ⭐ 카메라 부드러운 이동

### 보너스 (P2)
- 🎁 멀티 좀비 (3-5마리)
- 🎁 사운드 효과
- 🎁 미니맵

---

## 🔧 개발 환경 설정

### 필요한 도구
```bash
# Pole CLI
which pole  # /usr/local/bin/pole

# Rust 컴파일러 (LLVM 백엔드)
rustc --version

# SDL2
sdl2-config --version

# LLM API (OpenRouter)
echo $OPENROUTER_API_KEY  # 설정 확인
```

### 디렉토리 구조
```
pole/
  ├── games/zomboid/
  │   ├── specs/          # .pole 명세
  │   ├── build/          # 컴파일된 바이너리
  │   └── assets/         # 스프라이트, 타일셋
  │
  ├── pole_engine/        # 재사용 가능한 엔진 코드
  │   ├── render/
  │   ├── input/
  │   └── core/
  │
  └── examples/           # 테스트 예제
```

---

## 💡 팁

### LLM 사용 시
1. **구체적인 예제**: 최소 2-3개 예제 제공
2. **제약조건 명확히**: "맵 경계 체크" 같은 구체적 조건
3. **타입 명시**: 모든 파라미터와 반환값 타입 지정
4. **반복 생성**: 첫 생성이 이상하면 다시 생성

### 디버깅
1. **인터프리터 우선**: `pole run` 먼저 테스트
2. **타입 체크**: `pole check` 항상 실행
3. **단위 테스트**: 함수별로 작은 테스트 작성
4. **printf 디버깅**: `print()` 함수 적극 활용

### 성능
1. **조기 최적화 금지**: 일단 작동하게 만들기
2. **프로파일링 먼저**: 병목 찾고 수정
3. **LLVM 최적화**: `-O2` 플래그 활용

---

## 📞 문제 발생 시

### 언어 버그 발견
1. `docs/ISSUES.md`에 기록
2. 재현 가능한 최소 예제 작성
3. Day 3 (수요일)에 수정

### LLM 생성 실패
1. 명세를 더 구체적으로 수정
2. 예제 추가
3. System prompt 개선

### 성능 문제
1. 프로파일러 실행
2. 병목 지점 식별
3. 알고리즘 개선 또는 최적화

---

이번 주 화이팅! 🚀
