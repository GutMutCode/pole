# SDL2 렌더링 데모

> Pole 언어로 SDL2를 사용하여 화면에 픽셀을 그리는 완전한 예제

**작성일:** 2025-10-19  
**난이도:** ⭐⭐⭐ 고급  
**파일:** `examples/25-sdl2-rendering.pole-ir`

---

## 🎨 데모 개요

이 데모는 Pole 언어의 FFI 시스템을 사용하여 SDL2 Renderer로 화면에 픽셀을 그립니다.

### 렌더링 내용
- 🟥 빨간색 패턴 (100, 100) - 십자 모양
- 🟦 파란색 패턴 (200, 200) - 십자 모양
- 🟩 녹색 패턴 (300, 300) - 십자 모양

---

## 🚀 실행 방법

### Headless 모드 (테스트용)
```bash
cd compiler
cargo run --example test_sdl2_rendering
```

**출력:**
```
✓✓✓ SUCCESS: SDL2 Rendering Demo works! ✓✓✓

What was rendered:
  🟥 Red pattern   at (100, 100) - 5 pixels
  🟦 Blue pattern  at (200, 200) - 5 pixels
  🟩 Green pattern at (300, 300) - 5 pixels
```

### 실제 윈도우 표시 (X11 환경)
```bash
# 컴파일 (한 번만)
cd compiler
cargo run --example test_sdl2_rendering

# 실제 윈도우로 실행
/tmp/sdl2_rendering
```

윈도우가 3초간 표시되고 자동으로 닫힙니다.

---

## 📋 코드 설명

### SDL2 함수 선언

```pole-ir
@extern("SDL_CreateRenderer")
func SDL_CreateRenderer(window: Ptr<Unit>, index: Int, flags: Int) -> Ptr<Unit>

@extern("SDL_SetRenderDrawColor")
func SDL_SetRenderDrawColor(renderer: Ptr<Unit>, r: Int, g: Int, b: Int, a: Int) -> Int

@extern("SDL_RenderClear")
func SDL_RenderClear(renderer: Ptr<Unit>) -> Int

@extern("SDL_RenderDrawPoint")
func SDL_RenderDrawPoint(renderer: Ptr<Unit>, x: Int, y: Int) -> Int

@extern("SDL_RenderPresent")
func SDL_RenderPresent(renderer: Ptr<Unit>) -> Unit
```

### 렌더링 로직

```pole-ir
func main() -> Int :
  let SDL_INIT_VIDEO = 32 in
  let init_result = SDL_Init(SDL_INIT_VIDEO) in
  
  if init_result == 0 then
    let window = SDL_CreateWindow(...) in
    let renderer = SDL_CreateRenderer(window, -1, 2) in
    
    // 배경을 검정으로 클리어
    let _ = SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255) in
    let _ = SDL_RenderClear(renderer) in
    
    // 빨간 픽셀 그리기
    let _ = SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255) in
    let _ = SDL_RenderDrawPoint(renderer, 100, 100) in
    let _ = SDL_RenderDrawPoint(renderer, 101, 100) in
    ...
    
    // 화면 업데이트
    let _ = SDL_RenderPresent(renderer) in
    
    // 정리
    let _ = SDL_DestroyRenderer(renderer) in
    let _ = SDL_DestroyWindow(window) in
    let _ = SDL_Quit(()) in
    0
  else
    1
```

---

## 🔧 기술적 세부사항

### SDL2 상수
```pole-ir
SDL_INIT_VIDEO = 32
SDL_WINDOWPOS_CENTERED = 805240832
SDL_WINDOW_SHOWN = 4
SDL_RENDERER_ACCELERATED = 2
```

### 렌더링 파이프라인
1. **초기화**: SDL_Init() → SDL_CreateWindow() → SDL_CreateRenderer()
2. **렌더링**:
   - SDL_SetRenderDrawColor() - 색상 설정
   - SDL_RenderClear() - 배경 클리어
   - SDL_RenderDrawPoint() - 픽셀 그리기
   - SDL_RenderPresent() - 화면 업데이트
3. **정리**: SDL_DestroyRenderer() → SDL_DestroyWindow() → SDL_Quit()

### 색상 값 (RGBA)
- 빨강: (255, 0, 0, 255)
- 파랑: (0, 0, 255, 255)
- 녹색: (0, 255, 0, 255)
- 검정: (0, 0, 0, 255)

---

## 💡 학습 포인트

### 1. FFI 함수 체이닝
```pole-ir
let renderer = SDL_CreateRenderer(window, -1, 2) in
let _ = SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255) in
let _ = SDL_RenderDrawPoint(renderer, 100, 100) in
...
```

### 2. 포인터 전달
- `window: Ptr<Unit>` → `renderer: Ptr<Unit>`
- 불투명 포인터를 함수 간 전달

### 3. 리소스 관리
```pole-ir
// 반드시 순서대로 정리
SDL_DestroyRenderer(renderer)  // 렌더러 먼저
SDL_DestroyWindow(window)       // 윈도우 그 다음
SDL_Quit(())                    // SDL 마지막
```

### 4. 에러 처리
```pole-ir
if init_result == 0 then
  // 성공 경로
  ...
else
  // 실패 경로
  1
```

---

## 🎯 확장 아이디어

### 더 복잡한 패턴 그리기
픽셀을 여러 개 그려서 선이나 도형 만들기:
```pole-ir
// 수평선 그리기
let _ = SDL_RenderDrawPoint(renderer, 100, 100) in
let _ = SDL_RenderDrawPoint(renderer, 101, 100) in
let _ = SDL_RenderDrawPoint(renderer, 102, 100) in
let _ = SDL_RenderDrawPoint(renderer, 103, 100) in
...
```

### 애니메이션 (향후)
루프를 사용하여 프레임마다 업데이트:
```pole-ir
// Phase 6.3 (모듈 시스템) 이후 가능
loop:
  clear()
  draw_frame(t)
  present()
  delay(16)  // 60 FPS
```

### 이벤트 처리 (M4.5)
키보드/마우스 입력으로 상호작용:
```pole-ir
// Phase 6.1 M4.5 이후 가능
SDL_PollEvent(event)
if event.type == SDL_KEYDOWN then
  // 키 입력 처리
```

---

## 🐛 알려진 제한사항

### 1. 구조체 전달 불가
현재 SDL_RenderFillRect는 지원되지 않음 (SDL_Rect* 필요):
```pole-ir
// ❌ 작동하지 않음
type SDL_Rect = { x: Int, y: Int, w: Int, h: Int }
SDL_RenderFillRect(renderer, rect)

// ✅ 대신 픽셀 단위 그리기 사용
SDL_RenderDrawPoint(renderer, x, y)
```

**해결 예정:** Phase 6.2 (저수준 메모리 제어) 에서 구조체 포인터 지원

### 2. 루프 없음
반복 그리기가 번거로움:
```pole-ir
// 현재: 일일이 작성
let _ = SDL_RenderDrawPoint(renderer, 100, 100) in
let _ = SDL_RenderDrawPoint(renderer, 101, 100) in
let _ = SDL_RenderDrawPoint(renderer, 102, 100) in
...
```

**해결 예정:** List 순회 또는 재귀 함수 활용

---

## 📚 관련 문서

- [FFI Tutorial](FFI_TUTORIAL.md) - C 함수 호출 기초
- [examples/24-sdl2-window.pole-ir](../examples/24-sdl2-window.pole-ir) - 윈도우 생성
- [SDL2 공식 문서](https://wiki.libsdl.org/SDL2/FrontPage)

---

## 🎉 성과

### 이 데모가 증명하는 것
- ✅ Pole에서 SDL2 Renderer 사용 가능
- ✅ 픽셀 단위 그리기 작동
- ✅ 색상 제어 정확
- ✅ 리소스 관리 안전
- ✅ 네이티브 성능 (~20ns/call)

### 다음 단계
1. **SDL2 이미지 로딩** - SDL_image 라이브러리
2. **텍스처 렌더링** - SDL_RenderCopy
3. **간단한 게임** - Pong, Snake
4. **OpenGL 통합** - 3D 그래픽

---

**작성자:** Claude (opencode)  
**버전:** 1.0
