# Week 3 Day 2 계획: 비주얼 렌더링

## 현재 상태
- ✅ 컴파일 완료
- ✅ SDL2 초기화 
- ✅ 게임 루프 동작
- ❌ 초록 화면만 보임 (렌더링 미구현)

## 구현할 것

### 1. SDL_RenderFillRect 추가
플레이어와 좀비를 사각형으로 그리기

```pole-ir
@extern("SDL_RenderFillRect")
func SDL_RenderFillRect(renderer: Ptr<Unit>, rect: Ptr<Unit>) -> Int

type Rect = { x: Int, y: Int, w: Int, h: Int }
```

### 2. render_game_state 개선
```pole-ir
func render_game_state(renderer: Ptr<Unit>, state: GameState, ...) -> Int:
  // 배경 (초록색)
  let _ = SDL_SetRenderDrawColor(renderer, 40, 80, 40, 255) in
  let _ = SDL_RenderClear(renderer) in
  
  // 플레이어 그리기 (노란색)
  let _ = SDL_SetRenderDrawColor(renderer, 255, 255, 0, 255) in
  let px = state.player.position.x * 32 in
  let py = state.player.position.y * 32 in
  let player_rect = { x: px, y: py, w: 32, h: 32 } in
  let _ = SDL_RenderFillRect(renderer, &player_rect) in
  
  // 좀비 그리기 (빨간색)
  let _ = SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255) in
  let zx = state.zombie.position.x * 32 in
  let zy = state.zombie.position.y * 32 in
  let zombie_rect = { x: zx, y: zy, w: 32, h: 32 } in
  let _ = SDL_RenderFillRect(renderer, &zombie_rect) in
  
  let _ = SDL_RenderPresent(renderer) in
  0
```

### 3. 타일맵 그리기 (선택)
벽을 회색 사각형으로 표시

### 4. 키보드 입력 (Day 2 후반)
SDL_PollEvent로 WASD 키 처리

## 실행 결과 예상
- 노란색 사각형 (플레이어) @ (10, 10)
- 빨간색 사각형 (좀비) @ (5, 5)
- 초록색 배경
- 10초 후 자동 종료
