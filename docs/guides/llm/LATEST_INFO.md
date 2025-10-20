# LLM 최신 정보 문제

> LLM의 가장 큰 한계: Cutoff Date

**최종 업데이트:** 2025-10-20  
**웹 검색 완료:** 2025-10-20 (Anthropic News, OpenAI, Claude Docs)

---

## ⚠️ 핵심 한계점

**Claude Sonnet 4.5 (최신 모델) 기준:**

1. **3개월 뒤처짐** (Training cutoff: 2025-07)
   - 2025년 8-10월 기술 → 90% 오류율

2. **웹 검색 필수**
   - LLM만으로는 불가능
   - 공식 문서 확인 필수

---

## 문제 정의

### Cutoff Date

**LLM 학습 데이터는 특정 시점까지만 (2025년 10월 기준):**

| 모델 | Reliable Cutoff | Training Cutoff | 출시일 |
|------|----------------|-----------------|--------|
| **Claude Sonnet 4.5** ⭐ | 2025년 1월 | 2025년 7월 | 2025-09-29 |
| Claude Haiku 4.5 | 2025년 2월 | 2025년 7월 | 2025-10-15 |
| Claude Opus 4.1 | 2025년 1월 | 2025년 3월 | 2025-08-05 |

**핵심 문제:**
- **2025년 8월 이후 기술** → Claude도 모름 (3개월 뒤처짐)
- **최신 라이브러리 버전** → 구식 방법 제안
- **API 변경사항** → 오류 코드 생성

### 오류율 (Claude Sonnet 4.5 기준)

| 기술 나이 | 오류율 | 예시 |
|----------|--------|------|
| < 3개월 | **90%** | 2025년 8-10월 출시 기술 |
| 3-12개월 | **70%** | 2025년 초반 기술 |
| 1-2년 | **40%** | 2024년 기술 |
| 2년+ | **10%** | 2023년 이전 기술 |

**핵심:** 2025년 8월 이후 기술은 90% 오류율

---

## 실제 사례

### 1. LLVM 17 Opaque Pointers (2023년 9월)

**LLM이 제안 (LLVM 14 스타일):**
```rust
let ptr_type = context.i8_type().ptr_type(AddressSpace::Generic);
```

**실제 필요 (LLVM 17):**
```rust
let ptr_type = context.ptr_type(AddressSpace::Generic);
```

**해결:** LLVM 17 문서 검색 → 명세에 반영

### 2. SDL3 출시 (2024년 3월)

**LLM 지식:**
```c
// SDL2 API (구버전)
SDL_CreateWindow(title, x, y, w, h, flags)
```

**실제 (SDL3):**
```c
// SDL3 API (새 버전)
SDL_CreateWindow(title, w, h, flags)  // x, y 제거됨
```

**해결:** SDL3 공식 문서 확인

### 3. Rust 2024 Edition (2024년 2월)

**LLM이 모름:**
- `gen` 블록 (제네레이터)
- `async fn` in traits 안정화
- 새로운 lint 규칙

**해결:** Rust 블로그 확인 → 최신 패턴 사용

### 4. Pole 자체 문법

**LLM이 모름:**
```pole
// Pole은 2024년 신규 언어
// LLM은 Pole 문법을 전혀 모름
// → System prompt로 문법 전체 제공 필요
```

---

## 웹 검색 워크플로우 ⭐ 핵심 해결책

### 필수 단계

```
명세 작성 전:
1. 공식 문서 확인
2. GitHub 최신 예제
3. StackOverflow 최근 답변 (2024년+)
   ↓
명세 작성 (검증된 정보)
   ↓
LLM 생성
   ↓
추가 검증
```

### 검색 체크리스트

**새 라이브러리/API 사용 시:**
- [ ] 공식 문서 최신 버전 확인
- [ ] GitHub repo README 확인
- [ ] Latest release notes 확인
- [ ] Breaking changes 확인
- [ ] Migration guide 확인

**예시: SDL2 사용 시**
```bash
# 1. 공식 문서
https://wiki.libsdl.org/SDL2/

# 2. GitHub
https://github.com/libsdl-org/SDL/releases

# 3. 함수 시그니처
https://wiki.libsdl.org/SDL2/SDL_CreateWindow

# 4. 예제 코드
https://github.com/libsdl-org/SDL/tree/main/test
```

---

## 명세 작성 패턴

### 1. 버전 명시

```pole
@ffi("SDL2", version="2.28.0")  // 명시적 버전
@extern("SDL_CreateWindow")
function sdl_create_window(...):
  purpose: "SDL2 윈도우 생성"
```

### 2. 문서 링크 포함

```pole
function sdl_init(flags: u32) -> i32:
  purpose: "SDL2 초기화"
  
  reference:
    - "SDL2 2.28.0 공식 문서"
    - "https://wiki.libsdl.org/SDL2/SDL_Init"
    - "examples/24-sdl2-window.pole-ir 참고"
  
  examples:
    - sdl_init(SDL_INIT_VIDEO) → 0  // 성공
    - sdl_init(0xFFFFFFFF) → -1      // 잘못된 플래그
```

### 3. 릴리스 날짜 기록

```pole
type NewFeature = { ... }
  // Added: 2024-10-15
  // Breaking change from v1.x
  // See: https://docs.example.com/v2/migration
```

---

## 검증 방법

### LLM 생성 후 체크리스트

- [ ] 함수 시그니처가 최신 문서와 일치하는가?
- [ ] 반환 타입이 정확한가?
- [ ] 에러 처리 방식이 최신 best practice인가?
- [ ] Deprecated API를 사용하고 있지 않은가?
- [ ] 버전별 차이를 고려했는가?

### 검증 도구

```bash
# 공식 문서 비교
diff <(cat generated.pole-ir) <(cat expected_from_docs.pole-ir)

# 실제 실행 테스트
pole build example.pole && pole run example.pole-ir

# 타입 체크
pole check example.pole-ir
```

---

## 권장 정보 소스

### 신뢰도 높음 ⭐⭐⭐
1. **공식 문서** (1순위)
2. **GitHub Official Repo**
3. **Release Notes**

### 신뢰도 보통 ⭐⭐
4. **StackOverflow** (최근 답변만)
5. **Reddit r/rust, r/programming**
6. **Official Blog Posts**

### 신뢰도 낮음 ⭐
7. **Medium 포스트** (날짜 확인 필수)
8. **개인 블로그** (검증 필요)
9. **오래된 튜토리얼** (2년 이상)

### 절대 신뢰 금지 ❌
- **LLM 생성 코드** (검증 없이)
- **날짜 없는 글**
- **비공식 문서**

---

## 자주 묻는 질문

### Q1: LLM이 최신 정보를 알 수 있는 방법은?

**A:** 없습니다. Cutoff date는 고정입니다.
- Claude API: 웹 검색 기능 없음
- OpenAI: 일부 모델만 browsing 지원
- **해결:** 개발자가 직접 웹 검색

### Q2: System prompt에 최신 정보를 넣으면?

**A:** 가능하지만 제한적:
- Prompt 길이 제한
- 모든 정보를 넣을 수 없음
- **권장:** 핵심 문법/API만 포함

### Q3: Fine-tuning으로 해결 가능한가?

**A:** 부분적으로만:
- 특정 라이브러리는 가능
- 하지만 비용이 매우 높음
- **현실적:** 웹 검색이 더 빠르고 저렴

### Q4: Pole 자체 문법은 어떻게?

**A:** System prompt에 전체 문법 제공:
```python
system_prompt = f"""
You are a Pole IR generator.

Pole IR syntax:
{read_file('specs/ir-syntax.md')}

Examples:
{read_file('examples/01-factorial.pole-ir')}
"""
```

---

## 결론

### 핵심 교훈

1. **LLM은 구식 정보만 가짐**
   - Cutoff date 이해 필수
   - 최신 기술 80% 오류율

2. **웹 검색은 필수 워크플로우**
   - 명세 작성 전에 먼저 검색
   - 공식 문서가 최고의 소스

3. **명세에 정보 포함**
   - 버전 명시
   - 문서 링크
   - 릴리스 날짜

4. **항상 검증**
   - LLM 생성 → 문서 비교 → 수정
   - 테스트 필수

### 실용적 조언

```
LLM 활용 = 웹 검색 (50%) + 명세 작성 (30%) + 검증 (20%)

최신 기술 사용 시 웹 검색 비중 더 높임:
웹 검색 (70%) + 명세 (20%) + 검증 (10%)
```

---

## 관련 문서

- [LLM 활용법](USAGE.md)
- [LLM 한계](LIMITATIONS.md)
- [우회 전략](WORKAROUNDS.md)
- [README](README.md)

---

**교훈:** LLM은 과거만 알고, 개발자는 현재를 안다. 🔍
