# Pole Engine 상세 로드맵

> 2D 게임 엔진 모듈 개발 계획

**최종 업데이트:** 2025-10-20

---

## 현재 상태

### 보유 자산
- **72개 예제 코드** (타일맵, 좀비 AI, 네트워킹 등)
- **검증된 패턴** (SDL2 렌더링, 충돌 감지, 경로 찾기)
- **초기 모듈 구조** (pole_engine/ 디렉토리)

### 필요 작업
- 게임 코드 → 엔진 모듈 추출
- 명세 작성 & LLM 생성
- 문서화 & 테스트

---

## 단기 로드맵 (3개월)

### Week 1-4: Core 모듈 (4개)

#### core/types.pole
**기능:**
- Position, Direction 기본 타입
- AABB (충돌 박스)
- Color (RGB)

**예제:**
```pole
type Position = { x: Int, y: Int }
type Direction = North | South | East | West
type AABB = { x: Int, y: Int, width: Int, height: Int }
```

**추출 출처:** examples/30-camera-simple.pole-ir

#### core/math.pole
**기능:**
- manhattan_distance
- clamp
- lerp (선형 보간)

**예제:**
```pole
function manhattan_distance(pos1: Position, pos2: Position) -> Int
function clamp(value: Int, min: Int, max: Int) -> Int
function lerp(a: Float64, b: Float64, t: Float64) -> Float64
```

**추출 출처:** examples/33-point-distance.pole, examples/50-zombie-chase.pole-ir

#### core/time.pole
**기능:**
- FPS 계산
- Delta time 관리
- 타이머

**예제:**
```pole
function get_delta_time() -> Float64
function calculate_fps(frame_count: Int, elapsed: Float64) -> Int
```

#### core/random.pole
**기능:**
- 난수 생성
- 범위 난수
- 확률 체크

**예제:**
```pole
function random_int(min: Int, max: Int) -> Int
function random_float() -> Float64
function chance(probability: Float64) -> Bool  // 0.0 ~ 1.0
```

### Week 5-8: Render 모듈 (5개)

#### render/window.pole
**기능:**
- SDL2 윈도우 생성/파괴
- 이벤트 처리 래퍼

**추출 출처:** examples/24-sdl2-window.pole-ir

#### render/sprite.pole
**기능:**
- 스프라이트 렌더링
- 색상 지정
- 회전/스케일

**추출 출처:** examples/39-texture-demo.pole-ir

#### render/tilemap.pole
**기능:**
- 타일맵 렌더링
- 카메라 기준 렌더링
- 레이어 지원

**추출 출처:** examples/48-list-tilemap-final.pole-ir

#### render/camera.pole
**기능:**
- 카메라 이동
- 부드러운 추적
- 경계 제한

**추출 출처:** examples/31-interactive-camera.pole-ir

#### render/text.pole
**기능:**
- 텍스트 렌더링
- 폰트 지원
- UI 텍스트

### Week 9-12: Input 모듈 (3개)

#### input/keyboard.pole
**기능:**
- 키 눌림 감지
- 키 상태 추적
- 키 매핑

**추출 출처:** examples/45-keyboard-movement.pole-ir

#### input/mouse.pole
**기능:**
- 마우스 위치
- 클릭 감지
- 타일 호버

**추출 출처:** examples/38-mouse-hover.pole-ir

#### input/gamepad.pole
**기능:**
- 게임패드 입력
- 버튼 매핑
- 아날로그 스틱

---

## 중기 로드맵 (6개월)

### Month 4-5: Physics 모듈 (4개)

#### physics/collision.pole
**기능:**
- AABB vs AABB 충돌
- 점 vs 박스 충돌
- 타일맵 충돌

**추출 출처:** examples/32-tile-collision.pole-ir

#### physics/movement.pole
**기능:**
- 속도 기반 이동
- 가속도
- 마찰력

#### physics/rigidbody.pole
**기능:**
- 물리 시뮬레이션
- 중력
- 충격량

#### physics/quadtree.pole
**기능:**
- 공간 분할
- 효율적 충돌 감지
- 100+ 엔티티 지원

### Month 6: AI 모듈 (3개)

#### ai/pathfinding.pole
**기능:**
- A* 알고리즘
- 타일맵 경로 찾기
- 동적 장애물 회피

**추출 출처:** examples/50-zombie-chase.pole-ir

#### ai/behavior.pole
**기능:**
- FSM (Finite State Machine)
- Behavior Tree 기초
- AI 상태 관리

**추출 출처:** examples/51-multiple-zombies.pole-ir

#### ai/flock.pole
**기능:**
- 무리 행동
- 분리/정렬/응집
- 좀비 떼 시뮬레이션

---

## 장기 로드맵 (1-2년)

### Year 1: 완전한 2D 엔진

#### network/ 모듈 (4개)
- tcp.pole - TCP 소켓
- sync.pole - 게임 상태 동기화
- lobby.pole - 로비 시스템
- p2p.pole - P2P 연결

**추출 출처:** examples/59-coop-server.pole-ir, examples/60-coop-client.pole-ir

#### audio/ 모듈 (3개)
- sound.pole - 효과음
- music.pole - 배경 음악
- mixer.pole - 오디오 믹싱

#### ui/ 모듈 (5개)
- button.pole - 버튼
- panel.pole - UI 패널
- inventory_ui.pole - 인벤토리 UI
- health_bar.pole - 체력바
- menu.pole - 메뉴 시스템

#### animation/ 모듈 (2개)
- sprite_animation.pole - 스프라이트 애니메이션
- tween.pole - 보간 애니메이션

#### save/ 모듈 (2개)
- save_game.pole - 게임 저장
- load_game.pole - 게임 로드

**추출 출처:** examples/52-file-io-complete.pole-ir

### Year 2: 프로덕션 레디

#### editor/ 모듈 (4개)
- level_editor.pole - 레벨 에디터
- tilemap_editor.pole - 타일맵 에디터
- entity_editor.pole - 엔티티 배치
- export.pole - 데이터 내보내기

#### particle/ 모듈 (2개)
- particle_system.pole - 파티클 시스템
- effects.pole - 시각 효과

#### lighting/ 모듈 (2개)
- light.pole - 조명 시스템
- shadow.pole - 그림자

---

## 모듈 개발 프로세스

### 1. 게임 코드 분석 (목요일)
```bash
# 이번 주 작성된 게임 코드 확인
ls games/zomboid/specs/*.pole

# 재사용 가능한 함수 식별
# 예: distance() → manhattan_distance()
```

### 2. 엔진 명세 작성 (목요일)
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
    - "교환법칙: distance(a,b) == distance(b,a)"
  
  examples:
    - manhattan_distance({x=0, y=0}, {x=3, y=4}) → 7
    - manhattan_distance({x=5, y=5}, {x=5, y=5}) → 0
    
  performance: O(1)
  use_cases:
    - "AI 경로 찾기"
    - "충돌 감지 전처리"
    - "거리 기반 게임 로직"
```

### 3. LLM 생성 (금요일)
```bash
pole build pole_engine/core/math.pole
```

### 4. 테스트 작성 (금요일)
```pole
// examples/XX-test-math.pole-ir

def test_manhattan_distance() -> Int =
  let d1 = manhattan_distance({x=0, y=0}, {x=3, y=4}) in
  let d2 = manhattan_distance({x=5, y=5}, {x=5, y=5}) in
  if d1 == 7 && d2 == 0 then 0 else 1
```

### 5. 문서화 (금요일)
```markdown
# pole_engine/core/README.md

## math.pole

수학 유틸리티 함수

### Functions

#### manhattan_distance
두 점 사이의 맨해튼 거리를 계산합니다.

**사용 예:**
```pole
let dist = manhattan_distance(player.pos, zombie.pos)
if dist < 10 then
  // 좀비가 가까움
```
```

---

## 성능 목표

### 렌더링
- **목표:** 60 FPS @ 1920x1080
- **타일맵:** 100x100 맵, 60 FPS
- **스프라이트:** 1000개 동시, 60 FPS

### 물리
- **목표:** 100+ 엔티티
- **충돌 감지:** Quadtree로 O(n log n)
- **경로 찾기:** A* 10개 동시

### 네트워크
- **목표:** 4인 Co-op, < 50ms 지연
- **동기화:** 60Hz tick rate
- **대역폭:** < 100KB/s per client

---

## 우선순위

### P0 (필수, 3개월)
- [x] core/types.pole
- [ ] core/math.pole
- [ ] render/sprite.pole
- [ ] render/tilemap.pole
- [ ] input/keyboard.pole

### P1 (중요, 6개월)
- [ ] physics/collision.pole
- [ ] ai/pathfinding.pole
- [ ] camera.pole
- [ ] mouse.pole
- [ ] time.pole

### P2 (유용, 1년)
- [ ] network/tcp.pole
- [ ] audio/sound.pole
- [ ] ui/button.pole
- [ ] animation/sprite_animation.pole
- [ ] save/save_game.pole

### P3 (선택, 2년)
- [ ] editor/level_editor.pole
- [ ] particle/particle_system.pole
- [ ] lighting/light.pole

---

## 품질 기준

### 모든 모듈은 다음을 만족해야 함:

1. **명세 완성도**
   - [ ] purpose 명확
   - [ ] examples 3개 이상
   - [ ] constraints 명시
   - [ ] use_cases 설명

2. **테스트**
   - [ ] 유닛 테스트 작성
   - [ ] 통합 테스트 (게임에서 사용)
   - [ ] 성능 테스트

3. **문서화**
   - [ ] 모듈 README
   - [ ] 함수별 설명
   - [ ] 사용 예제

4. **성능**
   - [ ] 60 FPS 유지
   - [ ] 메모리 누수 없음
   - [ ] 프로파일링 완료

---

## 마일스톤

### M1: Core 모듈 완성 (Month 1)
- 4개 모듈 (types, math, time, random)
- 게임에서 사용 검증
- 문서 완성

### M2: Render + Input 완성 (Month 3)
- 8개 모듈 (window, sprite, tilemap, camera, text, keyboard, mouse, gamepad)
- 데모 게임 제작
- YouTube 시연

### M3: Physics + AI 완성 (Month 6)
- 7개 모듈 (collision, movement, rigidbody, quadtree, pathfinding, behavior, flock)
- 100 좀비 데모
- 성능 벤치마크

### M4: Pole Engine 1.0 (Year 1)
- 15개 모듈 완성
- 다른 게임 제작 가능
- 문서 사이트 오픈

---

## 관련 문서

- [엔진 개발 가이드](../guides/ENGINE_DEV.md)
- [Pole Engine README](../../pole_engine/README.md)
- [게임 개발 가이드](../guides/GAME_DEV.md)
- [주간 계획](WEEKLY_PLANS.md)

---

**목표:** 2년 내 프로덕션 레디 2D 엔진
