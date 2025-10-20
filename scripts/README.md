# Scripts

개발 자동화 스크립트 모음

## update_claude_priority.py

CLAUDE.md의 "Current Priority" 섹션을 자동 업데이트합니다.

### 사용법

```bash
# 자동 감지 (오늘 날짜 기준)
make update-priority
# 또는
python3 scripts/update_claude_priority.py

# 미리보기 (변경 없이)
python3 scripts/update_claude_priority.py --dry-run

# 수동 지정
python3 scripts/update_claude_priority.py --week 1 --day 3
```

### 언제 실행?

**매일 아침 개발 시작 전:**
```bash
make update-priority
```

**또는 새로운 날짜로 넘어갈 때 자동 실행 (선택):**
- Git pre-commit hook
- Cron job
- Shell startup script

### 동작 원리

1. **날짜 자동 감지:**
   - Week 1: 2025-10-20 ~ 2025-10-26
   - Week 2: 2025-10-27 ~ 2025-11-02
   - Week 3: 2025-11-03 ~ 2025-11-09
   - Week 4: 2025-11-10 ~ 2025-11-16

2. **Day-by-Day 작업 매핑:**
   - Monday (Day 1): Player 명세 & 구현
   - Tuesday (Day 2): Zombie 명세 & 구현
   - Wednesday (Day 3): 언어 개선 & 렌더링
   - Thursday (Day 4): 통합 데모 제작
   - Friday (Day 5): Pole Engine 리팩토링
   - Saturday (Day 6): 테스트 & 최적화
   - Sunday (Day 7): 데모 & 리뷰

3. **CLAUDE.md 업데이트:**
   - "🎯 Current Priority" 섹션을 오늘의 작업으로 교체

### 새 Week 추가하기

`update_claude_priority.py` 파일 수정:

1. **날짜 추가:**
```python
WEEK_STARTS = {
    1: datetime(2025, 10, 20),
    2: datetime(2025, 10, 27),
    3: datetime(2025, 11, 3),  # 새 Week 추가
}
```

2. **작업 정의 추가:**
```python
WEEK2_TASKS = {
    1: {
        "day": "Monday",
        "title": "전투 시스템",
        "tasks": [
            "Write combat.pole specification",
            # ...
        ],
    },
    # ...
}
```

3. **generate_priority_section() 함수 수정:**
```python
if week == 2:
    week_goal = "Combat system + 3 zombies"
    tasks_map = WEEK2_TASKS
```

## verify_development.sh

개발 중 모든 검증 체크를 실행합니다.

### 사용법

```bash
# 커밋 전 실행
./scripts/verify_development.sh

# 또는 Makefile 통해
make pre-commit
```

### 검증 항목

1. **의존성 체크**: pole, cargo 명령 사용 가능 여부
2. **.pole 파일 검증**: `pole check`로 명세 문법 확인
3. **Rust 컴파일러 빌드**: 최신 버전으로 빌드
4. **.pole-ir 파일 검증**: Rust 파서로 IR 문법 확인
5. **TODO 추적**: 미해결 TODO/FIXME 경고

### 언제 실행?

**필수:**
- Git commit 전
- Pull Request 생성 전

**권장:**
- 파일 수정 후 (빠른 피드백)
- Day 작업 완료 시

## 향후 추가 예정

- `validate_week_progress.py` - Week 완료도 체크
- `generate_weekly_report.py` - 주간 보고서 자동 생성
- `sync_roadmap.py` - ROADMAP.md와 CLAUDE.md 동기화
