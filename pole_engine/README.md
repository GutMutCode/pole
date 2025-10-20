# Pole Engine

> **2D 게임 엔진 라이브러리** - Pole 언어로 작성된 재사용 가능한 엔진 코드

Pole Engine은 Pole 언어의 핵심 가치를 실천하는 실전 프로젝트입니다:
- **명세 우선**: 모든 모듈은 .pole 명세로 시작
- **LLM 생성**: 구현은 LLM이 자동 생성
- **실전 검증**: Pole Zomboid 개발 과정에서 추출

---

## 📦 모듈 구조

### render/ - 렌더링 시스템
**SDL2 기반 2D 렌더링**
- `sprite.pole` - 스프라이트 렌더링
- `tilemap.pole` - 타일맵 렌더링
- `window.pole` - 윈도우 관리
- `camera.pole` - 카메라 시스템

### input/ - 입력 처리
**키보드, 마우스, 게임패드**
- `keyboard.pole` - 키보드 입력
- `mouse.pole` - 마우스 입력

### core/ - 핵심 유틸리티
**수학, 시간, 메모리**
- `types.pole` - 공통 타입 (Position, Color 등)
- `math.pole` - 수학 함수 (distance, clamp)
- `time.pole` - 시간 관리 (FPS, delta time)

### physics/ - 물리 시스템
**2D 충돌 감지**
- `collision.pole` - AABB 충돌
- `raycast.pole` - 레이캐스팅

### ai/ - AI 시스템
**경로 찾기, 행동 트리**
- `pathfinding.pole` - A* 알고리즘
- `behavior.pole` - FSM, 행동 트리

### network/ - 네트워킹
**멀티플레이어**
- `server.pole` - 서버
- `client.pole` - 클라이언트
- `protocol.pole` - 프로토콜 정의

---

## 🚀 사용 방법

### 1. 명세 확인
```pole
// pole_engine/render/sprite.pole 읽기
```

### 2. LLM으로 구현 생성
```bash
pole build pole_engine/render/sprite.pole
```

### 3. 게임에서 사용
```pole
// games/zomboid/main.pole
import pole_engine.render.sprite

function main() -> Int:
  let sprite = create_sprite(10, 10, 32, 32, Red) in
  draw_sprite(renderer, sprite)
```

---

## 📊 개발 상태

### Week 1 (2025-10-20)
- [ ] render/sprite.pole
- [ ] render/tilemap.pole
- [ ] input/keyboard.pole
- [ ] core/types.pole
- [ ] core/math.pole

### 향후 계획
- Week 2: physics, camera
- Week 3: ai/pathfinding
- Week 4: network 기초

---

## 🎯 설계 원칙

### 1. 단순함 (Simplicity)
- 복잡한 추상화 금지
- 명확하고 직관적인 API

### 2. 조합 가능 (Composable)
- 작은 함수들의 조합
- 의존성 최소화

### 3. 타입 안전 (Type Safe)
- 모든 함수 타입 명시
- 컴파일 타임 에러 감지

### 4. 성능 (Performance)
- Zero-cost abstractions
- 네이티브 컴파일

---

## 📝 기여 가이드

1. **명세 작성**: 새 기능은 .pole 명세로 시작
2. **LLM 생성**: `pole build` 사용
3. **테스트**: 반드시 테스트 케이스 포함
4. **문서화**: 각 함수에 purpose, examples 작성

---

Pole Engine과 함께 멋진 게임을 만들어보세요! 🎮
