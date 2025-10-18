# 게임 개발 로드맵 (Pole Engine 활용)

> **주의**: Pole Engine으로 게임을 개발하는 실전 가이드
> 
> **전제 조건**: Pole Engine 사용 가능 (GAME_ENGINE_VISION.md 참조)

**작성일**: 2025-10-19  
**문서 성격**: 실전 검증 + 상용 출시 + 커뮤니티 성장 + Pole 홍보

---

## 📋 문서 구조

이 로드맵은 **모듈형 구조**로 작성되어 있습니다:

- **Part 1**: 공통 개발 프로세스 (모든 게임에 적용)
- **Part 2**: 장르별 상세 가이드 (필요한 부분만 선택)
  - 2.1 2D 플랫포머
  - 2.2 3D FPS
  - 2.3 3D RPG
  - 2.4 멀티플레이어 게임

---

# Part 1: 공통 개발 프로세스

## 🎯 로드맵 목적

이 로드맵은 **4가지 목적**을 동시에 달성합니다:

1. **Pole Engine 실전 검증** - 엔진 기능과 성능을 실제 게임으로 검증
2. **상용 게임 출시** - 수익 창출 가능한 상용 게임 제작
3. **커뮤니티 성장** - 오픈소스 샘플 및 학습 자료 제공
4. **Pole 홍보** - Pole 언어와 엔진의 장점 입증

---

## 🏗️ 개발 단계 (애자일 방식)

모든 게임 프로젝트는 다음 **5단계**를 거칩니다:

### 1. MVP (Minimum Viable Product)
**목표**: 핵심 메커니즘 검증
- 기간: 전체 개발의 10-20%
- 산출물: 플레이 가능한 프로토타입
- 품질: 30 FPS 허용, 핵심 기능만 작동

### 2. 반복 개발 (Iterative Development)
**목표**: 기능 추가 및 개선
- 기간: 전체 개발의 40-50%
- 방식: 2주 스프린트
- 품질: 60 FPS, 주요 버그 수정
- 활동: 기능 추가, 플레이테스트, 피드백 반영

### 3. Early Access
**목표**: 실사용자 피드백 및 초기 수익
- 기간: 전체 개발의 20-30%
- 출시: Steam, itch.io
- 품질: 크래시율 < 1%, 기본 콘텐츠 완성
- 가격: 정가의 70-80% 할인

### 4. 정식 출시 (Release)
**목표**: 완성된 게임 런칭
- 품질: 크래시율 < 0.1%, 전체 콘텐츠 완성
- 가격: 정가 책정
- 마케팅: 트레일러, 리뷰어 배포, 런치 이벤트

### 5. 라이브 서비스 (Live Service)
**목표**: 운영 및 추가 수익
- 활동: 버그 패치, 밸런스 조정
- 콘텐츠: DLC, 시즌 패스, 코스메틱
- 기간: 장르에 따라 3개월 ~ 지속적

---

## 🛠️ Pole 언어 활용 전략

### 100% Pole 개발 원칙

모든 게임 로직과 시스템을 **Pole 언어로만 개발**합니다:

```pole
// 게임 로직 - Pole 언어
function player_movement:
  purpose: Handle player movement with physics
  requires: input.is_valid()
  ensures: player.position is within world_bounds
  
  implementation:
    - Read input (WASD, gamepad)
    - Apply physics (velocity, acceleration)
    - Handle collisions
    - Update animation state

// 엔진 확장 - Pole 언어
@component
type PlayerController:
  fields:
    - speed: float = 5.0
    - jump_force: float = 10.0
    - is_grounded: bool
  
  methods:
    - update(delta: float)
    - handle_input()
    - apply_physics()

// 쉐이더 - Pole 쉐이더 언어
@shader
function pixel_shader:
  input: vertex_output
  output: color (vec4)
  
  implementation:
    - Sample texture
    - Apply lighting
    - Output final color
```

### Pole의 장점 활용

1. **자연어 명세로 빠른 프로토타이핑**
   - 게임 디자이너도 로직 작성 가능
   - LLM이 최적화된 구현 생성

2. **자동 검증으로 버그 감소**
   - 컴파일 타임 타입 체크
   - 계약 프로그래밍 (requires/ensures)
   - 자동 테스트 생성

3. **핫 리로딩으로 빠른 반복**
   - 코드 수정 즉시 반영
   - 게임 상태 유지
   - 재컴파일 불필요

---

## 👥 팀 구성 원칙

### Pole 언어 스킬 요구사항

**모든 팀원 (필수):**
- Pole 기본 문법 이해
- LLM 활용법 (명세 작성 방법)
- Pole Engine 에디터 사용법

**프로그래머 (추가):**
- Pole 중급 이상
- 게임 프로그래밍 패턴
- Pole 표준 라이브러리 숙지

**기존 게임 개발 경험:**
- 권장하지만 필수 아님
- Pole의 낮은 진입장벽 활용

---

## 💰 수익 전략 (단계별)

### 1. MVP 단계
- **가격**: 무료 배포
- **목적**: 피드백 수집, 커뮤니티 형성
- **플랫폼**: itch.io, 게임잼, 소셜미디어

### 2. 반복 개발 단계
- **자금**: 크라우드펀딩 (Kickstarter, Tumblbug)
- **목표**: $20K-50K (장르에 따라 조정)
- **혜택**: Early Access 할인, 특전 아이템

### 3. Early Access
- **가격**: $10-20 (정가의 70-80%)
- **플랫폼**: Steam, Epic Games Store
- **목적**: 개발 자금 확보, 실사용자 피드백

### 4. 정식 출시
- **가격**: $20-40 (장르와 규모에 따라)
- **마케팅**: 트레일러, 스트리머/리뷰어 협업
- **목표**: 손익분기점 달성

### 5. 라이브 서비스
- **DLC/확장팩**: $5-15
- **시즌 패스**: $10-20
- **코스메틱**: $1-5 (F2P 게임의 경우)

---

## 💵 자금 계획 (단계별)

### Phase 1: MVP (자비 제작)
- **예산**: $5K-10K
- **용도**: 기본 개발 비용, 최소 에셋
- **조달**: 팀원 자비, 소규모 투자

### Phase 2: 반복 개발 (크라우드펀딩)
- **예산**: $20K-50K
- **플랫폼**: Kickstarter, Tumblbug
- **마케팅**: 프로토타입 영상, 데모 제공
- **리워드**: Early Access 키, 특전

### Phase 3: Early Access (선판매 수익)
- **수익**: 초기 판매로 개발 지속
- **재투자**: 콘텐츠 확장, 품질 개선
- **목표**: 자립 가능한 개발 속도 유지

### Phase 4: 정식 출시 (퍼블리셔 선택)
- **옵션 A**: 인디 퍼블리셔 협업 (마케팅 지원)
- **옵션 B**: 셀프 퍼블리싱 (수익률 높음)
- **선택 기준**: 팀 역량, 마케팅 경험

---

## 📊 성공 지표

### 기술적 목표 (단계별)

| 단계 | 성능 | 안정성 | 콘텐츠 | 품질 |
|------|------|--------|--------|------|
| **MVP** | 30 FPS | 크래시 허용 | 핵심 메커니즘 | 프로토타입 |
| **반복 개발** | 60 FPS | 크리티컬 버그 0개 | 기본 콘텐츠 | 알파 품질 |
| **Early Access** | 60 FPS | 크래시율 < 1% | 70% 콘텐츠 | 베타 품질 |
| **정식 출시** | 60 FPS | 크래시율 < 0.1% | 100% 콘텐츠 | 프로덕션 품질 |
| **라이브 서비스** | 최적화 | 모니터링 | 추가 DLC | 지속 개선 |

### 비즈니스 목표 (3단계)

#### 최소 목표 (손익분기)
- 개발 비용 회수
- 팀원 급여 지급
- 긍정적 리뷰 70%+

#### 중간 목표 (다음 프로젝트)
- 개발 비용 2-3배 수익
- 다음 프로젝트 자금 확보
- 긍정적 리뷰 80%+
- 커뮤니티 형성 (Discord 500+ 멤버)

#### 최대 목표 (스튜디오 성장)
- 개발 비용 5배+ 수익
- 팀 확장 및 스튜디오 설립
- 긍정적 리뷰 85%+
- 강력한 IP 구축 (시리즈화)

---

## 🔧 개발 방법론 (단계별 가이드)

### MVP 단계

**기간**: 전체 개발의 10-20%

**프로세스:**
- 핵심 기능만 구현 (80/20 법칙)
- 주 1회 플레이테스트
- 빠른 프로토타이핑 (Pole의 LLM 활용)
- 문서화 최소화

**체크리스트:**
- [ ] 핵심 게임 메커니즘 작동
- [ ] 기본 그래픽 (플레이스홀더 허용)
- [ ] 5-10분 플레이 가능
- [ ] 내부 플레이테스트 완료

### 반복 개발 단계

**기간**: 전체 개발의 40-50%

**프로세스:**
- 2주 스프린트 사이클
- 스프린트 시작: 계획 및 작업 배분
- 스프린트 중: 데일리 체크인 (선택)
- 스프린트 종료: 리뷰 및 회고
- 지속적 통합 (CI/CD)

**각 스프린트:**
1. **기능 추가**: 새 메커니즘, 콘텐츠
2. **버그 수정**: 이전 스프린트 이슈
3. **플레이테스트**: 내부 + 외부 테스터
4. **피드백 반영**: 다음 스프린트 계획

**도구:**
- 작업 관리: Notion, Trello, GitHub Projects
- 버전 관리: Git + GitHub
- 빌드: Pole Engine 빌드 시스템
- 테스트: Pole 자동 테스트 + 수동 QA

### Early Access 단계

**기간**: 전체 개발의 20-30%

**프로세스:**
- 주간 또는 격주 업데이트
- 커뮤니티 피드백 우선
- 공개 로드맵 관리
- 투명한 커뮤니케이션

**활동:**
1. **버그 수정**: 사용자 리포트 대응
2. **밸런스 조정**: 데이터 기반 조정
3. **콘텐츠 추가**: 로드맵에 따라
4. **커뮤니티 관리**: Discord, Steam 포럼

**커뮤니케이션:**
- 주간 개발 일지 (Steam 뉴스)
- 패치 노트 상세 작성
- 커뮤니티 Q&A 세션
- 버그 리포트 신속 대응

### 정식 출시 단계

**준비 기간**: 1-2개월

**체크리스트:**
- [ ] 모든 크리티컬 버그 수정
- [ ] 전체 콘텐츠 완성
- [ ] 튜토리얼 및 온보딩
- [ ] 런치 트레일러 제작
- [ ] 리뷰어/스트리머 키 배포
- [ ] 마케팅 자료 준비
- [ ] 가격 최종 결정
- [ ] 플랫폼 심사 통과

**런치 주:**
- 소셜미디어 집중 홍보
- 스트리머 동시 방송
- 런치 할인 (10-20%, 1주일)
- 실시간 모니터링 (크래시, 버그)
- 긴급 패치 준비

### 라이브 서비스 단계

**기간**: 장르에 따라 다름
- 단일 플레이: 3-6개월 (버그 패치)
- 멀티플레이: 지속적 (콘텐츠 업데이트)

**활동:**
1. **유지보수**: 버그 패치, 밸런스
2. **콘텐츠 업데이트**: DLC, 시즌
3. **커뮤니티 이벤트**: 대회, 챌린지
4. **모니터링**: 플레이어 행동 분석

---

## 🎮 Pole Engine 기능 요구사항 (공통)

### 최소 요구사항 (모든 게임)

**필수 라이브러리:**
- `pole_core`: 기본 시스템, 루프, 이벤트
- `pole_graphics`: 렌더링 (2D 또는 3D)
- `pole_input`: 입력 처리 (키보드, 마우스, 게임패드)
- `pole_audio`: 오디오 재생 (음악, 효과음)

**엔진 성숙도:**
- **알파**: 프로토타입 및 MVP 가능
- **베타**: Early Access 가능
- **프로덕션**: 정식 출시 권장

### 권장 라이브러리

- `pole_ecs`: Entity Component System (성능 최적화)
- `pole_physics`: 물리 엔진 (충돌, 중력)
- `pole_ui`: UI 시스템 (메뉴, HUD)
- `pole_assets`: 에셋 관리 (비동기 로딩)

---

# Part 2: 장르별 상세 가이드

## 2.1 2D 플랫포머

### 📋 프로젝트 개요

**장르**: 2D 플랫포머 (예: Celeste, Hollow Knight 스타일)  
**타겟 플랫폼**: PC (Steam, itch.io), 모바일 (선택)  
**개발 기간**: 6개월  
**팀 규모**: 1-3명  

### 👥 팀 구성

**최소 팀 (1-2명):**
- **프로그래머 1명**
  - 역할: 게임 로직, 시스템 구현
  - Pole 스킬: 중급
  - 경험: 2D 게임 기본 이해
  
- **아티스트 1명**
  - 역할: 스프라이트, 애니메이션, 타일셋
  - 도구: Aseprite, Photoshop, Spine (선택)
  - 스타일: 픽셀아트 또는 벡터

**확장 팀 (3명):**
- 위 2명 +
- **디자이너/사운드 1명**
  - 역할: 레벨 디자인, 사운드 이펙트, 음악
  - 도구: Pole Engine 에디터, Audacity, LMMS

### 🛠️ Pole Engine 기능 요구사항

**필수 기능:**

```
pole_graphics (2D):
  ✅ Sprite rendering (batching)
  ✅ 2D camera (zoom, follow)
  ✅ Layers (background, midground, foreground)
  ✅ Sprite animation (frame-based)
  ✅ Tilemap rendering

pole_physics (2D):
  ✅ Box/Circle colliders
  ✅ Raycasting
  ✅ Gravity and velocity
  ✅ One-way platforms
  ✅ Slope handling

pole_input:
  ✅ Keyboard (WASD, arrows)
  ✅ Gamepad (Xbox, PlayStation)
  ✅ Input buffering (responsive controls)

pole_audio:
  ✅ Sound effects
  ✅ Music streaming (loop)
  ✅ Volume control
```

**선택 기능:**

```
pole_particles (2D):
  ⚠️ Particle system (dust, effects)

pole_ui:
  ⚠️ Health bar, score display
  ⚠️ Pause menu

pole_assets:
  ⚠️ Async asset loading
  ⚠️ Texture atlas
```

**엔진 성숙도:**
- **최소**: Pole Engine 알파 (프로토타입 가능)
- **권장**: Pole Engine 베타 (상용 출시)

### 📅 개발 일정 (6개월)

#### Month 1: MVP
**목표**: 플레이 가능한 핵심 메커니즘

**작업:**
- [ ] 플레이어 이동 (걷기, 점프)
- [ ] 기본 물리 (중력, 충돌)
- [ ] 1개 테스트 레벨
- [ ] 플레이스홀더 그래픽

**Pole 예시 코드:**
```pole
function player_movement:
  purpose: Handle player movement with physics
  input:
    - input_horizontal: float  // -1 to 1
    - is_jump_pressed: bool
  output: Vector2  // new velocity
  
  requires:
    - player.is_grounded or player.can_air_control
  
  ensures:
    - velocity.x <= max_speed
    - velocity.y is affected by gravity
  
  examples:
    - player_movement(1.0, false) → velocity(5.0, -2.0)  // moving right
    - player_movement(0.0, true) → velocity(0.0, 10.0)   // jumping
```

**완료 기준:**
- 5분 플레이 가능
- 기본 조작감 확인

#### Month 2-3: 반복 개발
**목표**: 핵심 기능 완성

**Sprint 1-2 (Month 2):**
- [ ] 적 AI (기본 패턴)
- [ ] 공격 메커니즘
- [ ] 체력/데미지 시스템
- [ ] 5개 레벨 제작

**Sprint 3-4 (Month 3):**
- [ ] 파워업/아이템
- [ ] 체크포인트 시스템
- [ ] 보스 전투 (1개)
- [ ] 최종 그래픽 적용 시작

**완료 기준:**
- 30분 플레이 가능
- 핵심 루프 완성

#### Month 4: Early Access 준비
**목표**: 출시 가능한 품질

**작업:**
- [ ] 10-15개 레벨 완성
- [ ] UI/UX 개선 (메뉴, HUD)
- [ ] 사운드/음악 추가
- [ ] 버그 수정 및 최적화
- [ ] Steam 페이지 준비

**완료 기준:**
- 1-2시간 플레이 가능
- 크래시율 < 1%
- Steam Early Access 출시

#### Month 5: Early Access 운영
**목표**: 피드백 반영 및 콘텐츠 추가

**작업:**
- [ ] 사용자 버그 리포트 대응
- [ ] 난이도 밸런스 조정
- [ ] 추가 레벨 5-10개
- [ ] 새 메커니즘 추가 (선택)

**완료 기준:**
- 2-3시간 플레이 타임
- 긍정적 리뷰 70%+

#### Month 6: 정식 출시
**목표**: 완성도 극대화

**작업:**
- [ ] 전체 레벨 폴리싱
- [ ] 엔딩 컨텐츠
- [ ] 런치 트레일러
- [ ] 마케팅 활동
- [ ] 최종 버그 수정

**완료 기준:**
- 3-5시간 플레이 타임
- 20-30개 레벨
- 크래시율 < 0.1%

### 💰 예산 및 수익 목표

**개발 예산:**
- MVP: $5K (자비)
- 반복 개발: $15K (크라우드펀딩 목표)
- 총 예산: $20K-30K

**수익 목표:**

| 단계 | 판매량 | 가격 | 수익 |
|------|--------|------|------|
| Early Access | 500-1,000 | $10 | $5K-10K |
| 정식 출시 | 5,000-10,000 | $15 | $75K-150K |
| 1년 후 | 10,000-20,000 | $12 (할인) | $120K-240K |

**목표:**
- 최소: 손익분기 ($20K)
- 중간: 2배 수익 ($40K-60K)
- 최대: 5배 수익 ($100K-150K)

---

## 2.2 3D FPS

### 📋 프로젝트 개요

**장르**: 3D FPS (예: DOOM, Titanfall 스타일)  
**타겟 플랫폼**: PC (Steam)  
**개발 기간**: 1년  
**팀 규모**: 3-5명  

### 👥 팀 구성

**핵심 팀 (3명):**
- **리드 프로그래머 1명**
  - 역할: 게임플레이, 시스템 아키텍처
  - Pole 스킬: 고급
  - 경험: 3D 게임, 물리 시뮬레이션
  
- **그래픽/엔진 프로그래머 1명**
  - 역할: 렌더링, 최적화, 쉐이더
  - Pole 스킬: 고급 (쉐이더 포함)
  - 경험: 3D 그래픽스, Vulkan/OpenGL
  
- **3D 아티스트 1명**
  - 역할: 모델링, 텍스처, 애니메이션
  - 도구: Blender, Substance Painter
  - 스타일: Low-poly 또는 realistic

**확장 팀 (5명):**
- 위 3명 +
- **게임플레이 프로그래머 1명**: 무기, AI
- **레벨 디자이너/사운드 1명**: 맵 디자인, 사운드

### 🛠️ Pole Engine 기능 요구사항

**필수 기능:**

```
pole_graphics (3D):
  ✅ 3D mesh rendering
  ✅ PBR materials (optional for MVP)
  ✅ FPS camera controller
  ✅ Skybox
  ✅ Basic lighting (directional, point)
  ✅ Shadow mapping (선택적)

pole_physics (3D):
  ✅ Character controller (capsule)
  ✅ Raycasting (hitscan weapons)
  ✅ Rigidbody (projectiles)
  ✅ Collision detection (level geometry)
  ✅ Trigger volumes

pole_input:
  ✅ Mouse look (FPS controls)
  ✅ Keyboard + Mouse
  ✅ Gamepad (aim assist)

pole_audio:
  ✅ 3D spatial audio
  ✅ Sound effects (weapons, footsteps)
  ✅ Music system

pole_ecs:
  ✅ High-performance entity management
  ✅ Component-based architecture
```

**선택 기능:**

```
pole_graphics (고급):
  ⚠️ Post-processing (bloom, DOF)
  ⚠️ Particle systems (muzzle flash, explosions)
  ⚠️ Decals (bullet holes)

pole_animation:
  ⚠️ Character animations (view model)
  ⚠️ Animation blending

pole_ai:
  ⚠️ Behavior trees (enemy AI)
  ⚠️ Pathfinding (navmesh)

pole_net:
  ⚠️ Multiplayer (Phase 2)
```

**엔진 성숙도:**
- **최소**: Pole Engine 베타 (3D 기능 안정화)
- **권장**: Pole Engine 1.0 (프로덕션)

### 📅 개발 일정 (12개월)

#### Month 1-2: MVP
**목표**: FPS 핵심 메커니즘

**작업:**
- [ ] FPS 카메라 및 이동
- [ ] 1개 무기 (레이캐스트)
- [ ] 기본 적 AI (정적 타겟)
- [ ] 1개 테스트 맵
- [ ] 플레이스홀더 모델

**Pole 예시:**
```pole
function raycast_weapon_fire:
  purpose: Hitscan weapon (instant hit)
  input:
    - origin: Vector3  // camera position
    - direction: Vector3  // aim direction
    - max_distance: float = 1000.0
  output: HitResult
  
  requires:
    - weapon.ammo > 0
    - not weapon.is_reloading
  
  ensures:
    - weapon.ammo decreased by 1
    - if hit: damage applied to target
  
  implementation:
    - Cast ray from origin in direction
    - Check collision with enemies/environment
    - Apply damage if hit
    - Spawn hit effects (particles, decals)
```

#### Month 3-6: 반복 개발
**Sprint 별 작업:**

**Month 3-4: 무기 및 전투**
- [ ] 3-5종 무기 (권총, 소총, 샷건, 로켓)
- [ ] 탄약 및 재장전 시스템
- [ ] 체력/피격 시스템
- [ ] 기본 HUD

**Month 5-6: AI 및 레벨**
- [ ] 적 AI (순찰, 추적, 공격)
- [ ] 3-5개 레벨 제작
- [ ] 레벨 오브젝트 (커버, 폭발물)
- [ ] 체크포인트

#### Month 7-9: Early Access 준비
**작업:**
- [ ] 전체 10-15개 레벨
- [ ] 보스 전투 2-3개
- [ ] 최종 그래픽 (모델, 텍스처)
- [ ] 사운드/음악 완성
- [ ] 난이도 밸런스

**완료 기준:**
- 3-4시간 캠페인
- 60 FPS (1080p)
- Early Access 출시

#### Month 10-11: Early Access 운영
**작업:**
- [ ] 버그 수정
- [ ] 추가 레벨/무기
- [ ] 커뮤니티 피드백 반영
- [ ] 성능 최적화

#### Month 12: 정식 출시
**작업:**
- [ ] 최종 폴리싱
- [ ] 엔딩 및 크레딧
- [ ] 마케팅 (트레일러, 스트리머)
- [ ] 런치

### 💰 예산 및 수익 목표

**개발 예산:**
- MVP: $10K
- 반복 개발: $40K (크라우드펀딩)
- 총 예산: $50K-100K

**수익 목표:**

| 단계 | 판매량 | 가격 | 수익 |
|------|--------|------|------|
| Early Access | 2,000-5,000 | $20 | $40K-100K |
| 정식 출시 | 10,000-30,000 | $25 | $250K-750K |
| 1년 후 | 30,000-100,000 | $20 (할인) | $600K-2M |

**목표:**
- 최소: 손익분기 ($50K)
- 중간: 3배 수익 ($150K-300K)
- 최대: 10배 수익 ($500K-1M)

---

## 2.3 3D RPG

### 📋 프로젝트 개요

**장르**: 3D 액션 RPG (예: Dark Souls, Elden Ring 스타일)  
**타겟 플랫폼**: PC, 콘솔 (PlayStation, Xbox)  
**개발 기간**: 1.5-2년  
**팀 규모**: 5-10명  

### 👥 팀 구성

**핵심 팀 (5명):**
- **리드 프로그래머 1명**: 시스템 아키텍처
- **게임플레이 프로그래머 1명**: 전투, 스킬
- **UI/시스템 프로그래머 1명**: 인벤토리, 퀘스트
- **3D 아티스트 1명**: 캐릭터, 환경
- **레벨 디자이너 1명**: 월드 디자인

**확장 팀 (10명):**
- 위 5명 +
- **애니메이터 1명**: 캐릭터 애니메이션
- **컨셉 아티스트 1명**: 아트 디렉션
- **사운드 디자이너 1명**: 음악, 효과음
- **QA 테스터 1명**: 밸런스, 버그
- **나레이션/라이터 1명**: 스토리, 대사

### 🛠️ Pole Engine 기능 요구사항

**필수 기능:**

```
pole_graphics (3D):
  ✅ PBR rendering (고품질)
  ✅ Real-time lighting (dynamic)
  ✅ Shadow mapping (cascaded)
  ✅ Post-processing (HDR, tone mapping)
  ✅ LOD system (성능 최적화)

pole_animation:
  ✅ Skeletal animation
  ✅ Animation blending (smooth transitions)
  ✅ Inverse kinematics (IK)
  ✅ Animation state machine

pole_physics (3D):
  ✅ Character controller (slope handling)
  ✅ Ragdoll physics
  ✅ Physics-based combat

pole_ui:
  ✅ Inventory system
  ✅ Character stats screen
  ✅ Dialogue system
  ✅ Quest log
  ✅ Minimap

pole_ecs:
  ✅ Component-based entities
  ✅ Performance (1000+ entities)

pole_assets:
  ✅ Async streaming (open world)
  ✅ Asset bundles
```

**선택 기능:**

```
pole_graphics (고급):
  ⚠️ Ray tracing (선택)
  ⚠️ Global illumination

pole_ai:
  ⚠️ Advanced behavior trees
  ⚠️ Dynamic pathfinding

pole_cutscene:
  ⚠️ Cutscene editor
  ⚠️ Camera sequencer
```

**엔진 성숙도:**
- **최소**: Pole Engine 베타 후반
- **권장**: Pole Engine 1.0

### 📅 개발 일정 (18-24개월)

#### Month 1-3: MVP
**목표**: 전투 메커니즘 프로토타입

**작업:**
- [ ] 플레이어 이동 및 전투
- [ ] 1-2종 무기 (검, 방패)
- [ ] 기본 적 1종 (전투 테스트)
- [ ] 테스트 아레나

**Pole 예시:**
```pole
type CombatSystem:
  purpose: Action combat with stamina management
  
  components:
    - health: int
    - stamina: int
    - is_blocking: bool
    - is_dodging: bool
  
  methods:
    - attack(weapon: Weapon) -> AttackResult
    - block() -> bool
    - dodge() -> bool
    - take_damage(amount: int, can_block: bool) -> DamageResult
  
  requires:
    - stamina >= action_cost
  
  ensures:
    - stamina decreases on action
    - stamina regenerates over time
    - can't attack while blocking
```

#### Month 4-12: 반복 개발
**주요 시스템 구축:**

**Month 4-6: 전투 시스템**
- [ ] 다양한 무기 (검, 도끼, 활, 마법)
- [ ] 스킬 트리
- [ ] 적 AI (3-5종)
- [ ] 보스 전투 (1-2개)

**Month 7-9: RPG 시스템**
- [ ] 인벤토리 및 아이템
- [ ] 캐릭터 커스터마이징
- [ ] 레벨업 시스템
- [ ] 장비 강화

**Month 10-12: 월드 및 퀘스트**
- [ ] 오픈월드 또는 허브 구조
- [ ] 주요 퀘스트 라인
- [ ] NPC 및 대화 시스템
- [ ] 세이브/로드 시스템

#### Month 13-18: Early Access
**콘텐츠 확장:**
- [ ] 전체 월드 (3-5개 지역)
- [ ] 메인 스토리 (70% 완성)
- [ ] 사이드 퀘스트 10-20개
- [ ] 최종 그래픽 및 애니메이션

**완료 기준:**
- 20-30시간 플레이 타임
- 60 FPS (1080p)
- Early Access 출시

#### Month 19-24: 정식 출시
**완성도 극대화:**
- [ ] 엔딩 콘텐츠
- [ ] 뉴게임+ 모드
- [ ] 숨겨진 보스/아이템
- [ ] 컷씬 및 시네마틱
- [ ] 최종 밸런스 조정
- [ ] 콘솔 포팅 (선택)

### 💰 예산 및 수익 목표

**개발 예산:**
- MVP: $20K
- 반복 개발: $100K (크라우드펀딩 + 퍼블리셔)
- 총 예산: $150K-300K

**수익 목표:**

| 단계 | 판매량 | 가격 | 수익 |
|------|--------|------|------|
| Early Access | 10,000-20,000 | $30 | $300K-600K |
| 정식 출시 | 50,000-200,000 | $40 | $2M-8M |
| 1년 후 (DLC 포함) | 100,000-500,000 | $35 평균 | $3.5M-17.5M |

**목표:**
- 최소: 손익분기 ($150K)
- 중간: 5배 수익 ($750K-1.5M)
- 최대: 20배 수익 ($3M-6M)

---

## 2.4 멀티플레이어 게임

### 📋 프로젝트 개요

**장르**: 온라인 멀티플레이어 (예: Valorant, Overwatch 스타일)  
**타겟 플랫폼**: PC (Steam, Epic), 콘솔  
**개발 기간**: 2-3년  
**팀 규모**: 5-15명  

### 👥 팀 구성

**핵심 팀 (5명):**
- **리드 프로그래머 1명**: 아키텍처
- **네트워크 프로그래머 1명**: 서버/클라이언트
- **게임플레이 프로그래머 1명**: 게임 로직
- **3D 아티스트 1명**: 캐릭터, 맵
- **게임 디자이너 1명**: 밸런스, 레벨

**확장 팀 (15명):**
- 코어 프로그래머 3명
- 네트워크/백엔드 2명
- 게임플레이 2명
- 아티스트 3명
- 디자이너 2명
- UI/UX 1명
- 사운드 1명
- QA 2명
- 커뮤니티 매니저 1명

### 🛠️ Pole Engine 기능 요구사항

**필수 기능:**

```
pole_net:
  ✅ Client-server architecture
  ✅ State synchronization
  ✅ Lag compensation (client prediction, server reconciliation)
  ✅ Interpolation/extrapolation
  ✅ Matchmaking system
  ✅ Server browser
  ✅ Anti-cheat (basic)

pole_graphics (3D):
  ✅ 3D FPS 렌더링 (2.2 참조)
  ✅ 성능 최적화 (120 FPS 목표)

pole_physics (3D):
  ✅ Deterministic physics (네트워크 동기화)
  ✅ Hit registration (server-authoritative)

pole_ui:
  ✅ 스코어보드
  ✅ 채팅 시스템
  ✅ 팀 관리

pole_ecs:
  ✅ 네트워크 최적화된 ECS
  ✅ Replicated components
```

**백엔드 (Pole로 구현):**

```
pole_server:
  ✅ Game server (dedicated)
  ✅ Matchmaking service
  ✅ Account system
  ✅ Leaderboard
  ✅ Analytics
```

**엔진 성숙도:**
- **최소**: Pole Engine 1.0 (네트워킹 안정화)
- **권장**: Pole Engine 1.0 + 실전 검증

### 📅 개발 일정 (24-36개월)

#### Month 1-6: MVP (네트워크 프로토타입)
**목표**: 멀티플레이어 핵심 증명

**작업:**
- [ ] 클라이언트-서버 기본 구조
- [ ] 플레이어 이동 동기화
- [ ] 기본 무기 (hitscan)
- [ ] 2-4명 테스트 맵
- [ ] 로컬 네트워크 테스트

**Pole 예시:**
```pole
@networked
type PlayerState:
  fields:
    - position: Vector3 @replicated
    - rotation: Quaternion @replicated
    - health: int @replicated
    - velocity: Vector3 @client_predicted
  
  methods:
    @server_rpc
    - move(input: InputState)
    
    @client_rpc
    - take_damage(amount: int)
    
    @client_predicted
    - apply_movement(delta: float)

function lag_compensation:
  purpose: Server-side hit detection with rewind
  input:
    - shooter: PlayerId
    - target: PlayerId
    - shoot_time: timestamp
  output: bool (hit or miss)
  
  implementation:
    - Rewind target to shooter's perceived time
    - Perform raycast
    - Validate hit (anti-cheat)
    - Apply damage if valid
```

#### Month 7-18: 반복 개발
**주요 시스템:**

**Month 7-12: 게임플레이**
- [ ] 3-5종 무기
- [ ] 캐릭터 클래스 또는 영웅 (3-5개)
- [ ] 게임 모드 (팀 데스매치, 점령전)
- [ ] 3-5개 맵

**Month 13-18: 백엔드 및 매칭**
- [ ] 매치메이킹 시스템
- [ ] 계정 시스템 (로그인, 프로필)
- [ ] 랭킹 시스템
- [ ] 리플레이 시스템
- [ ] 안티치트 강화

#### Month 19-24: 클로즈 베타
**목표**: 실사용자 테스트

**작업:**
- [ ] 500-1,000명 초대 테스트
- [ ] 서버 부하 테스트
- [ ] 밸런스 조정
- [ ] 버그 수정
- [ ] 추가 콘텐츠 (맵, 캐릭터)

**완료 기준:**
- 안정적 서버 (99% 업타임)
- 매치메이킹 < 1분
- 120 FPS (경쟁 게임 표준)

#### Month 25-30: 오픈 베타 & 정식 출시
**F2P 모델:**
- [ ] 무료 배포 (모든 핵심 콘텐츠)
- [ ] 배틀 패스 시스템
- [ ] 스킨/코스메틱 상점
- [ ] 시즌 시스템 (3개월 주기)

**런치:**
- [ ] 마케팅 (스트리머, 이스포츠)
- [ ] 서버 인프라 확장
- [ ] 24/7 모니터링

#### Month 31-36+: 라이브 서비스
**지속 운영:**
- [ ] 시즌 업데이트 (신 캐릭터, 맵)
- [ ] 밸런스 패치 (2주마다)
- [ ] 이벤트 및 토너먼트
- [ ] 커뮤니티 관리

### 💰 예산 및 수익 목표

**개발 예산:**
- MVP: $30K
- 반복 개발: $200K
- 베타 운영: $100K
- 총 예산: $300K-500K

**수익 모델 (F2P):**

| 항목 | 가격 | 예상 전환율 |
|------|------|-------------|
| 배틀 패스 | $10/시즌 | 10-20% 플레이어 |
| 스킨 | $5-20 | 5-10% 플레이어 |
| 캐릭터 (선택) | $10 | 3-5% 플레이어 |

**수익 목표:**

| 시기 | DAU | ARPU | 월 수익 |
|------|-----|------|---------|
| 런치 (Month 1) | 10K-50K | $2-5 | $20K-250K |
| 6개월 후 | 50K-200K | $3-6 | $150K-1.2M |
| 1년 후 | 100K-500K | $4-8 | $400K-4M |

**목표:**
- 최소: 손익분기 ($300K, 6개월 내)
- 중간: 연 $2M-5M
- 최대: 연 $10M+ (이스포츠 성공 시)

---

# 부록: 리소스 및 팁

## A. Pole 언어 학습 자료

**필수 문서:**
- [README.md](README.md) - Pole 언어 소개
- [specs/syntax-v0.md](specs/syntax-v0.md) - 명세 언어 문법
- [specs/ir-syntax.md](specs/ir-syntax.md) - IR 문법
- [DEVELOPMENT.md](DEVELOPMENT.md) - 개발 환경 설정

**예제 코드:**
- `examples/*.pole` - 명세 언어 예제
- `examples/*.pole-ir` - IR 예제

## B. Pole Engine 문서

**공식 문서** (GAME_ENGINE_VISION.md 참조)
- API 레퍼런스
- 튜토리얼
- 베스트 프랙티스

## C. 게임 개발 리소스

**무료 에셋:**
- OpenGameArt.org - 2D/3D 에셋
- Freesound.org - 사운드 이펙트
- Incompetech.com - 무료 음악 (Kevin MacLeod)

**유료 에셋:**
- Unity Asset Store (Pole Engine 호환 모델)
- itch.io Marketplace

**학습 자료:**
- Game Programming Patterns (책)
- Gamasutra / Game Developer (블로그)
- GDC Vault (강연)

## D. 커뮤니티

**Pole 커뮤니티:**
- GitHub Discussions
- Discord 서버
- Reddit r/pole (가상)

**게임 개발 커뮤니티:**
- r/gamedev
- IndieDB
- TIGSource

---

# 요약

이 로드맵은 **Pole Engine으로 게임을 개발하는 실전 가이드**입니다.

**핵심 포인트:**

1. **애자일 방식**: MVP → 반복 개발 → Early Access → 정식 출시 → 라이브 서비스
2. **Pole 100% 활용**: 게임 로직, 엔진 확장, 쉐이더 모두 Pole 언어
3. **장르별 맞춤**: 2D 플랫포머 (6개월) → 멀티플레이어 (2-3년)
4. **실용적 목표**: 최소(손익분기) / 중간(다음 프로젝트) / 최대(스튜디오 성장)

**3개 로드맵 관계:**

```
ROADMAP.md (Pole 언어 개발)
  ↓
GAME_ENGINE_VISION.md (Pole Engine 프로젝트)
  ↓
GAME_DEVELOPMENT_ROADMAP.md (Pole Engine으로 게임 개발) ← 현재 문서
```

**다음 단계:**
1. 장르 선택 (2.1 ~ 2.4)
2. 팀 구성
3. Pole Engine 준비 확인
4. MVP 시작!

---

**Pole로 게임을 만들고, 세상에 공개하세요!** 🎮✨
