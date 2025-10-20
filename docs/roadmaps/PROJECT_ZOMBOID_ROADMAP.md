# 🧟 Project Zomboid Clone 개발 로드맵

> **최종 목표**: 3년 내 Project Zomboid 스타일 생존 게임 출시
> 
> **전략**: 역산 계획(Backward Planning) - PZ Clone에서 시작해 필요한 기능만 개발

**작성일**: 2025-10-19  
**목표 출시**: 2028년 상반기  
**개발 방식**: PZ-First Development (Project Zomboid 우선 개발)

---

## 📊 전체 개요

### 핵심 원칙

1. **"Project Zomboid에 필요한가?"** 가 모든 결정 기준
2. 범용 엔진 ❌ → PZ 전용 엔진 ✅
3. 불필요한 기능은 과감히 제거
4. 매달 플레이 가능한 데모 제작

### 3년 타임라인

```
Year 1 (2025-2026): Pole 언어 PZ 기능 완성
Year 2 (2026-2027): PZ 전용 엔진 개발
Year 3 (2027-2028): PZ Clone 개발 및 출시
```

---

## 🎯 역산 요구사항 분석

### Level 3: Project Zomboid Clone 필수 기능

```markdown
게임 핵심:
✅ 아이소메트릭 타일 렌더링 (2D)
✅ 대규모 좀비 시뮬레이션 (1000+ 엔티티)
✅ 인벤토리/제작 시스템
✅ 건축/바리케이드
✅ 생존 시스템 (배고픔, 갈증, 피로)
✅ 세이브/로드
✅ 멀티플레이어 (2-8명 Co-op 필수)
```

### Level 2: Pole Engine 최소 요구사항

```markdown
렌더링:
✅ SDL2/OpenGL 2D 타일맵
✅ 아이소메트릭 투영
✅ 스프라이트 배칭
✅ 카메라 시스템

게임 시스템:
✅ 간단한 ECS (1000+ 엔티티)
✅ 그리드 기반 충돌
✅ A* 경로 찾기
✅ UI 시스템 (인벤토리)

데이터:
✅ 파일 I/O (세이브/로드)
✅ JSON 파싱
✅ 에셋 로딩

네트워킹:
✅ Client-Server 아키텍처
✅ 상태 동기화 (플레이어, 좀비, 아이템)
✅ 호스트 마이그레이션
✅ 2-8명 동시 접속
```

### Level 1: Pole Language 최소 요구사항

```markdown
완료 (Phase 5-6):
✅ 네이티브 컴파일 (LLVM)
✅ FFI (SDL2/OpenGL)
✅ 기본 타입 시스템

필요 (우선순위):
⚠️ 파일 시스템 접근
⚠️ 동적 메모리 관리
⚠️ 컬렉션 (Array, HashMap)
⚠️ JSON 파싱
⚠️ 네트워킹 (TCP/UDP 소켓)
⚠️ **동시성 (스레드, async/await)** ⭐ PZ 필수
⚠️ **병렬 처리 (좀비 AI)** ⭐ PZ 필수
⚠️ **메모리 최적화 (pooling, arena)** ⭐ PZ 필수
❌ 3D 그래픽 (불필요)
❌ 고급 물리 (불필요)
```

---

## 📅 연도별 개발 계획

## Year 1 (2025-2026): Pole 언어 PZ 기능 완성

### Q1 (2025-10 ~ 2025-12) ✅ 일부 완료
- [x] LLVM 네이티브 컴파일
- [x] SDL2 FFI 통합
- [x] **동적 배열/HashMap** (완료!)
  - List_push, List_length 구현
  - HashMap_new, HashMap_put, HashMap_get 구현
  - 100 zombies @ 60 FPS 검증
- [ ] **파일 I/O 시스템** (다음 우선순위!)

### Q2 (2026-01 ~ 2026-03): 핵심 자료구조
- [ ] 벡터/리스트 완전 구현
- [ ] HashMap/Dictionary
- [ ] 문자열 조작 라이브러리
- [ ] JSON 파서/생성기

### Q3 (2026-04 ~ 2026-06): 동시성 & 성능 최적화 ⭐ PZ 필수
- [ ] **스레드 지원** (std::thread FFI)
- [ ] **비동기 I/O** (네트워크 & 파일)
- [ ] **병렬 처리** (좀비 AI 병렬화)
- [ ] **TCP/UDP 소켓 FFI**
- [ ] **Arena allocator** (프레임 메모리)
- [ ] **Object pooling** (좀비/아이템 재활용)
- [ ] **공간 분할** (QuadTree/Grid)
- [ ] 최적화 목표: 1000+ 좀비 @ 60 FPS

### Q4 (2026-07 ~ 2026-09): 2D 게임 & 네트워크 검증
- [x] **아이소메트릭 렌더링 데모** (조기 완료!)
- [x] 100x100 타일맵 (조기 완료!)
- [x] 좀비 100마리 시뮬레이션 (조기 완료!)
- [ ] 기본 UI (인벤토리 그리드)
- [ ] **2인 네트워크 테스트** (LAN)

**Year 1 산출물**: 
- Pole로 작성한 아이소메트릭 타일 엔진
- 좀비 100마리 동시 시뮬레이션 데모
- 2인 협동 프로토타입 (LAN)

---

## Year 2 (2026-2027): PZ 전용 엔진 개발

### Q1 (2026-10 ~ 2026-12): 코어 시스템
- [ ] 타일맵 시스템 (청크 로딩)
- [ ] 아이소메트릭 카메라
- [ ] 스프라이트 렌더러
- [ ] 기본 조명 시스템

### Q2 (2027-01 ~ 2027-03): 게임플레이 시스템
- [ ] 간단한 ECS 구현
- [ ] 좀비 AI (시각/청각)
- [ ] 경로 찾기 (A*)
- [ ] 충돌 감지 (그리드)

### Q3 (2027-04 ~ 2027-06): UI & 멀티플레이어
- [ ] 인벤토리 시스템
- [ ] 제작 시스템
- [ ] 컨텍스트 메뉴
- [ ] **Client-Server 아키텍처**
- [ ] **상태 동기화 (플레이어, 좀비)**

### Q4 (2027-07 ~ 2027-09): PZ 프로토타입
- [ ] **플레이 가능한 PZ 프로토타입**
- [ ] 1개 작은 도시
- [ ] 생존 루프 완성
- [ ] 1시간 플레이 콘텐츠
- [ ] **4인 Co-op 지원** (안정적)

**Year 2 산출물**:
- PZ 전용 게임 엔진 (최소 기능)
- 1시간 플레이 가능한 프로토타입
- 4인 협동 멀티플레이 작동

---

## Year 3 (2027-2028): Project Zomboid Clone 개발

### Q1 (2027-10 ~ 2027-12): 핵심 게임플레이
- [ ] 전체 생존 시스템
- [ ] 스킬 시스템
- [ ] 다양한 좀비 타입
- [ ] 무기/도구 시스템

### Q2 (2028-01 ~ 2028-03): 콘텐츠 확장
- [ ] 3-5개 도시
- [ ] 건축 시스템
- [ ] 농사/낚시
- [ ] 날씨/계절

### Q3 (2028-04 ~ 2028-06): Early Access
- [ ] **Steam Early Access 출시**
- [ ] 10시간 플레이 콘텐츠
- [ ] 세이브/로드 완성
- [ ] **8인 멀티플레이어 지원**
- [ ] **전용 서버 도구**
- [ ] 버그 수정 & 밸런싱

### Q4 (2028-07 ~ 2028-09): 정식 출시
- [ ] **정식 출시**
- [ ] 20-30시간 콘텐츠
- [ ] **안정적인 8인 멀티플레이어**
- [ ] **PvP/PvE 서버 모드**
- [ ] **보이스 채팅 (근거리)**
- [ ] 모딩 지원 (선택)

**Year 3 산출물**:
- Steam 출시된 Project Zomboid Clone
- 목표: 10,000 판매, $200K 수익

---

## 🚀 즉시 실행 계획

### Week 1 (2025-10-20 ~ 2025-10-26): 아이소메트릭 PoC

```markdown
Day 1-2: 컴파일러 버그 수정 & 기본 렌더링 🔥 최우선
- [ ] Unit 반환 타입 LLVM 버그 수정 (codegen.rs)
- [ ] 27-isometric-simple.pole-ir 컴파일 성공
- [ ] 3x3 아이소메트릭 그리드 SDL2 렌더링 확인
- [ ] 스크린샷 캡처 및 검증
- [ ] (선택) For 루프 구문 구현

Day 3-4: SDL2 이벤트 폴링 & 입력 처리
- [ ] SDL_PollEvent FFI 바인딩
- [ ] 키보드 입력 감지 (WASD, ESC)
- [ ] 마우스 입력 감지 (위치, 클릭)
- [ ] 이벤트 루프 구현

Day 5-6: 카메라 컨트롤
- [ ] WASD 카메라 이동
- [ ] 마우스 휠 줌 (또는 +/- 키)
- [ ] 카메라 경계 제한
- [ ] 타일 하이라이트 (마우스 호버)

Day 7: 데모 & 공유
- [ ] 10x10 타일로 확장
- [ ] 데모 비디오 녹화 (OBS)
- [ ] YouTube 업로드
- [ ] Reddit r/projectzomboid 공유
- [ ] 피드백 수집
```

### Month 1 (2025-11): 타일맵 시스템

```rust
// 목표: 100x100 아이소메트릭 타일맵

type TileMap = {
  tiles: Array<Array<Tile>>,
  width: Int,
  height: Int
}

function render_tilemap(map: TileMap, camera: Camera):
  for y in 0..map.height:
    for x in 0..map.width:
      let screen_pos = iso_to_screen(x, y, camera)
      render_tile(map.tiles[y][x], screen_pos)
```

### Month 2-3 (2025-12 ~ 2026-01): 좀비 시뮬레이션 & 네트워크 테스트 ✅

```rust
// 목표: 100 좀비 동시 처리 + 네트워크 동기화

type Zombie = {
  id: Int,  // 네트워크 ID
  position: Vec2,
  target: Option<Vec2>,
  state: ZombieState,
  sight_range: Float,
  hearing_range: Float,
  last_sync: Float  // 동기화 타임스탬프
}

function update_zombies(zombies: Array<Zombie>, players: Array<Player>, delta: Float, is_server: Bool):
  if is_server:
    for zombie in zombies:
      // 서버: 모든 플레이어에 대해 AI 실행
      for player in players:
        if can_see_player(zombie, player):
          zombie.target = Some(player.position)
      
      move_toward_target(zombie, delta)
      update_zombie_state(zombie)
      
      // 네트워크 동기화 (100ms마다)
      if time_since(zombie.last_sync) > 0.1:
        broadcast_zombie_state(zombie)
  else:
    // 클라이언트: 서버 상태 수신 및 보간
    interpolate_zombie_positions(zombies, delta)
```

**완료 현황 (2025-10-20)**:
- ✅ 100 zombies 동시 처리 구현 (`52-hundred-zombies.pole-ir`)
- ✅ HashMap 기반 entity storage 
- ✅ Greedy chase AI (단순 추적 알고리즘)
- ✅ 60 FPS @ 100 zombies 달성 (600 frames, 10초)
- ✅ 50x50 isometric grid rendering with viewport culling
- ⏳ 네트워크 동기화 (미완성 - Q1 2025 우선순위로 연기)

**산출물**:
- `49-player-entity.pole-ir` - Player entity with camera follow
- `50-zombie-chase.pole-ir` - Single zombie chase AI
- `51-multiple-zombies.pole-ir` - 10 zombies HashMap demo
- `52-hundred-zombies.pole-ir` - 100 zombies final milestone

---

## 📈 단계별 데모 목표

### Demo 1 (3개월): "Walking Simulator"
- 아이소메트릭 10x10 도시
- 플레이어 이동
- 건물 진입/탈출
- 카메라 컨트롤

### Demo 2 (6개월): "Zombie Encounter"  
- 좀비 100마리
- 근접 전투
- 체력/피해 시스템
- 아이템 줍기/드롭

### Demo 3 (9개월): "Survival Loop"
- 배고픔/갈증/피로
- 루팅 시스템
- 간단한 제작
- Day/Night 사이클
- **2인 Co-op (LAN)**

### Demo 4 (12개월): "Early Access Ready"
- 1개 도시 완성
- 5시간 플레이 콘텐츠
- 세이브/로드
- **4인 Co-op (온라인)**
- Steam 페이지 준비

### Demo 5 (18개월): "Beta"
- 3개 도시
- 10시간 콘텐츠
- 전체 제작 시스템
- 건축 기능
- **8인 멀티플레이어**
- **전용 서버 지원**

### Demo 6 (24개월): "Release Candidate"
- 5개 도시
- 20시간 콘텐츠
- 모든 핵심 기능
- 폴리싱 완료

---

## 🎮 기능 우선순위

### P0 (필수 - PZ 핵심)
```markdown
렌더링:
✅ 아이소메트릭 타일맵
✅ 스프라이트 렌더링
✅ Y-sorting
✅ 카메라 시스템

게임플레이:
✅ 좀비 AI (시각/청각)
✅ 근접/원거리 전투
✅ 인벤토리 관리
✅ 제작 시스템
✅ 생존 욕구

시스템:
✅ 세이브/로드
✅ 설정 메뉴
✅ 기본 UI

멀티플레이어:
✅ Client-Server 아키텍처
✅ 2-8인 동시 접속
✅ 상태 동기화
✅ 전용 서버
```

### P1 (중요 - 게임 깊이)
```markdown
✅ 건축 시스템
✅ 바리케이드
✅ 스킬 시스템
✅ 날씨/계절
✅ 농사/낚시
⚠️ NPC (선택)
```

### P2 (나중에 - 확장)
```markdown
⚠️ 32인+ 대규모 서버
⚠️ 모딩 지원
⚠️ 스토리 모드
⚠️ 콘솔 포팅
⚠️ 크로스플랫폼 플레이
```

### ❌ 제외 (PZ 불필요)
```markdown
❌ 3D 렌더링
❌ 고급 물리 엔진
❌ 절차적 생성
❌ VR 지원
❌ 클라우드 저장
```

---

## 👥 팀 구성

### 최소 팀 (2명)
```yaml
개발자 1 (Core):
  - Pole 언어 개발
  - 엔진 시스템
  - 게임 로직
  - 시간: 풀타임

아티스트 1 (Art):
  - 픽셀아트 타일셋
  - 캐릭터/좀비 스프라이트  
  - UI 에셋
  - 시간: 파트타임/외주
```

### 확장 팀 (5명) - Year 2
```yaml
위 2명 +
게임 디자이너 1:
  - 레벨 디자인
  - 밸런싱
  - 콘텐츠

QA 테스터 1:
  - 버그 테스트
  - 플레이 테스트
  - 피드백

사운드 디자이너 1:
  - 효과음
  - 앰비언트
  - 음악 (선택)
```

---

## 💰 예산 계획

### 개발 비용 (3년)
```markdown
Year 1: $50K
  - 개발자 급여: $40K
  - 에셋/도구: $5K
  - 인프라: $5K

Year 2: $100K  
  - 팀 2명: $80K
  - 에셋/외주: $10K
  - 마케팅: $10K

Year 3: $150K
  - 팀 3-5명: $120K
  - 마케팅: $20K
  - 출시 비용: $10K

총 예산: $300K
```

### 수익 목표
```markdown
Early Access (Year 3 Q3):
  - 가격: $15
  - 판매: 5,000
  - 수익: $75K

정식 출시 (Year 3 Q4):
  - 가격: $20
  - 판매: 10,000
  - 수익: $200K

1년 후:
  - 누적 판매: 30,000
  - 총 수익: $500K+
```

---

## ⚠️ 리스크 관리

### 기술적 리스크

#### 1. 성능 (1000+ 좀비)
- **리스크**: 낮은 FPS, 메모리 부족
- **완화**: 
  - **병렬 처리** (4+ 코어 활용)
  - **공간 분할** (QuadTree/Grid)
  - **Object pooling** (좀비 재활용)
  - **LOD 시스템** (거리별 디테일)
  - **비활성화** (화면 밖 좀비)
  - **Arena allocator** (프레임 메모리)

#### 2. 메모리 (거대한 맵)
- **리스크**: RAM 초과, 로딩 시간
- **완화**:
  - 청크 기반 로딩
  - 스트리밍 시스템
  - 압축 저장

#### 3. 복잡도 (많은 시스템)
- **리스크**: 버그, 밸런스 문제
- **완화**:
  - 점진적 구현
  - 철저한 테스트
  - Early Access 피드백

### 사업적 리스크

#### 1. PZ와 직접 비교
- **리스크**: "싸구려 카피" 인식
- **완화**:
  - 독창적 요소 추가
  - "영감받은" 포지셔닝
  - 커뮤니티 투명성

#### 2. 개발 지연
- **리스크**: 3년 초과
- **완화**:
  - 매달 데모 검증
  - 범위 조정 가능
  - Early Access 활용

#### 3. 1인 개발 한계
- **리스크**: 번아웃, 품질
- **완화**:
  - 조기 팀 구성
  - 에셋 구매/외주
  - 커뮤니티 참여

---

## ✅ 성공 지표 (KPI)

### 3개월
- [ ] 아이소메트릭 렌더링 작동
- [ ] 100x100 타일맵
- [ ] YouTube 조회수 1000+
- [ ] Discord 멤버 50+

### 6개월  
- [ ] 좀비 100마리 시뮬레이션
- [ ] 기본 전투 시스템
- [ ] 플레이 가능한 데모
- [ ] 팔로워 500+

### 1년
- [ ] 1시간 플레이 콘텐츠
- [ ] 4인 Co-op 작동
- [ ] 프로토타입 공개
- [ ] 크라우드펀딩 $50K
- [ ] 위시리스트 1000+

### 2년
- [ ] 5시간 콘텐츠
- [ ] Early Access 준비
- [ ] 위시리스트 5000+
- [ ] 스트리머 관심

### 3년
- [ ] Steam 출시
- [ ] 8인 멀티플레이어
- [ ] 판매 10,000+
- [ ] 수익 $200K+
- [ ] 평점 80%+
- [ ] 활성 서버 100+

---

## 🔄 Pole 언어 로드맵 수정 사항

### 추가 필요 (PZ 필수)
```markdown
즉시 (1개월):
- [ ] 파일 I/O 시스템
- [ ] 동적 배열 완성
- [ ] HashMap 구현

단기 (3개월):
- [ ] JSON 파서
- [ ] 2D 벡터 연산
- [ ] 문자열 조작 확장
- [ ] **TCP/UDP 소켓 FFI**
- [ ] **스레드 기본 지원**

중기 (6개월):
- [ ] **병렬 처리 시스템** (좀비 AI)
- [ ] **스레드 풀** (워커 스레드)
- [ ] **Lock-free 자료구조** (성능)
- [ ] **메모리 풀링** (object pool)
- [ ] **공간 분할** (QuadTree/Grid)
- [ ] **경로 찾기** (A* 병렬화)
- [ ] **비동기 I/O** (파일/네트워크)
- [ ] **네트워크 직렬화** (상태 동기화)
- [ ] **프레임 기반 Arena** (60 FPS)
```

### 연기/제거 (PZ 불필요)
```markdown
연기:
- ⏸ 3D 그래픽 라이브러리
- ⏸ 고급 물리 엔진
- ⏸ 모듈 시스템
- ⏸ 대규모 MMO 기능

제거:
- ❌ VR 지원
- ❌ 웹 어셈블리
- ❌ 모바일 타겟
```

---

## 📝 참고 자료

### Project Zomboid 분석
- 코어 메커니즘: 생존, 좀비, 제작, 건축
- 기술 스택: Java, LWJGL, 2D 아이소메트릭
- 성공 요인: 깊이, 리얼리즘, 모딩

### 필요 문서
- `docs/specs/isometric_rendering.md` (작성 예정)
- `docs/specs/zombie_ai.md` (작성 예정)
- `docs/specs/crafting_system.md` (작성 예정)

### 관련 로드맵
- [ROADMAP.md](../../ROADMAP.md) - Pole 언어 개발
- [game-engine-vision.md](game-engine-vision.md) - 장기 엔진 비전
- [project-zomboid-clone.md](examples/project-zomboid-clone.md) - 원본 계획

---

## 다음 단계

1. **이번 주**: 아이소메트릭 렌더링 PoC 완성
2. **이번 달**: 100x100 타일맵 데모
3. **3개월**: 좀비 시뮬레이션 프로토타입
4. **6개월**: 플레이 가능한 생존 게임 데모

**"3년 안에 Project Zomboid Clone 출시!"** 🧟‍♂️🎮