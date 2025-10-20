# LLM 한계 및 우회 전략

> Pole 개발에서 LLM의 한계와 이를 극복하는 방법

**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [LLM 한계 개요](#llm-한계-개요)
2. [기술적 한계](#기술적-한계)
3. [생성 품질 한계](#생성-품질-한계)
4. [우회 전략](#우회-전략)
5. [현실적 기대치](#현실적-기대치)

---

## LLM 한계 개요

### Pole에서 LLM의 역할

```
명세 (.pole) → LLM → 구현 (.pole-ir)
```

**LLM이 하는 것:**
- 명세 이해
- IR 코드 생성
- 예제 기반 학습

**LLM이 하지 않는 것:**
- 타입 체크 (Rust 컴파일러가 함)
- 최적화 (LLVM이 함)
- 버그 수정 (개발자가 함)

### 왜 LLM 한계를 이해해야 하는가?

1. **현실적 기대:** 100% 정확도는 불가능
2. **워크플로우 개선:** 한계를 알면 명세를 더 잘 작성
3. **디버깅 효율:** 문제 발생 시 빠른 대응
4. **전략 수립:** 한계를 우회하는 패턴 개발

---

## 기술적 한계

### 1. 최신 정보 부족 ⭐ 중요

**문제:**
- LLM 학습 데이터는 특정 시점까지만 (cutoff date)
- **Claude Sonnet 4.5** (최신): 2025년 1월 (reliable), 7월 (training)
- **Claude Haiku 4.5**: 2025년 2월 (reliable), 7월 (training)
- **GPT-4o**: ~2023년 10월 (1년+ 구식)
- **문제:** 2025년 하반기 기술은 Claude도 모름!

**증상:**
```pole
// LLM이 생성한 코드
@ffi("SDL2")
@extern("SDL_CreateWindow")
function sdl_create_window(...) -> Ptr<SDL_Window>

// 문제: SDL3 출시 (2024년 3월)
// LLM은 SDL2만 알고 있음
```

**구체적 예시:**

1. **라이브러리 버전**
   ```
   LLM 지식: LLVM 14 (2022)
   현재 사용: LLVM 17 (2023)
   → Opaque pointer 문법 변경 인식 못함
   ```

2. **언어 기능**
   ```
   LLM 지식: Rust 1.70
   현재 사용: Rust 1.75+
   → 새로운 문법 (let-else) 제안 못함
   ```

3. **Best practices**
   ```
   LLM이 제안: 구식 패턴
   현재 권장: 새로운 패턴
   → 비효율적 코드 생성
   ```

**발생 빈도 (2025년 10월 기준):**
- **초최신** (< 3개월, 2025-08~10): Claude 90%, GPT-4o 95%
- **최신** (3-12개월, 2025년 초): Claude 70%, GPT-4o 85%
- **비교적 새** (1-2년, 2024): Claude 40%, GPT-4o 60%
- **안정** (2년+, 2023 이전): Claude 10%, GPT-4o 20%

**권장:** **Claude Sonnet 4.5 사용** (GPT-4o는 1년 뒤처짐)

**우회 전략:**

1. **웹 검색 활용** ⭐ 핵심 해결책
   ```bash
   # 명세 작성 전에 먼저 검색
   1. 공식 문서 확인
   2. GitHub 최신 예제 확인
   3. StackOverflow 최근 답변 확인
   
   # 예: SDL2 최신 API
   https://wiki.libsdl.org/SDL2/
   → 최신 함수 시그니처 확인
   ```

2. **명세에 버전 명시**
   ```pole
   @ffi("SDL2", version="2.28.0")  // 명시적 버전
   @extern("SDL_CreateWindow")
   function sdl_create_window(...):
     // 공식 문서 링크 포함
     // https://wiki.libsdl.org/SDL2/SDL_CreateWindow
   ```

3. **검증된 예제 참조**
   ```pole
   function sdl_init():
     purpose: "SDL2 초기화"
     
     reference:
       - "SDL2 2.28.0 공식 문서"
       - "examples/24-sdl2-window.pole-ir 참고"
     
     examples:
       - sdl_init(SDL_INIT_VIDEO) → 0  // 성공
   ```

4. **개발자 직접 확인:** 공식 문서 비교, 시그니처 검증

**Pole 워크플로우:**
```
웹 검색 (최신 정보) → 명세 작성 → LLM 생성 → 검증
```

**실제 사례:**
- LLVM 17 Opaque Pointers: 문서 검색 후 명세 반영
- Rust 2024 Edition: 공식 블로그 확인 후 적용

### 2. 컨텍스트 윈도우 제한

**문제:**
- **Claude Sonnet 4.5**: 200K tokens (1M beta 지원)
- **Claude Haiku 4.5**: 200K tokens
- **GPT-4o**: 128K tokens
- 큰 명세는 잘려서 전달

**참고:** Claude 1M context는 베타 (실험적)

**증상:**
```pole
// 500줄 명세 작성
function complex_function(...):
  purpose: "..."
  examples: [50개 예제]
  
// LLM은 뒷부분만 보고 생성
// 앞부분 context 손실
```

**우회:** 명세 분할 (200줄 이하), 핵심 예제만 (3-5개)

### 2. 문법 오류 빈도

**문제:**
- LLM은 확률 기반 생성
- 문법 규칙을 100% 따르지 않음

**증상 (Pole IR):**
```pole-ir
// LLM 생성 코드
def factorial(n: Int) -> Int =
  if n <= 1 then
    1
  // else 빠뜨림! ❌
```

**발생 빈도:**
- Claude 3.5: ~5-10%
- GPT-4: ~10-15%
- GPT-3.5: ~20-30%

**우회:** System prompt 강조, 명세에 명시, 재생성 (`--retry 3`)

### 3. 다중 인자 함수 혼동

**문제:** Pole IR `f((a,b))` vs spec `f(a,b)` 혼동 (~15-20%)  
**우회:** System prompt에 tuple 문법 명시

### 4. 타입 추론 실패

**문제:** LLM이 타입 명시 누락  
**우회:** 명세에 모든 타입 명시

---

## 생성 품질 한계

### 1. 복잡한 로직 이해 부족

**문제:**
- LLM은 의미 이해 없이 패턴 매칭
- 복잡한 알고리즘 생성 실패

**증상:**
```pole
function a_star_pathfinding(start, goal, tilemap) -> List<Position>:
  purpose: "A* 알고리즘으로 최단 경로 찾기"
  examples:
    - a_star((0,0), (5,5), map) → [(0,0), (1,1), ..., (5,5)]
```

**LLM 생성 결과:**
- 잘못된 휴리스틱
- 우선순위 큐 오류
- 무한 루프 가능성

**성공률:**
- 단순 함수 (factorial): ~95%
- 중간 복잡도 (BFS): ~70%
- 고급 알고리즘 (A*): ~40%

**우회 전략:**

1. **알고리즘 분해**
   ```pole
   // 나쁨: 한 번에 전체
   function a_star(...) -> Path
   
   // 좋음: 단계별 분해
   function heuristic(pos1, pos2) -> Int
   function get_neighbors(pos, map) -> List<Position>
   function reconstruct_path(came_from, current) -> List<Position>
   function a_star_search(...) -> Path
   ```

2. **의사코드 제공**
   ```pole
   function a_star(...):
     purpose: "A* pathfinding"
     
     algorithm:
       1. Initialize open_set with start
       2. While open_set not empty:
         a. Get node with lowest f_score
         b. If node == goal, return path
         c. For each neighbor:
           - Calculate tentative g_score
           - If better than current, update
       3. Return empty (no path)
   ```

3. **검증된 예제 참고**
   ```pole
   // See: examples/XX-bfs.pole-ir for similar pattern
   ```

### 2. 엣지 케이스 누락

**문제:**
- LLM은 일반 케이스만 고려
- 예외 상황 처리 누락

**증상:**
```pole-ir
def divide(a: Int, b: Int) -> Int =
  a / b  // ❌ b=0 처리 안 함
```

**발생 빈도:**
- Claude: ~30%
- GPT-4: ~40%

**우회 전략:**

1. **명세에 명시**
   ```pole
   function divide(a: Int, b: Int) -> Result<Int, Error>:
     constraints:
       - "b == 0이면 DivisionByZero 에러"
       - "오버플로우 체크"
     
     examples:
       - divide(10, 2) → Ok(5)
       - divide(10, 0) → Err(DivisionByZero)
   ```

2. **타입으로 강제**
   ```pole
   // Int 대신 Result 사용
   function divide(...) -> Result<Int, Error>
   ```

### 3. 성능 최적화 미고려

**문제:**
- LLM은 정확성만 고려
- 성능 최적화 안 함

**증상:**
```pole-ir
// LLM 생성: O(n²)
def contains(list: List<Int>, item: Int) -> Bool =
  match list with
  | [] -> false
  | head :: tail ->
    if head == item then true
    else contains(tail, item)  // 재귀 (느림)
```

**우회 전략:**

1. **성능 요구사항 명시**
   ```pole
   function contains(...):
     constraints:
       - "시간 복잡도: O(n)"
       - "공간 복잡도: O(1)"
   ```

2. **최적화 힌트**
   ```pole
   function batch_update_zombies(zombies: List<Zombie>):
     performance:
       - "병렬 처리 가능 (각 좀비 독립적)"
       - "캐시 locality 고려"
   ```

### 4. 일관성 부족

**문제:**
- 같은 명세로 다른 결과
- 재생성 시 스타일 변경

**증상:**
```pole-ir
// 첫 생성
def factorial(n: Int) -> Int =
  if n <= 1 then 1 else n * factorial(n - 1)

// 재생성 (다른 스타일)
def factorial(n: Int) -> Int =
  match n with
  | 0 -> 1
  | 1 -> 1
  | _ -> n * factorial(n - 1)
```

**우회 전략:**

1. **스타일 명시**
   ```pole
   constraints:
     - "if-else 스타일 사용 (match 사용 금지)"
   ```

2. **Temperature 낮추기**
   ```python
   # LLM API 호출 시
   temperature = 0.2  # 기본 0.7
   # 더 결정적, 덜 창의적
   ```

---

## 우회 전략

### 1. 명세 품질 향상

**체크리스트:**
- [ ] purpose 1-2줄 명확
- [ ] 모든 타입 명시
- [ ] 예제 3개 이상 (정상/엣지/에러)
- [ ] 제약조건 3개 이상
- [ ] 성능 요구사항 명시

### 2. 반복적 개선

```
명세 → 생성 → 검증 → 실패
  ↓
명세 개선 → 재생성 → 성공
```

**팁:** 2-3번 재시도로 95% 성공률

### 3. 검증 자동화

```bash
pole build player.pole && pole test player.pole-ir
```

---

## 현실적 기대치

### 성공률 (경험 기반)

| 작업 복잡도 | Claude 3.5 | GPT-4 | GPT-3.5 |
|------------|-----------|-------|---------|
| 단순 함수 (factorial) | 95% | 90% | 80% |
| 중간 복잡도 (BFS) | 75% | 70% | 50% |
| 복잡한 로직 (A*) | 50% | 40% | 20% |
| 시스템 통합 | 30% | 25% | 10% |

### 시간 예상

| 작업 | 이상적 | 현실 |
|------|--------|------|
| 단순 함수 | 1분 | 5-10분 (재시도 포함) |
| 중간 함수 | 5분 | 20-30분 |
| 복잡한 함수 | 10분 | 1-2시간 |
| 전체 시스템 | 30분 | 1일 |

### 개발자 개입 필요

**항상 필요:**
- 명세 작성
- 생성 결과 검증
- 버그 수정
- 성능 최적화

**가끔 필요:**
- IR 직접 수정
- 알고리즘 재설계
- 엣지 케이스 추가

**불필요:**
- 단순 타이핑
- 보일러플레이트 코드
- 반복적 패턴

### Pole의 철학

```
LLM은 도구이지 마법이 아니다.

- LLM: 80% 코드 작성
- 개발자: 20% 검증 및 수정

= 5배 생산성 향상
```

---

## 모범 사례

### Do ✅
1. 구체적 명세 (예제 3개 이상)
2. 단계적 복잡도 증가
3. 생성 결과 항상 검증

### Don't ❌
1. 모호한 명세
2. LLM 100% 신뢰
3. 복잡한 함수 한 번에

---

## 결론

### 핵심 교훈

1. **LLM은 완벽하지 않다**
   - 95% 정확도가 현실적 목표
   - 항상 검증 필요

2. **명세가 핵심이다**
   - 좋은 명세 = 좋은 생성
   - 시간을 명세에 투자

3. **반복이 필요하다**
   - 첫 시도는 실패 예상
   - 2-3번 재시도로 성공

4. **개발자는 여전히 필요하다**
   - LLM은 도구일 뿐
   - 전문성은 대체 불가

### 실용적 조언

```
LLM 활용 = 명세 작성 (30%) + 검증 (20%) + 수정 (10%)
          + 재시도 (20%) + 최적화 (20%)

결과 = 5배 생산성, 100배 만족도
```

---

## 관련 문서

- [LLM 활용 가이드](LLM_USAGE.md) - 효과적 사용법
- [명세 언어](../../specs/syntax-v0.md) - .pole 문법
- [워크플로우](../../specs/workflow.md) - LLM 변환 과정

---

**교훈:** LLM의 한계를 이해하면 더 잘 활용할 수 있다. 🧠
