# Project Zomboid Clone - Pole Engine 개발 로드맵

> **프로젝트명**: "Pole Survival" (가칭)
>
> **원작**: Project Zomboid (The Indie Stone, 2013)
>
> **개발 도구**: Pole Engine + Pole 언어

**작성일**: 2025-10-19  
**문서 목적**: Project Zomboid 스타일 좀비 생존 게임을 Pole Engine으로 개발하는 실전 로드맵

---

## 📋 목차

1. [프로젝트 개요](#프로젝트-개요)
2. [Project Zomboid 분석](#project-zomboid-분석)
3. [개발 계획](#개발-계획)
4. [개발 일정 (24개월)](#개발-일정-24개월)
5. [예산 및 수익 목표](#예산-및-수익-목표)
6. [기술 구현 상세](#기술-구현-상세)
7. [리스크 관리](#리스크-관리)

---

## 프로젝트 개요

### 🎯 게임 컨셉

**장르**: 아이소메트릭 좀비 생존 RPG 샌드박스  
**타겟 플랫폼**: PC (Steam), Mac, Linux  
**개발 기간**: 24개월 (2년)  
**팀 규모**: 3-7명

### 핵심 목표

1. **Project Zomboid의 핵심 메커니즘 재현**

   - 아이소메트릭 뷰 + 타일 기반 월드
   - 현실적인 생존 시뮬레이션
   - 느린 좀비 호드 메커니즘
   - 장기 생존 중심 게임플레이

2. **Pole 언어/엔진의 장점 활용**

   - LLM 기반 AI 행동 자동 생성
   - 자연어 명세로 복잡한 생존 로직 작성
   - 핫 리로딩으로 빠른 밸런스 조정
   - 계약 프로그래밍으로 버그 감소

3. **차별화 포인트**
   - 더 나은 멀티플레이어 (동기화 안정성)
   - 모듈식 콘텐츠 (Pole 모딩 지원)
   - 향상된 UI/UX
   - 크로스 플랫폼 지원

---

## Project Zomboid 분석

### 게임 핵심 시스템

#### 1. 생존 시스템

- **체력 관리**: 건강, 부상, 감염, 질병
- **욕구 시스템**: 배고픔, 갈증, 피로, 우울증, 권태감
- **체온 조절**: 날씨, 계절, 의복
- **위생**: 혈액, 오염, 청결

#### 2. 좀비 시스템

- **시각/청각 시스템**: 빛, 소음에 반응
- **군집 메커니즘**: 좀비 무리 형성 및 이동
- **감염 전파**: 긁힘, 물림, 공기 전염 (플레이어는 면역)
- **다양한 좀비 타입**: 느린 좀비, 달리는 좀비 (설정)

#### 3. 건설 & 제작 시스템

- **바리케이드**: 창문, 문 강화
- **건축**: 벽, 계단, 가구
- **제작**: 무기, 도구, 음식, 의약품
- **농사**: 작물 재배, 수확
- **낚시 & 포획**: 덫, 낚시대

#### 4. 기술 & 성장 시스템

- **스킬 트리**: 18개 스킬 (목공, 요리, 농사, 의료 등)
- **경험치**: 행동으로 스킬 성장
- **특성 시스템**: 장점/단점 선택
- **직업**: 시작 스킬 보너스

#### 5. 월드 시스템

- **거대한 오픈월드**: Muldraugh, West Point, Louisville 등
- **시간 경과**: 전기/수도 차단, 시체 부패, 식물 자라기
- **날씨**: 비, 눈, 온도 변화
- **계절**: 봄, 여름, 가을, 겨울

#### 6. 멀티플레이어

- **협동/PvP**: 서버 설정에 따라
- **세이프하우스**: 팀 거점
- **보이스 채팅**: 근거리 음성
- **지속 서버**: 24/7 운영

### 기술 스택 (원작)

- **엔진**: Java (LWJGL)
- **렌더링**: 2D 타일 + 아이소메트릭
- **네트워킹**: 클라이언트-서버
- **모딩**: Lua 스크립팅
- **맵 에디터**: 커스텀 에디터

### 성공 요인 분석

**강점:**

- 깊이 있는 생존 시뮬레이션
- 긴 플레이 타임 (수백~수천 시간)
- 강력한 모딩 커뮤니티
- 지속적인 업데이트 (11년+)
- 멀티플레이어 재미

**약점 (개선 기회):**

- 오래된 UI (Java Swing 느낌)
- 느린 업데이트 (NPC 시스템 미완성)
- 최적화 이슈 (Java GC)
- 컨트롤러 지원 미흡
- 온보딩 어려움 (가파른 학습 곡선)

---

## 개발 계획

### 👥 팀 구성

#### 최소 팀 (3명) - MVP 단계

- **리드 프로그래머 1명**

  - 역할: 시스템 아키텍처, 코어 메커니즘
  - Pole 스킬: 고급
  - 경험: 타일 기반 게임, 시뮬레이션

- **그래픽/UI 아티스트 1명**

  - 역할: 타일셋, 스프라이트, UI
  - 도구: Aseprite, Tiled, Figma
  - 스타일: 픽셀아트 아이소메트릭

- **게임 디자이너 1명** (겸업 가능)
  - 역할: 밸런스, 콘텐츠 디자인
  - Pole 스킬: 기본 (명세 작성 가능)

#### 확장 팀 (7명) - 정식 출시

- 위 3명 +
- **네트워크 프로그래머 1명**: 멀티플레이어
- **사운드 디자이너 1명**: 음악, 효과음
- **QA 테스터 1명**: 밸런스, 버그
- **커뮤니티 매니저 1명**: Discord, 피드백

### 🛠️ Pole Engine 기능 요구사항

#### 필수 기능

```
pole_graphics (2D):
  ✅ Isometric rendering
  ✅ Tile-based world (chunk loading)
  ✅ Sprite batching (수천 타일)
  ✅ Layering system (floor, objects, roof)
  ✅ Lighting system (동적 조명, 그림자)
  ✅ Fog of War (시야 시스템)
  ✅ 2D camera (zoom, pan, rotate)

pole_physics (2D):
  ✅ Grid-based collision
  ✅ Pathfinding (A*, navmesh)
  ✅ Zombie crowd simulation

pole_ecs:
  ✅ 대규모 entity 관리 (수천 좀비)
  ✅ Component-based architecture
  ✅ Spatial partitioning (quad tree)

pole_ui:
  ✅ 복잡한 인벤토리 시스템
  ✅ 드래그 앤 드롭
  ✅ 컨텍스트 메뉴
  ✅ 스킬 UI, 캐릭터 시트
  ✅ 건설 모드 UI

pole_audio:
  ✅ 3D 공간 오디오 (좀비 소리)
  ✅ Ambient sounds
  ✅ 음악 시스템

pole_world:
  ✅ Chunk streaming (거대한 월드)
  ✅ Save/Load system (세이브 파일)
  ✅ 시간 시스템 (day/night cycle)
  ✅ 날씨 시스템

pole_net:
  ✅ Client-server multiplayer
  ✅ State synchronization
  ✅ 대규모 오브젝트 동기화
```

#### 선택 기능

```
pole_mod:
  ⚠️ Lua 모딩 API (Pole 스크립트로 대체 가능)
  ⚠️ 맵 에디터

pole_analytics:
  ⚠️ 플레이어 행동 분석
  ⚠️ 밸런스 데이터 수집
```

#### 엔진 성숙도

- **최소**: Pole Engine 베타 (2D 타일 렌더링 안정화)
- **권장**: Pole Engine 1.0

### 💰 수익 전략

#### 가격 모델

- **Early Access**: $15 (₩20,000)
- **정식 출시**: $20 (₩27,000)
- **멀티팩**: 4팩 $60 (1팩당 $15)

#### 출시 플랫폼

1. **Steam** (주력)
2. **GOG** (DRM-free)
3. **itch.io** (인디 커뮤니티)

#### DLC 계획 (정식 출시 후)

- **맵 확장팩**: 새로운 도시 ($5-10)
- **시나리오 팩**: 챌린지 모드 ($5)
- **코스메틱**: 캐릭터 스킨 (무료/유료)

---

## 개발 일정 (24개월)

### Phase 1: MVP (Month 1-6)

**목표**: 핵심 생존 루프 + 좀비 AI

#### Month 1-2: 기술 기반

**작업:**

- [ ] 아이소메트릭 타일 렌더링 시스템
- [ ] 플레이어 이동 (WASD, 마우스 클릭)
- [ ] 카메라 시스템 (줌, 팬)
- [ ] 기본 타일맵 (1개 소규모 타운)

**Pole 예시 코드:**

```pole
type IsometricRenderer:
  purpose: Render tile-based isometric world

  fields:
    - tile_size: Vector2 = (64, 32)  // 아이소메트릭 타일 크기
    - viewport: Rectangle
    - camera_position: Vector2
    - zoom: float = 1.0

  methods:
    - render_tile(position: Vector2, sprite: Sprite, layer: int)
    - world_to_screen(world_pos: Vector2) -> Vector2
    - screen_to_world(screen_pos: Vector2) -> Vector2

  implementation:
    - Render tiles in correct depth order
    - Support multiple layers (floor, walls, roof)
    - Cull offscreen tiles
```

**완료 기준:**

- 플레이어가 타일맵에서 이동 가능
- 건물 내부/외부 구분
- 30 FPS 이상

#### Month 3-4: 생존 시스템 프로토타입

**작업:**

- [ ] 좀비 AI (시각/청각 시스템)
- [ ] 기본 전투 (근접 무기)
- [ ] 체력/배고픔/갈증 시스템
- [ ] 아이템 시스템 (습득, 드롭)
- [ ] 인벤토리 UI

**Pole 예시 코드:**

```pole
type ZombieSenses:
  purpose: Zombie sight and hearing system

  fields:
    - sight_range: float = 20.0  // 타일 단위
    - hearing_range: float = 30.0
    - noise_threshold: float = 5.0

  methods:
    @hot_reload  // 밸런스 조정 시 즉시 반영
    - can_see(zombie_pos: Vector2, target_pos: Vector2, light_level: float) -> bool
    - can_hear(zombie_pos: Vector2, noise_pos: Vector2, noise_level: float) -> bool
    - find_target() -> Option<Entity>

  requires:
    - sight_range > 0
    - hearing_range > sight_range  // 청각이 시각보다 먼 거리

  ensures:
    - if target found: distance to target <= max(sight_range, hearing_range)

  examples:
    - can_see((10, 10), (15, 10), 1.0) → true  // 밝은 곳에서 근거리
    - can_see((10, 10), (35, 10), 0.1) → false  // 어두운 곳에서 원거리
    - can_hear((10, 10), (30, 10), 15.0) → true  // 큰 소음
```

**완료 기준:**

- 좀비가 플레이어 추적
- 생존 게이지가 시간에 따라 감소
- 음식/물 섭취로 회복

#### Month 5-6: 건설 & 제작 시스템

**작업:**

- [ ] 바리케이드 시스템
- [ ] 제작 시스템 (레시피)
- [ ] 건축 (벽, 문)
- [ ] 루팅 (컨테이너 검색)
- [ ] Day/Night 사이클

**Pole 예시 코드:**

```pole
type CraftingSystem:
  purpose: Item crafting with recipes

  type Recipe:
    - name: string
    - inputs: Map<ItemType, int>  // 재료
    - output: ItemType
    - skill_required: Map<SkillType, int>
    - time: float  // 초

  function can_craft(player: Player, recipe: Recipe) -> bool:
    purpose: Check if player can craft item

    requires:
      - player.inventory is valid
      - recipe is valid

    ensures:
      - result = true iff player has all materials and skills

    implementation:
      - Check materials in inventory
      - Check skill levels
      - Return true if both pass

  @hot_reload
  data recipes:
    - name: "Stone Axe"
      inputs:
        - Stick: 1
        - Rock: 2
        - Rope: 1
      output: StoneAxe
      skill_required:
        - Crafting: 1
      time: 10.0
```

**완료 기준:**

- 플레이어가 간단한 도구 제작 가능
- 건물 창문에 바리케이드 설치 가능
- 밤에 어두워지고 좀비 활성화

### Phase 2: 반복 개발 (Month 7-15)

#### Month 7-9: 콘텐츠 확장

**Sprint 1-3:**

- [ ] 스킬 시스템 (10개 스킬)
- [ ] 다양한 무기 (총기, 근접)
- [ ] 농사 시스템
- [ ] 낚시 & 덫
- [ ] 차량 시스템 (간단한 버전)

#### Month 10-12: 멀티플레이어

**작업:**

- [ ] Client-server 아키텍처
- [ ] 플레이어 동기화
- [ ] 좀비/아이템 동기화
- [ ] 세이프하우스 시스템
- [ ] 보이스 채팅 (선택)

**Pole 예시 코드:**

```pole
@networked
type PlayerState:
  fields:
    - position: Vector2 @replicated
    - health: float @replicated
    - hunger: float @replicated
    - inventory: Inventory @replicated
    - current_action: PlayerAction @replicated

  methods:
    @server_rpc
    - move_to(target: Vector2)

    @server_rpc
    - use_item(item_id: int)

    @client_rpc
    - update_stats(health: float, hunger: float, thirst: float)

function zombie_spawn_manager:
  purpose: Spawn zombies around players (server-side)

  requires:
    - player_count > 0

  ensures:
    - zombie_density appropriate for player count
    - zombies spawn outside player sight range

  implementation:
    - For each player: calculate spawn zones (outside sight)
    - Spawn zombies based on difficulty + time survived
    - Migrate zombie hordes toward noise/players
```

#### Month 13-15: 월드 & 콘텐츠

**작업:**

- [ ] 대규모 맵 (3-5개 타운)
- [ ] 다양한 건물 타입 (집, 상점, 병원, 경찰서)
- [ ] 날씨 시스템
- [ ] 계절 변화
- [ ] 전기/수도 차단 시스템

### Phase 3: Early Access (Month 16-18)

**목표**: 안정적인 Early Access 출시

#### Month 16: Early Access 준비

**작업:**

- [ ] UI/UX 폴리싱
- [ ] 튜토리얼 (첫 1시간 가이드)
- [ ] 사운드/음악 완성
- [ ] 성능 최적화 (60 FPS 목표)
- [ ] Steam 페이지 준비

**완료 기준:**

- 5-10시간 플레이 타임
- 멀티플레이어 안정적 (4-8명)
- 크래시율 < 1%
- Steam Early Access 출시

#### Month 17-18: Early Access 운영

**작업:**

- [ ] 커뮤니티 피드백 수집
- [ ] 주간 패치 (버그 수정)
- [ ] 밸런스 조정
- [ ] 추가 콘텐츠 (무기, 레시피)

**목표:**

- 긍정적 리뷰 70%+
- 동시 접속자 100-500명
- Discord 커뮤니티 500+ 멤버

### Phase 4: 정식 출시 준비 (Month 19-24)

#### Month 19-22: 기능 완성

**작업:**

- [ ] NPC 시스템 (선택적)
- [ ] 고급 건설 (멀티 층 건물)
- [ ] 의료 시스템 확장
- [ ] 심리 시스템 (우울증, 불안, 공황)
- [ ] 전체 맵 완성
- [ ] 스토리 모드 (선택적)

#### Month 23-24: 폴리싱 & 런칭

**작업:**

- [ ] 최종 버그 수정
- [ ] 성능 최적화
- [ ] 런치 트레일러
- [ ] 마케팅 (스트리머, 리뷰어)
- [ ] 정식 출시

**완료 기준:**

- 50-100시간 플레이 타임
- 크래시율 < 0.1%
- 멀티플레이어 32명+ 지원
- 모딩 지원 완성

---

## 예산 및 수익 목표

### 개발 예산

| 단계             | 기간        | 예산      | 용도                 |
| ---------------- | ----------- | --------- | -------------------- |
| **MVP**          | Month 1-6   | $20K      | 기본 개발, 최소 에셋 |
| **반복 개발**    | Month 7-15  | $80K      | 멀티플레이어, 콘텐츠 |
| **Early Access** | Month 16-18 | $30K      | 폴리싱, 서버 비용    |
| **정식 출시**    | Month 19-24 | $70K      | 완성도, 마케팅       |
| **총 예산**      | 24개월      | **$200K** |                      |

### 자금 조달 계획

- **Month 1-6** (MVP): 자비 ($20K)
- **Month 7** (크라우드펀딩): Kickstarter 목표 $50K
- **Month 16** (Early Access): 선판매 수익으로 개발 지속
- **Month 24** (정식 출시): 수익으로 라이브 서비스 운영

### 수익 목표

#### Early Access

| 시점              | 판매량             | 가격 | 수익       |
| ----------------- | ------------------ | ---- | ---------- |
| 런치 (Month 16)   | 5,000-10,000       | $15  | $75K-150K  |
| +3개월 (Month 18) | 15,000-30,000 누적 | $15  | $225K-450K |

#### 정식 출시

| 시점            | 판매량              | 가격     | 수익       |
| --------------- | ------------------- | -------- | ---------- |
| 런치 (Month 24) | 50,000-100,000 누적 | $20      | $1M-2M     |
| +6개월          | 100,000-300,000     | $15 평균 | $1.5M-4.5M |
| +1년            | 200,000-500,000     | $12 평균 | $2.4M-6M   |

#### 비즈니스 목표

**최소 목표** (손익분기):

- 판매량: 10,000 카피
- 수익: $200K (개발 비용 회수)
- 리뷰: 긍정적 70%+

**중간 목표** (다음 프로젝트):

- 판매량: 100,000 카피
- 수익: $1.5M-2M
- 리뷰: 긍정적 85%+
- 동시 접속: 1,000-5,000명

**최대 목표** (시리즈화):

- 판매량: 500,000+ 카피
- 수익: $5M-10M
- 리뷰: 긍정적 90%+ (Very Positive)
- 동시 접속: 10,000-30,000명
- 모딩 커뮤니티 활성화

---

## 기술 구현 상세

### 1. 아이소메트릭 렌더링 시스템

#### 타일 좌표 변환

```pole
function isometric_projection:
  purpose: Convert world coordinates to screen coordinates

  input:
    - world_x: float  // 월드 X 좌표
    - world_y: float  // 월드 Y 좌표
    - tile_width: float = 64.0
    - tile_height: float = 32.0

  output: Vector2  // 스크린 좌표

  implementation:
    screen_x = (world_x - world_y) * (tile_width / 2)
    screen_y = (world_x + world_y) * (tile_height / 2)
    return Vector2(screen_x, screen_y)

  examples:
    - isometric_projection(0, 0, 64, 32) → (0, 0)
    - isometric_projection(1, 0, 64, 32) → (32, 16)
    - isometric_projection(0, 1, 64, 32) → (-32, 16)
```

#### 렌더링 순서 (Depth Sorting)

```pole
type TileLayer:
  - Floor = 0      // 바닥
  - Objects = 1    // 가구, 좀비, 플레이어
  - Walls = 2      // 벽
  - Roof = 3       // 지붕

function render_order:
  purpose: Determine rendering order for isometric depth

  requires:
    - tiles are sorted by (layer, y, x)

  ensures:
    - tiles render back-to-front (painter's algorithm)
    - no visual glitches

  implementation:
    - Sort by layer first (floor → roof)
    - Within layer, sort by Y (north to south)
    - Within same Y, sort by X (west to east)
```

### 2. 좀비 AI 시스템

#### 시야 시스템 (Line of Sight)

```pole
type VisionSystem:
  purpose: Raycasting-based line of sight

  function has_line_of_sight(
    from: Vector2,
    to: Vector2,
    world: TileWorld
  ) -> bool:

    purpose: Check if zombie can see target

    requires:
      - from and to are valid world coordinates
      - world contains tile data

    ensures:
      - returns true only if unobstructed path exists

    implementation:
      - Cast ray from 'from' to 'to'
      - Check each tile along ray
      - Return false if wall/obstacle found
      - Consider light level (dark = reduced range)

    examples:
      - has_line_of_sight((5, 5), (6, 5), open_field) → true
      - has_line_of_sight((5, 5), (6, 5), walled_area) → false
```

#### 군집 행동 (Crowd Simulation)

```pole
type ZombieHorde:
  purpose: Manage zombie crowd behavior

  fields:
    - zombies: Array<Entity>
    - center_of_mass: Vector2
    - target: Option<Vector2>

  methods:
    @hot_reload  // 밸런스 조정용
    - update_formation(delta: float)
    - migrate_toward(target: Vector2, speed: float)
    - avoid_collisions()

  behavior rules:
    - Follow target if detected
    - Maintain minimum distance from other zombies
    - Clump together when idle (migration)
    - Sound attracts entire horde

  implementation:
    - Use spatial partitioning (grid) for neighbor queries
    - Apply flocking algorithm (cohesion, separation, alignment)
    - Pathfind to target, avoid obstacles
```

### 3. 생존 시스템

#### 멀티 욕구 시스템

```pole
type SurvivalNeeds:
  purpose: Manage player survival stats

  fields:
    - hunger: float = 100.0  // 0 = starving
    - thirst: float = 100.0  // 0 = dehydrated
    - fatigue: float = 0.0   // 100 = exhausted
    - temperature: float = 37.0  // 체온 (℃)

  methods:
    @hot_reload
    - update(delta: float, environment: Environment)
    - consume_food(food: FoodItem)
    - consume_water(water: WaterItem)
    - rest(quality: float, duration: float)

  @hot_reload
  decay_rates:
    - hunger: -0.5 per minute
    - thirst: -1.0 per minute (갈증이 더 빠름)
    - fatigue: +0.3 per minute (활동 중)
    - temperature: affected by weather, clothing

  effects:
    - hunger < 20: movement speed -30%, damage -20%
    - thirst < 20: vision range -50%, confusion
    - fatigue > 80: cannot sprint, -50% accuracy
    - temperature < 35 or > 40: health damage over time

  ensures:
    - all values clamped [0, 100] except temperature
    - death occurs if hunger = 0 for 3+ days or thirst = 0 for 1+ day
```

#### 부상 & 질병 시스템

```pole
type InjurySystem:
  purpose: Manage wounds, infections, and healing

  type Wound:
    - location: BodyPart  // 머리, 팔, 다리, 몸통
    - type: WoundType  // 긁힘, 깊은 상처, 물림, 골절
    - severity: float  // 0-100
    - is_infected: bool
    - is_bandaged: bool
    - time_since_injury: float

  function apply_wound(
    player: Player,
    location: BodyPart,
    wound_type: WoundType,
    severity: float
  ):
    purpose: Add injury to player

    requires:
      - severity >= 0 and severity <= 100

    ensures:
      - wound added to player.wounds
      - health decreased
      - bleeding starts if severity > 30

    implementation:
      - Create Wound instance
      - Apply immediate health damage
      - Start bleeding timer
      - Check for infection chance (bite = 100%)

  @hot_reload
  healing_rates:
    - Scratch: 1 day (if bandaged)
    - Deep Wound: 5-7 days (requires stitches)
    - Bite: never heals, always infected → zombification
    - Fracture: 2-4 weeks (requires splint)

  infection_system:
    - Check infection every 6 hours
    - Infection chance: wound severity × hygiene factor
    - Infected wound: fever, health drain
    - Treatment: antibiotics, disinfectant
```

### 4. 건설 & 제작 시스템

#### 건축 시스템

```pole
type BuildingSystem:
  purpose: Allow players to build structures

  type BuildAction:
    - item_type: ConstructionType  // 벽, 문, 창문, 계단
    - position: Vector2
    - rotation: int  // 0, 90, 180, 270

  function can_build(
    player: Player,
    action: BuildAction,
    world: TileWorld
  ) -> Result<bool, BuildError>:

    purpose: Validate building action

    requires:
      - player has required materials
      - player has required skill level
      - position is valid and empty

    ensures:
      - returns Ok(true) only if all checks pass
      - returns Err with specific reason if failed

    implementation:
      - Check materials in inventory
      - Check skill (Carpentry for wood, Metalworking for metal)
      - Check tile is empty or supports construction
      - Check nearby support (walls need floor/foundation)

  @hot_reload
  construction_recipes:
    - Wooden Wall:
        materials:
          - Plank: 4
          - Nail: 8
        tools: Hammer
        skill: Carpentry 1
        time: 30 seconds
        hp: 300

    - Metal Door:
        materials:
          - Metal Sheet: 3
          - Hinge: 2
          - Screws: 12
        tools: Screwdriver
        skill: Metalworking 2
        time: 60 seconds
        hp: 800
```

#### 제작 시스템 (확장)

```pole
type CraftingStation:
  purpose: Crafting with specialized workbenches

  enum StationType:
    - None  // 손으로 제작
    - Campfire  // 요리
    - Workbench  // 고급 제작
    - Anvil  // 금속 작업

  function craft_item(
    player: Player,
    recipe: Recipe,
    station: Option<StationType>
  ) -> Result<Item, CraftError>:

    purpose: Craft item with materials and station

    requires:
      - player.has_materials(recipe.inputs)
      - player.has_skill(recipe.skill_required)
      - station matches recipe.required_station

    ensures:
      - materials consumed
      - item created
      - skill XP granted

    implementation:
      - Validate requirements
      - Consume materials from inventory
      - Wait craft_time (can be interrupted)
      - Grant skill XP based on recipe difficulty
      - Add crafted item to inventory

  @hot_reload
  advanced_recipes:
    - Molotov Cocktail:
        inputs:
          - Empty Bottle: 1
          - Gasoline: 100ml
          - Ripped Sheet: 1
        required_station: None
        skill: None
        time: 5 seconds
        output: MolotovCocktail

    - Pipe Bomb:
        inputs:
          - Pipe: 1
          - Gunpowder: 50g
          - Electronics: 1
          - Duct Tape: 1
        required_station: Workbench
        skill: Electrical 3
        time: 120 seconds
        output: PipeBomb
```

### 5. 멀티플레이어 시스템

#### 상태 동기화

```pole
@networked
type WorldState:
  purpose: Synchronized game world state

  fields:
    - time: GameTime @replicated
    - weather: Weather @replicated
    - power_grid: bool @replicated  // 전기 on/off
    - water_grid: bool @replicated  // 수도 on/off

  @server_rpc
  function advance_time(hours: float):
    purpose: Server controls time progression

    ensures:
      - time advances consistently for all clients
      - events triggered at specific times (power shutdown, etc.)

@networked
type MultiplayerZombie:
  purpose: Network-synchronized zombie

  fields:
    - position: Vector2 @replicated(interpolated)  // 보간 적용
    - target: Option<EntityId> @replicated
    - state: ZombieState @replicated  // idle, walking, attacking

  optimization:
    - Only sync position every 100ms (not every frame)
    - Use client-side prediction for smooth movement
    - Server is authoritative for attacks and damage

function safehouse_system:
  purpose: Team base protection and spawning

  type Safehouse:
    - owner_team: TeamId
    - bounds: Rectangle
    - is_pvp_protected: bool
    - spawn_point: Vector2

  rules:
    - Team members can spawn at safehouse
    - Other teams cannot build inside (if PvP protected)
    - Loot containers inside are private
    - Door access controlled by team permissions
```

### 6. 모딩 지원

#### Pole 모딩 API

```pole
@mod_api
type PoleModAPI:
  purpose: Allow players to mod the game with Pole scripts

  available_hooks:
    - on_player_spawn(player: Player)
    - on_zombie_death(zombie: Zombie, killer: Option<Player>)
    - on_item_craft(player: Player, item: Item)
    - on_building_construct(player: Player, building: Construction)
    - on_time_advance(old_time: GameTime, new_time: GameTime)

  data_access:
    - Read game data (items, recipes, etc.)
    - Add new items, recipes, constructions
    - Modify existing data (balance mods)
    - Cannot access network internals (anti-cheat)

  example_mod:
    ``pole
    @mod("Hardcore Survival")
    mod hardcore_survival:

      @hook(on_player_spawn)
      function make_it_harder(player: Player):
        player.hunger = 50.0  // 배고픔 상태로 시작
        player.thirst = 30.0  // 목마른 상태
        player.inventory.remove_all()  // 아이템 없이 시작

      @hot_reload
      @hook(on_zombie_spawn)
      function stronger_zombies(zombie: Zombie):
        zombie.health *= 1.5  // 좀비 체력 50% 증가
        zombie.speed *= 1.2   // 좀비 속도 20% 증가
    ``
```

---

## 리스크 관리

### 기술적 리스크

#### 1. 성능 (대규모 좀비 시뮬레이션)

**리스크**: 수천 좀비 + 거대한 월드 → 낮은 FPS

**완화 전략:**

- 공간 분할 (Quad Tree, Grid)
- 시야 밖 좀비 비활성화
- LOD (Level of Detail): 먼 좀비는 단순 AI
- Chunk 기반 로딩
- ECS 최적화 (캐시 친화적)

**비상 계획:**

- 좀비 최대 개수 제한 (1000-2000)
- 멀티코어 병렬 처리 (Pole 동시성)

#### 2. 네트워크 동기화 (멀티플레이어)

**리스크**: 대량 오브젝트 동기화 → 높은 대역폭/지연

**완화 전략:**

- 관심 영역 관리 (플레이어 주변만 동기화)
- 예측 + 보정 (Client Prediction)
- Delta compression (변경된 것만 전송)
- 서버 권위 (치트 방지)

**비상 계획:**

- 서버 플레이어 수 제한 (32명)
- 지역별 서버 (레이턴시 감소)

#### 3. Pole Engine 미완성 기능

**리스크**: 필요한 기능이 Pole Engine에 없음

**완화 전략:**

- 사전 Pole Engine 팀과 협의
- 필수 기능 리스트 공유
- 베타 단계 Pole Engine 선택 시 리스크 감수

**비상 계획:**

- 일부 기능을 Pole로 직접 구현
- 또는 기존 라이브러리 FFI 연동

### 비즈니스 리스크

#### 1. 시장 경쟁 (Project Zomboid와 비교)

**리스크**: "Project Zomboid 짝퉁" 인식

**완화 전략:**

- 차별화 요소 강조 (Pole LLM AI, 더 나은 UI)
- "오마주/영감" 명시
- 커뮤니티 투명성 (개발 일지)
- 독창적 콘텐츠 추가 (스토리 모드, NPC)

**비상 계획:**

- 가격 경쟁력 ($15 vs $20)
- 무료 주말 이벤트
- 적극적인 모딩 지원

#### 2. 개발 지연

**리스크**: 24개월 목표 초과

**완화 전략:**

- MVP 우선 (핵심 루프 먼저)
- 애자일 방식 (2주 스프린트)
- Early Access로 피드백 조기 수집
- 일부 기능 Post-launch DLC로 연기

**비상 계획:**

- Early Access 기간 연장 (1년+)
- 팀 확장 (외주 활용)

---

## 마일스톤 요약

### Milestone 1: MVP 완성 (Month 6)

- 플레이 가능한 프로토타입
- 좀비 AI + 생존 시스템
- 1개 작은 타운
- 1-2시간 플레이 타임

### Milestone 2: Early Access (Month 16)

- 멀티플레이어 지원
- 3-5개 타운
- 5-10시간 플레이 타임
- Steam 출시

### Milestone 3: 정식 출시 (Month 24)

- 전체 맵 완성
- 50-100시간 플레이 타임
- 멀티플레이어 32명+
- 모딩 지원
- 긍정적 리뷰 85%+

---

## 핵심 성공 요인

### Pole 언어/엔진 활용

1. **LLM 기반 AI 개발**

   - 좀비 행동을 자연어로 명세
   - 복잡한 생존 로직 자동 생성
   - 빠른 프로토타이핑

2. **핫 리로딩**

   - 밸런스 조정 즉시 반영
   - QA 시간 단축
   - 커뮤니티 피드백 빠른 적용

3. **계약 프로그래밍**
   - 버그 사전 방지
   - 멀티플레이어 안정성
   - 세이브 파일 무결성

### 커뮤니티 중심 개발

1. **Early Access 활용**

   - 조기 피드백 수집
   - 커뮤니티와 함께 성장
   - Project Zomboid의 성공 사례

2. **모딩 지원**

   - Pole 스크립트로 쉬운 모딩
   - Steam Workshop 통합
   - 커뮤니티 콘텐츠

3. **투명성**
   - 주간 개발 일지
   - 공개 로드맵
   - Discord 소통

---

## 결론

**Pole Survival**은 Project Zomboid의 핵심 메커니즘을 Pole Engine으로 재현하는 프로젝트입니다.

**차별화 요소:**

- LLM 기반 AI (자연어 명세)
- 향상된 UI/UX
- 안정적인 멀티플레이어
- 크로스 플랫폼
- Pole 모딩 지원

**개발 기간:** 24개월  
**예산:** $200K  
**목표 판매량:** 100,000-500,000 카피  
**예상 수익:** $1.5M-6M

**다음 단계:**

1. 팀 구성
2. Pole Engine 기능 협의
3. MVP 개발 시작 (Month 1)

---

**Pole로 좀비 서바이벌의 새로운 기준을 만들어갑시다!** 🧟‍♂️🔨
