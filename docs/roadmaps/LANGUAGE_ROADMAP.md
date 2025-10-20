# Pole 언어 상세 로드맵

> 컴파일러, 런타임, LLM 통합 개발 계획

**최종 업데이트:** 2025-10-20

---

## 현재 상태

### 완료된 Phase (0-6)

**Phase 0-4: Python 프로토타입** ✅
- 명세 언어(.pole) 파서
- LLM 변환기
- IR 파서 & 타입 체커
- Python 인터프리터

**Phase 5: Rust 컴파일러** ✅
- IR Parser (23.4x 성능 향상)
- Type Checker (25.6x 성능 향상)
- 기본 LLVM 코드젠 (factorial 예제)

**Phase 6: FFI & 런타임** ✅
- SDL2 FFI 바인딩
- 런타임 함수 (print, malloc, free)
- 72개 예제 동작

---

## 단기 로드맵 (3개월)

### M2: LLVM 백엔드 완성 (2주)

**목표:** 모든 예제 네이티브 컴파일

**작업:**
- [ ] 함수 호출 코드젠
- [ ] if-else 코드젠
- [ ] Record 타입 코드젠
- [ ] List 타입 코드젠
- [ ] FFI 호출 코드젠

**성공 기준:**
- 72개 예제 모두 네이티브 컴파일
- 성능: Python 대비 100x

### M3: Arena Allocator (3주)

**목표:** 메모리 효율 75% 개선

**작업:**
- [ ] bumpalo 통합
- [ ] CompilerArenas 구조체
- [ ] OOM 복구 메커니즘
- [ ] 메모리 통계 수집

**성공 기준:**
- factorial: 110MB → 30MB
- 컴파일 속도 3x 향상
- OOM 크래시 0건

**상세:** [메모리 관리 가이드](../guides/MEMORY_MANAGEMENT.md)

### M4: 루프 구문 (2주)

**목표:** for/while 루프 지원

**명세 언어:**
```pole
loop i from 0 to 10:
  print(i)

while condition:
  ...
```

**IR:**
```
def loop_sum(n: Int) -> Int =
  loop i from 0 to n acc 0:
    acc + i
```

**작업:**
- [ ] 명세 언어 파서 수정
- [ ] LLM System Prompt 개선
- [ ] IR 문법 정의
- [ ] Rust 타입 체커 수정
- [ ] LLVM 코드젠 (br, phi)

---

## 중기 로드맵 (6개월)

### M5: 배열 & 슬라이싱

**명세:**
```pole
let arr = [1, 2, 3, 4, 5]
let slice = arr[1:3]  // [2, 3]
```

**IR:**
```
def slice(arr: Array<Int>, start: Int, end: Int) -> Array<Int>
```

### M6: 패턴 매칭

**명세:**
```pole
match value:
  case Some(x):
    print(x)
  case None:
    print("empty")
```

**IR:**
```
match value with
| Some(x) -> print(x)
| None -> print("empty")
```

### M7: 멀티스레드

**명세:**
```pole
spawn thread:
  heavy_computation()

await thread
```

**IR (Tentative):**
```
def spawn_thread(f: () -> Unit) -> Thread
def await_thread(t: Thread) -> Unit
```

### M8: LSP 기초

**기능:**
- 자동완성
- 타입 힌트
- 에러 표시
- 정의로 이동

**기술 스택:**
- Rust tower-lsp
- VS Code 확장

---

## 장기 로드맵 (1-2년)

### M9: 디버거

**기능:**
- 브레이크포인트
- 단계 실행
- 변수 inspect
- 콜스택 추적

**통합:**
- LLDB 연동
- DAP (Debug Adapter Protocol)

### M10: 패키지 관리자

**명세:**
```toml
# pole.toml
[package]
name = "my_game"
version = "0.1.0"

[dependencies]
pole_engine = "0.1"
```

**기능:**
- 중앙 레지스트리
- 버전 관리
- 의존성 해결

### M11: 에러 메시지 개선

**현재:**
```
Type error: expected Int, found String at line 15
```

**목표:**
```
error: type mismatch
  ┌─ player.pole:15:5
  │
15│     let x: Int = "hello"
  │                  ^^^^^^^ expected `Int`, found `String`
  │
  = help: convert with `parse_int(string)`
```

### M12: 최적화

**목표:**
- 인라인 최적화
- 데드 코드 제거
- 상수 폴딩
- 루프 전개

**기술:**
- LLVM Pass
- 프로파일 가이드 최적화

---

## 언어 기능 우선순위

### P0 (필수, 3개월 내)
- [x] 함수
- [x] if-else
- [x] Record, List
- [x] FFI
- [ ] 루프 (for, while)
- [ ] 배열 & 슬라이싱

### P1 (중요, 6개월 내)
- [ ] 패턴 매칭
- [ ] 제네릭 (기초)
- [ ] 트레잇/인터페이스
- [ ] 에러 처리 (Result<T, E>)

### P2 (유용, 1년 내)
- [ ] 멀티스레드
- [ ] async/await
- [ ] 매크로
- [ ] LSP

### P3 (선택, 2년 내)
- [ ] 패키지 관리자
- [ ] 디버거
- [ ] 프로파일러
- [ ] 웹어셈블리 백엔드

---

## LLM 통합 개선

### 현재 워크플로우

```
개발자: .pole 명세 작성
  ↓
LLM: .pole-ir 생성
  ↓
Rust: 타입 체크
  ↓
LLVM: 네이티브 컴파일
```

### 개선 계획

#### M13: Few-Shot Learning
- 예제 데이터베이스 구축
- 유사 패턴 자동 검색
- 생성 품질 90% → 99%

#### M14: 에러 피드백 루프
```
LLM 생성 → 타입 체크 실패
  ↓
에러 메시지를 LLM에 전달
  ↓
LLM 재생성 (에러 수정)
  ↓
성공할 때까지 반복 (최대 3회)
```

#### M15: 코드 리뷰 에이전트
- 생성된 IR 코드 검토
- 성능 문제 지적
- 안전성 검증
- 대안 제시

---

## 성능 목표

### 컴파일 속도

| 프로젝트 크기 | 현재 | 6개월 후 | 1년 후 |
|-------------|------|---------|--------|
| 1 파일 | 5ms | 2ms | 1ms |
| 100 파일 | 500ms | 200ms | 100ms |
| 1000 파일 | 5s | 2s | 1s |

### 런타임 성능

| 벤치마크 | Python | Pole (현재) | 목표 |
|---------|--------|------------|------|
| factorial | 1x | 100x | 100x |
| fibonacci | 1x | 100x | 100x |
| 좀비 AI (100개) | 1x | - | 100x |

### 메모리 사용량

| 단계 | 현재 | Arena 후 |
|------|------|---------|
| 파싱 | 50MB | 15MB |
| IR 생성 | 30MB | 10MB |
| 코드젠 | 20MB | 5MB |
| **합계** | 110MB | 30MB |

---

## 위험 요소 & 완화

### 위험 1: LLM 생성 품질 불안정

**완화:**
- Few-shot learning
- 에러 피드백 루프
- 명세 템플릿 제공

### 위험 2: LLVM 학습 곡선

**완화:**
- inkwell 사용 (High-level wrapper)
- 예제 코드 참고
- 단계적 기능 추가

### 위험 3: 타입 시스템 복잡도

**완화:**
- 점진적 타입 도입
- Hindley-Milner 알고리즘
- Rust 타입 체커 참고

---

## 관련 문서

- [언어 개발 가이드](../guides/LANGUAGE_DEV.md)
- [LLM 활용 가이드](../guides/LLM_USAGE.md)
- [메모리 관리](../guides/MEMORY_MANAGEMENT.md)
- [주간 계획](WEEKLY_PLANS.md)

---

**목표:** 2년 내 Pole 1.0 릴리스 (프로덕션 레디)
