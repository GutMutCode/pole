# Project Zomboid Clone

Pole 언어로 개발하는 Project Zomboid 스타일 생존 게임

## 구조

```
lib/          # 재사용 가능한 공통 라이브러리
  core.pole-ir         - Entity, Component 기본 구조
  inventory.pole-ir    - 인벤토리 시스템
  combat.pole-ir       - 전투 시스템
  survival.pole-ir     - 생존 시스템 (hunger/thirst)
  crafting.pole-ir     - 제작 시스템
  rendering.pole-ir    - SDL2 렌더링 래퍼

src/          # PZ 게임 로직
  main.pole-ir         - 메인 게임 루프
  player.pole-ir       - 플레이어 로직
  zombie.pole-ir       - 좀비 AI
  world.pole-ir        - 월드 관리
```

## 빌드

```bash
cd ../../compiler
cargo run --example zomboid_game
```

## 현재 상태

- [x] 공통 라이브러리 추출 중
- [ ] 게임 루프 통합
- [ ] 첫 플레이 가능 버전
