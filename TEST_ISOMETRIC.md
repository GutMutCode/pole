# 아이소메트릭 렌더링 데모 테스트 방법

## M2.1 완료 상태

✅ Unit 반환 타입 버그 수정
✅ 27-isometric-simple.pole-ir 컴파일 성공
✅ 프로그램 정상 실행 (exit code 0)

## NixOS SDL2 GUI 창 표시 문제 해결

### 원인
- NixOS는 `/usr/lib` 같은 표준 경로를 사용하지 않음
- SDL2가 X11/Wayland 라이브러리를 런타임에 찾지 못함

### 해결
`shell.nix`에 X11/Wayland 라이브러리 추가 완료:
- libX11, libXext, libXcursor, libXi, libXrandr
- libGL, libxkbcommon, wayland

## 테스트 방법

### 방법 1: 실제 터미널에서 실행 (권장)

Hyprland에서 새 터미널 열고:

```bash
cd /home/gmc/Devs/pole
nix-shell
/tmp/isometric_demo
```

**예상 결과:**
- 800x600 SDL2 윈도우 생성
- 제목: "Pole Isometric Demo"
- 배경: 어두운 녹색
- 중앙에 녹색 아이소메트릭 타일 1개
- 10초 후 자동 종료

### 방법 2: 재컴파일 후 실행

```bash
cd /home/gmc/Devs/pole
nix-shell
cd compiler
cargo run --example test_isometric
```

### 방법 3: 환경변수 직접 설정

```bash
# LD_LIBRARY_PATH 확인
nix-shell --run 'echo $LD_LIBRARY_PATH'

# 해당 환경변수로 실행
nix-shell --run '/tmp/isometric_demo'
```

## OpenCode 통합 터미널 제약

OpenCode IDE 통합 터미널에서는 SDL2 GUI 창이 표시되지 않을 수 있습니다.
반드시 **실제 시스템 터미널** (kitty, alacritty, konsole 등)에서 실행하세요.

## 검증 완료 항목

✅ 컴파일 성공
✅ LLVM IR 생성 성공
✅ 네이티브 실행 파일 생성
✅ SDL2 초기화 성공
✅ Exit code 0 (정상 종료)
✅ shell.nix SDL2 라이브러리 설정 완료

## 다음 단계 (M2.1 Day 2)

- [ ] For 루프 구현
- [ ] 10x10 그리드로 확장
- [ ] 또는 Day 3-4로 진행 (SDL2 이벤트 폴링)
