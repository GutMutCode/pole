# LLM 효과적 활용 가이드

> Pole 언어 개발에서 LLM을 최대한 활용하는 방법

**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [LLM의 역할](#llm의-역할)
2. [명세 작성 원칙](#명세-작성-원칙)
3. [System Prompt 설계](#system-prompt-설계)
4. [트러블슈팅](#트러블슈팅)
5. [고급 기법](#고급-기법)

---

## LLM의 역할

### Pole의 LLM 워크플로우

```
1. 개발자: .pole 명세 작성 (자연어 + 타입)
   ↓
2. LLM: .pole-ir 구현 생성 (Pole IR 코드)
   ↓
3. Rust 컴파일러: 타입 체크 + LLVM 컴파일
   ↓
4. 네이티브 실행 파일
```

**LLM의 책임:**
- 명세 이해
- IR 코드 생성
- 타입 안전성 유지

**LLM이 하지 않는 것:**
- 타입 체크 (Rust가 함)
- 최적화 (LLVM이 함)
- 실행 (CPU가 함)

---

## 명세 작성 원칙

### 1. Purpose는 "무엇"과 "왜"

**나쁜 예:**
```pole
function factorial(n: Int) -> Int:
  purpose: "계승"
```

**좋은 예:**
```pole
function factorial(n: Int) -> Int:
  purpose: "정수 n의 계승(n!)을 계산. 수학적 조합 계산에 사용"
```

### 2. Examples는 구체적이고 다양하게

**최소 3가지 케이스:**
1. **정상 케이스:** 일반적 사용
2. **엣지 케이스:** 경계 조건 (0, 1, 최댓값)
3. **에러 케이스:** 잘못된 입력

**예제:**
```pole
function clamp(value: Int, min: Int, max: Int) -> Int:
  purpose: "값을 [min, max] 범위로 제한"
  
  examples:
    - clamp(50, 0, 100) → 50      # 정상
    - clamp(150, 0, 100) → 100    # 최댓값 초과
    - clamp(-10, 0, 100) → 0      # 최솟값 미만
    - clamp(0, 0, 100) → 0        # 경계값
    - clamp(100, 0, 100) → 100    # 경계값
```

### 3. Constraints는 명시적으로

**나쁜 예:**
```pole
constraints:
  - "제대로 작동해야 함"
```

**좋은 예:**
```pole
constraints:
  - "0 <= x < map_width (맵 경계 체크)"
  - "tile_id == 1 (벽)이면 이동 불가"
  - "반환값은 항상 양수"
  - "시간 복잡도 O(1)"
```

### 4. 타입은 모든 곳에

```pole
// 나쁨: 타입 생략
function foo(x, y):
  ...

// 좋음: 모든 타입 명시
function manhattan_distance(pos1: Position, pos2: Position) -> Int:
  ...
```

---

## System Prompt 설계

### 현재 Pole System Prompt

```markdown
You are a Pole IR code generator.

Rules:
1. Always include 'else' branch for all 'if' expressions
2. Multi-argument function calls use tuple: f(a, b) becomes f((a, b))
3. Use descriptive variable names: new_player instead of p2
4. Ensure type inference succeeds (explicit types where needed)
5. Follow functional programming style (immutable data)

Output format:
- Only output Pole IR code
- No explanations or comments
- Ensure code type-checks

Example:
Input spec: factorial(5) → 120
Output IR:
def factorial(n: Int) -> Int =
  if n <= 1 then
    1
  else
    n * factorial(n - 1)
```

### System Prompt 개선 팁

#### 1. 구체적인 문법 규칙

```markdown
Function call syntax:
- Single arg: f(x)
- Multiple args: f((x, y, z))  # Tuple!
- No args: f(())

If-else syntax:
- ALWAYS include else branch
- if condition then expr1 else expr2

Let binding:
- let x = expr1 in expr2
- Chain multiple: let x = e1 in let y = e2 in e3
```

#### 2. 예제 포함

```markdown
Example 1: Simple function
def add(x: Int, y: Int) -> Int = x + y

Example 2: If-else
def max(a: Int, b: Int) -> Int =
  if a > b then a else b

Example 3: Let binding
def distance(x1: Int, y1: Int, x2: Int, y2: Int) -> Int =
  let dx = abs(x1 - x2) in
  let dy = abs(y1 - y2) in
  dx + dy
```

#### 3. 금지 사항 명시

```markdown
DO NOT:
- Omit 'else' branch
- Use multi-arg syntax f(a, b) - use f((a, b)) instead
- Use undefined variables
- Mix tabs and spaces
- Add comments (code only)
```

---

## 트러블슈팅

### 문제 1: LLM이 else를 빠뜨림

**증상:**
```
Type error: if expression missing else branch
```

**해결:**
1. System prompt에 강조 추가
2. 예제에서 모든 if에 else 포함
3. 명세에 명시:
   ```pole
   constraints:
     - "모든 if는 else 포함 (Pole IR 문법)"
   ```

### 문제 2: Multi-arg 함수 호출 문법 오류

**증상:**
```
Parse error: expected tuple for multi-arg call
```

**해결:**
1. System prompt에 tuple 문법 명시
2. 예제에서 정확한 문법 사용:
   ```pole
   examples:
     - distance(0, 0, 3, 4) → 7  # Spec 언어
     # IR에서는: distance((0, 0, 3, 4))
   ```

### 문제 3: 타입 추론 실패

**증상:**
```
Type error: cannot infer type of variable 'x'
```

**해결:**
1. 명세에서 중간 변수 타입 힌트 제공
2. System prompt에 타입 명시 규칙 추가:
   ```markdown
   When type inference might fail:
   - Add explicit type: let (x: Int) = expr in ...
   ```

### 문제 4: 생성된 코드가 명세와 다름

**증상:**
LLM이 요구사항을 잘못 이해

**해결:**
1. Purpose를 더 명확하게
2. Examples를 더 구체적으로
3. Constraints 추가
4. 재생성 (`--force` 플래그)

---

## 고급 기법

### 1. Few-Shot Learning

명세에 유사한 예제 포함:

```pole
// Similar function reference
// See: examples/07-max.pole-ir for similar pattern

function min(a: Int, b: Int) -> Int:
  purpose: "두 수 중 작은 값 반환 (max의 반대)"
  examples:
    - min(10, 20) → 10
```

### 2. 점진적 복잡도 증가

**Step 1: 단순 버전**
```pole
function move_player_simple(player: Player, dx: Int, dy: Int) -> Player:
  purpose: "플레이어 위치 이동 (충돌 체크 없음)"
```

**Step 2: 경계 체크 추가**
```pole
function move_player_bounded(player: Player, dx: Int, dy: Int, 
                             map_width: Int, map_height: Int) -> Player:
  purpose: "플레이어 이동 (맵 경계 체크)"
```

**Step 3: 충돌 체크 추가**
```pole
function move_player_full(player: Player, direction: Direction, 
                         tilemap: Tilemap) -> Player:
  purpose: "플레이어 이동 (경계 + 충돌 체크)"
```

### 3. 테스트 주도 명세 작성

명세에 검증 가능한 속성 포함:

```pole
function sort(list: List<Int>) -> List<Int>:
  purpose: "정수 리스트를 오름차순 정렬"
  
  constraints:
    - "결과 길이 = 입력 길이"
    - "결과는 정렬됨: result[i] <= result[i+1]"
    - "입력의 모든 원소가 결과에 포함"
  
  examples:
    - sort([3, 1, 2]) → [1, 2, 3]
    - sort([]) → []
    - sort([5]) → [5]
    - sort([1, 1, 1]) → [1, 1, 1]  # 중복 처리
```

### 4. 명세 템플릿 활용

**함수 템플릿:**
```pole
function FUNCTION_NAME(PARAM1: TYPE1, PARAM2: TYPE2) -> RETURN_TYPE:
  purpose: "[무엇을] [왜]"
  
  input:
    - PARAM1: [설명]
    - PARAM2: [설명]
  
  output: [설명]
  
  constraints:
    - "[제약조건 1]"
    - "[제약조건 2]"
  
  examples:
    - FUNCTION_NAME(정상_입력) → 정상_출력
    - FUNCTION_NAME(엣지_케이스) → 엣지_출력
    - FUNCTION_NAME(에러_케이스) → 에러_출력
```

### 5. LLM 피드백 루프

```bash
# 1차 생성
pole build player.pole

# 타입 체크
pole check player.pole-ir

# 에러 발견 시 명세 개선
vim player.pole  # 예제/제약조건 추가

# 2차 생성
pole build player.pole --force

# 검증
pole test player.pole-ir
```

---

## 모범 사례 체크리스트

### 명세 작성 전

- [ ] 비슷한 기존 예제 찾기 (`examples/` 디렉토리)
- [ ] 필요한 타입 정의 확인
- [ ] 엣지 케이스 생각하기

### 명세 작성 시

- [ ] purpose: "무엇" + "왜"
- [ ] 모든 파라미터 타입 명시
- [ ] input/output 설명
- [ ] constraints 3개 이상
- [ ] examples 3개 이상 (정상/엣지/에러)

### LLM 생성 후

- [ ] `pole check` 실행 (타입 체크)
- [ ] `pole test` 실행 (유닛 테스트)
- [ ] 생성된 코드 리뷰
- [ ] 필요시 재생성

### 문제 발생 시

- [ ] 에러 메시지 확인
- [ ] 명세 개선 (더 구체적으로)
- [ ] System prompt 조정
- [ ] 비슷한 예제 참고

---

## 관련 문서

- [워크플로우](../../specs/workflow.md) - LLM 변환 상세 과정
- [명세 언어](../../specs/syntax-v0.md) - .pole 문법
- [IR 문법](../../specs/ir-syntax.md) - .pole-ir 문법
- [게임 개발 가이드](GAME_DEV.md) - 실제 사용 예제

---

## 추천 설정

### OpenRouter API

```bash
export OPENROUTER_API_KEY="sk-or-..."
export OPENROUTER_MODEL="anthropic/claude-3.5-sonnet"  # 추천
```

### 모델 선택 기준

| 모델 | 장점 | 단점 | 추천 용도 |
|------|------|------|-----------|
| Claude 3.5 Sonnet | 정확도 높음, 복잡한 로직 | 느림, 비쌈 | 복잡한 함수 |
| GPT-4 Turbo | 빠름, 안정적 | 가끔 문법 오류 | 간단한 함수 |
| GPT-3.5 | 매우 빠름, 저렴 | 정확도 낮음 | 프로토타이핑 |

---

**팁:** 명세가 구체적일수록 LLM 결과가 좋습니다! 🚀
