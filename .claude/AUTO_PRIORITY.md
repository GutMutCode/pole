# 자동 우선순위 조정 시스템 (Auto Priority Management)

> **목표:** LLM이 새로운 문제 발견 시 자동으로 우선순위를 조정하고 작업 계획을 재배치

**작성일:** 2025-10-21  
**상태:** 설계 완료, 구현 대기

---

## 🎯 목표

### Before (현재 - 반자동)
```
1. LLM이 문제 발견
2. 🤔 사람에게 물어봄: "이 문제를 언제 처리해야 할까요?"
3. 사람이 우선순위 결정
4. LLM이 문서 작성 (.claude/DECISION.md 등)
5. 사람이 승인
6. LLM이 실행
```

### After (완전 자동)
```
1. LLM이 문제 발견
2. 🤖 자동 분석 시스템 실행:
   - 영향도 평가
   - 긴급도 평가
   - 복잡도 평가
   - 의존성 분석
3. 🎯 자동 우선순위 결정
4. 📋 TODO 리스트 자동 재배치
5. 📝 의사결정 기록 (자동)
6. ℹ️ 사람에게 간단히 알림
7. ▶️ 즉시 실행 (또는 사람 승인 대기)
```

---

## 📊 자동 분석 프레임워크

### 1. 영향도 분석 (Impact Analysis)

**질문:**
- Week 1 목표 달성을 차단하는가?
- 현재 진행 중인 작업에 영향을 주는가?
- LLM 자동화 파이프라인을 망가뜨리는가?

**점수 계산:**
```python
def calculate_impact(issue):
    score = 0
    
    # Week 1 목표 차단?
    if blocks_demo():
        score += 100  # Critical
    
    # 현재 작업 차단?
    if blocks_current_task():
        score += 50   # High
    
    # LLM 파이프라인 영향?
    if breaks_automation():
        score += 200  # Blocker!
    
    # 영향 범위
    affected_files = count_affected_files()
    score += affected_files * 5
    
    return score
```

**예시:**
```
Issue: "Variant constructors not in scope"
- Blocks demo? ✅ (player.pole-ir 실행 불가) → +100
- Blocks current? ✅ (Day 5 type checker 완성 못함) → +50
- Breaks automation? ❌ (Python fallback 존재) → +0
- Affected files: 2 (player, zombie) → +10
Total Impact: 160 (HIGH)
```

---

### 2. 긴급도 분석 (Urgency Analysis)

**질문:**
- Deadline이 언제인가?
- 다른 작업의 선행 조건인가?
- 얼마나 빨리 악화될 수 있는가?

**점수 계산:**
```python
def calculate_urgency(issue, current_date, deadline):
    score = 0
    
    # Deadline까지 남은 시간
    days_left = (deadline - current_date).days
    if days_left <= 1:
        score += 100  # 내일까지
    elif days_left <= 3:
        score += 50   # 3일 이내
    else:
        score += 10   # 여유 있음
    
    # Blocking 관계
    if blocks_other_tasks():
        blocked_count = count_blocked_tasks()
        score += blocked_count * 20
    
    # 악화 가능성
    if can_worsen():
        score += 30
    
    return score
```

**예시:**
```
Issue: "Variant constructors not in scope"
- Days to Week 1 deadline: 5일 → +10
- Blocks other tasks: 2개 (zombie test, integration) → +40
- Can worsen: ❌ (isolated issue) → +0
Total Urgency: 50 (MEDIUM)
```

---

### 3. 복잡도 분석 (Complexity Analysis)

**질문:**
- 구현하는 데 얼마나 걸리는가?
- 위험도는 얼마나 되는가?
- 필요한 지식 수준은?

**점수 계산:**
```python
def calculate_complexity(issue):
    # 낮을수록 좋음 (쉬운 작업)
    score = 0
    
    # 예상 시간
    estimated_hours = estimate_time(issue)
    score += estimated_hours * 10
    
    # 위험도
    if requires_architecture_change():
        score += 50  # High risk
    elif touches_core_system():
        score += 30  # Medium risk
    else:
        score += 10  # Low risk
    
    # 지식 요구 수준
    if needs_deep_knowledge():
        score += 20
    
    return score
```

**예시:**
```
Issue: "Variant constructors not in scope"
- Estimated time: 1.5시간 → +15
- Risk: Core type checker (medium) → +30
- Knowledge: Rust + type theory (medium) → +20
Total Complexity: 65 (MEDIUM)
```

---

### 4. ROI 계산 (Return on Investment)

**공식:**
```python
def calculate_priority(issue):
    impact = calculate_impact(issue)      # 높을수록 좋음
    urgency = calculate_urgency(issue)    # 높을수록 좋음
    complexity = calculate_complexity(issue)  # 낮을수록 좋음
    
    # ROI = (영향도 × 긴급도) / 복잡도
    roi = (impact * urgency) / max(complexity, 1)
    
    # Priority tier 결정
    if roi > 200:
        return "P0"  # Do now
    elif roi > 50:
        return "P1"  # Do soon
    else:
        return "P2"  # Do later
```

**예시:**
```
Issue: "Variant constructors not in scope"
ROI = (160 × 50) / 65 = 123
Priority: P1 (Do soon)

Issue: "Full bidirectional type checking"
Impact: 80 (nice to have)
Urgency: 10 (no deadline pressure)
Complexity: 200 (2 weeks work)
ROI = (80 × 10) / 200 = 4
Priority: P2 (Do later)
```

---

## 🤖 자동 의사결정 로직

### Decision Tree

```python
def auto_decide_action(issue):
    priority = calculate_priority(issue)
    context = get_current_context()
    
    if priority == "P0":
        # Blocker - 즉시 처리
        if context.in_middle_of_task:
            action = "PAUSE_AND_SWITCH"
            reason = "Critical blocker detected"
        else:
            action = "DO_NOW"
            reason = "P0 issue blocks progress"
    
    elif priority == "P1":
        # Important - 판단 필요
        if context.current_day_almost_done:
            action = "DO_NOW"
            reason = "Can complete before day end"
        elif issue.blocks_next_task:
            action = "SCHEDULE_NEXT"
            reason = "Prerequisite for upcoming work"
        else:
            action = "SCHEDULE_END_OF_WEEK"
            reason = "Important but not blocking"
    
    elif priority == "P2":
        # Optional - 미루기
        action = "DEFER_TO_WEEK2"
        reason = "Low ROI, can wait"
    
    return {
        "action": action,
        "priority": priority,
        "reason": reason,
        "estimated_time": issue.complexity / 10,  # hours
    }
```

---

## 📋 자동 TODO 재배치

### Before (수동)
```markdown
## TODO
- [ ] Continue Day 5 - Pole Engine refactoring
- [ ] Write documentation
```

### After (자동)
```markdown
## TODO

### 🔴 URGENT (P0)
(None)

### 🟡 HIGH PRIORITY (P1)
- [ ] **[NEW]** Fix variant constructors (1-2h) - Blocks player/zombie tests
- [ ] **[NEW]** Fix record type inference (1-2h) - Blocks LLM generated code

### 🟢 CURRENT PLAN
- [ ] Continue Day 5 - Pole Engine refactoring (pushed to afternoon)
- [ ] Write documentation (pushed to end of day)

### ⚪ DEFERRED (P2)
- [ ] Let expression edge cases (Week 2)
- [ ] Full bidirectional type checking (Week 2+)
```

---

## 🔄 자동화 워크플로우

### Trigger: LLM이 에러 발견 시

```python
# 1. 자동 분석
issue = analyze_error(error_message)
decision = auto_decide_action(issue)

# 2. 문서 자동 생성
create_decision_doc(issue, decision)  # .claude/DECISION.md
update_pending_issues(issue, decision)  # .claude/PENDING_ISSUES.md
update_summary(issue, decision)  # .claude/SUMMARY.md

# 3. TODO 자동 재배치
current_todos = todoread()
new_todos = reorder_todos(current_todos, issue, decision)
todowrite(new_todos)

# 4. 사람에게 알림 (간단히)
notify_user(f"""
새로운 {decision.priority} 이슈 발견:
  {issue.title}
  
자동 결정:
  → {decision.action}
  → 예상 시간: {decision.estimated_time}h
  → 이유: {decision.reason}

상세: .claude/DECISION.md 참조
계속 진행할까요? (자동 실행 10초 후)
""")

# 5. 자동 실행 (또는 대기)
if decision.action == "DO_NOW":
    if AUTO_EXECUTE:
        execute_immediately()
    else:
        wait_for_approval(timeout=10)  # 10초 후 자동 실행
```

---

## 📝 자동 문서 생성 템플릿

### .claude/DECISION.md (자동 생성)

```markdown
# Auto-Decision: {issue.title}

**Date:** {current_date}
**Decision:** {decision.action}
**Priority:** {decision.priority}
**Auto-generated:** Yes

---

## Analysis

### Impact Score: {impact}/200
- Blocks demo: {yes/no}
- Blocks current task: {yes/no}
- Breaks automation: {yes/no}
- Affected files: {count}

### Urgency Score: {urgency}/100
- Days to deadline: {days}
- Blocks other tasks: {count}
- Can worsen: {yes/no}

### Complexity Score: {complexity}/100
- Estimated time: {hours}h
- Risk level: {low/medium/high}
- Knowledge required: {basic/medium/advanced}

### ROI Calculation
```
ROI = ({impact} × {urgency}) / {complexity}
    = {roi}
    → Priority: {priority}
```

---

## Decision

**Action:** {action}
**Reasoning:**
{auto_generated_reasoning}

**Alternatives Considered:**
1. Do now → ROI={roi1}
2. Do later → ROI={roi2}
3. Skip → Impact too high

**Selected:** Option 1 (Do now)

---

## Execution Plan

{auto_generated_plan}

---

**Human Approval:** {required/optional}
**Auto-execute in:** {countdown}s
```

---

## 🎛️ 설정 (Configuration)

### .claude/config.yml (새 파일)

```yaml
auto_priority:
  enabled: true
  
  # 자동 실행 임계값
  auto_execute:
    p0: true   # P0는 자동 실행
    p1: false  # P1은 승인 대기
    p2: false  # P2는 승인 필요
  
  # 승인 대기 시간 (초)
  approval_timeout:
    p0: 5
    p1: 10
    p2: 30
  
  # ROI 임계값
  thresholds:
    p0_min: 200
    p1_min: 50
    p2_max: 50
  
  # 가중치 조정
  weights:
    impact: 1.0
    urgency: 1.0
    complexity: 1.0
  
  # 알림 설정
  notifications:
    verbose: false  # false면 간단히만
    write_files: true  # 문서 자동 생성
```

---

## 🚀 구현 단계

### Phase 1: 분석 엔진 (1-2h)
```python
# scripts/auto_priority.py

class IssueAnalyzer:
    def analyze(self, issue, context):
        impact = self.calculate_impact(issue, context)
        urgency = self.calculate_urgency(issue, context)
        complexity = self.calculate_complexity(issue)
        roi = self.calculate_roi(impact, urgency, complexity)
        priority = self.determine_priority(roi)
        
        return AnalysisResult(
            impact=impact,
            urgency=urgency,
            complexity=complexity,
            roi=roi,
            priority=priority
        )
```

### Phase 2: 의사결정 엔진 (1h)
```python
class DecisionEngine:
    def decide(self, analysis, context):
        if analysis.priority == "P0":
            return Decision(
                action="DO_NOW",
                reason="Critical blocker",
                auto_execute=True
            )
        # ...
```

### Phase 3: 문서 생성기 (1h)
```python
class DocGenerator:
    def generate_decision_doc(self, issue, analysis, decision):
        template = load_template("decision.md")
        content = template.render(
            issue=issue,
            analysis=analysis,
            decision=decision
        )
        write_file(".claude/DECISION.md", content)
```

### Phase 4: CLAUDE.md 통합 (30m)
```markdown
<!-- CLAUDE.md에 추가 -->

## 🤖 Auto Priority System

When you encounter a new issue:

1. **Analyze automatically:**
   ```bash
   python scripts/auto_priority.py analyze "issue description"
   ```

2. **Get recommendation:**
   - Priority: P0/P1/P2
   - Action: DO_NOW / SCHEDULE / DEFER
   - Estimated time
   - Reasoning

3. **Execute or wait:**
   - P0: Auto-execute after 5s
   - P1: Wait for approval (10s)
   - P2: Document only

4. **Update TODO list:**
   - Reorder based on priority
   - Add time estimates
   - Mark dependencies
```

---

## 📊 성공 지표

### Before Auto-Priority
- 문제 발견 → 사람 질문 → 회의 → 결정 (30분+)
- 우선순위 일관성: 중간 (사람 판단 의존)
- 문서화: 가끔 빠짐

### After Auto-Priority
- 문제 발견 → 자동 분석 → 자동 결정 (10초)
- 우선순위 일관성: 높음 (명확한 공식)
- 문서화: 100% (자동 생성)

---

## 🎯 예상 효과

### 시간 절약
- 의사결정 시간: 30분 → 10초 (99% 감소)
- 문서 작성 시간: 10분 → 0분 (자동)

### 일관성 향상
- ROI 기반 객관적 판단
- 사람의 기분/피로 영향 제거

### LLM 자율성 향상
- 90% 케이스 자동 처리
- 사람은 review만

---

## ⚠️ 주의사항

### 언제 자동화하면 안 되는가?

1. **Architecture 결정** - 사람 판단 필요
2. **보안 관련** - 신중한 검토 필요
3. **External dependency 추가** - 승인 필요

### Fallback

자동 시스템이 확신 없을 때:
```python
if confidence < 0.7:
    return Decision(
        action="ASK_HUMAN",
        reason="Uncertain - multiple factors conflict",
        options=[option1, option2, option3]
    )
```

---

## 📚 참고 자료

- `.claude/ERROR_RECOVERY.md` - 기존 에러 처리 프로토콜
- `scripts/auto_development.py` - 기존 자동화 스크립트
- `docs/guides/AUTOMATION_GUIDE.md` - 자동화 철학

---

**Status:** Design complete, ready to implement  
**Next Step:** Implement Phase 1 (Analysis Engine)
