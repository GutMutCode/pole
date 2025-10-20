#!/usr/bin/env python3
"""
Auto Priority Management System

자동으로 새로운 이슈의 우선순위를 분석하고 TODO 리스트를 재배치합니다.

Usage:
    python scripts/auto_priority.py analyze "Variant constructors not in scope"
    python scripts/auto_priority.py reorder
"""

import sys
import os
from dataclasses import dataclass
from datetime import datetime, timedelta
from pathlib import Path

# Add src to path
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))


@dataclass
class IssueAnalysis:
    """이슈 분석 결과"""

    impact: int  # 0-200
    urgency: int  # 0-100
    complexity: int  # 0-200 (낮을수록 좋음)
    roi: float  # ROI 점수
    priority: str  # P0/P1/P2
    reasoning: str


@dataclass
class Decision:
    """자동 의사결정 결과"""

    action: str  # DO_NOW / SCHEDULE_NEXT / DEFER
    reason: str
    estimated_hours: float
    auto_execute: bool


class IssueAnalyzer:
    """이슈 분석 엔진"""

    def __init__(self, config=None):
        self.config = config or self._default_config()

    def _default_config(self):
        return {
            "deadline": datetime(2025, 10, 26),  # Week 1 deadline
            "p0_threshold": 200,
            "p1_threshold": 50,
            "weights": {
                "impact": 1.0,
                "urgency": 1.0,
                "complexity": 1.0,
            },
        }

    def analyze(self, issue_desc: str, context: dict | None = None) -> IssueAnalysis:
        """
        이슈를 분석하여 우선순위를 결정

        Args:
            issue_desc: 이슈 설명
            context: 현재 컨텍스트 (current_task, day 등)

        Returns:
            IssueAnalysis 객체
        """
        context = context or {}

        # 1. 영향도 분석
        impact = self._calculate_impact(issue_desc, context)

        # 2. 긴급도 분석
        urgency = self._calculate_urgency(issue_desc, context)

        # 3. 복잡도 분석
        complexity = self._calculate_complexity(issue_desc)

        # 4. ROI 계산
        roi = self._calculate_roi(impact, urgency, complexity)

        # 5. 우선순위 결정
        priority = self._determine_priority(roi)

        # 6. 근거 생성
        reasoning = self._generate_reasoning(impact, urgency, complexity, roi, priority)

        return IssueAnalysis(
            impact=impact,
            urgency=urgency,
            complexity=complexity,
            roi=roi,
            priority=priority,
            reasoning=reasoning,
        )

    def _calculate_impact(self, issue: str, context: dict) -> int:
        """영향도 계산 (0-200)"""
        score = 0
        issue_lower = issue.lower()

        # Week 1 데모 차단?
        if any(
            keyword in issue_lower
            for keyword in [
                "blocks demo",
                "cannot run",
                "execution fails",
                "player",
                "zombie",
                "game loop",
            ]
        ):
            score += 100

        # 현재 작업 차단?
        if context.get("blocks_current_task"):
            score += 50
        elif any(keyword in issue_lower for keyword in ["type check fails", "compilation error"]):
            score += 30

        # LLM 파이프라인 영향?
        if any(keyword in issue_lower for keyword in ["llm generation", "automation", "pipeline"]):
            score += 200  # Critical!

        # 영향 범위 (파일 개수 추정)
        if "all files" in issue_lower:
            score += 50
        elif "multiple files" in issue_lower:
            score += 20
        elif "one file" in issue_lower or "single" in issue_lower:
            score += 5

        return min(score, 200)  # Cap at 200

    def _calculate_urgency(self, issue: str, context: dict) -> int:
        """긴급도 계산 (0-100)"""
        score = 0

        # Deadline까지 남은 시간
        today = datetime.now()
        deadline = self.config["deadline"]
        days_left = (deadline - today).days

        if days_left <= 1:
            score += 100
        elif days_left <= 3:
            score += 50
        elif days_left <= 5:
            score += 20
        else:
            score += 10

        # 다른 작업을 차단하는가?
        if context.get("blocks_other_tasks"):
            blocked_count = context.get("blocked_task_count", 1)
            score += min(blocked_count * 20, 40)

        # 악화 가능성
        if any(keyword in issue.lower() for keyword in ["getting worse", "spreading", "cascading"]):
            score += 30

        return min(score, 100)

    def _calculate_complexity(self, issue: str) -> int:
        """복잡도 계산 (0-200, 낮을수록 좋음)"""
        score = 0
        issue_lower = issue.lower()

        # 예상 시간 추정 (키워드 기반)
        if "simple" in issue_lower or "quick fix" in issue_lower:
            hours = 0.5
        elif "add" in issue_lower or "register" in issue_lower:
            hours = 1
        elif "fix" in issue_lower or "update" in issue_lower:
            hours = 2
        elif "implement" in issue_lower or "create" in issue_lower:
            hours = 4
        elif "redesign" in issue_lower or "refactor" in issue_lower:
            hours = 8
        elif "bidirectional" in issue_lower or "full" in issue_lower:
            hours = 16
        else:
            hours = 2  # Default

        score += int(hours * 10)

        # 위험도
        if any(
            keyword in issue_lower for keyword in ["architecture", "core system", "breaking change"]
        ):
            score += 50  # High risk
        elif any(keyword in issue_lower for keyword in ["type checker", "parser", "compiler"]):
            score += 30  # Medium risk
        else:
            score += 10  # Low risk

        # 지식 요구 수준
        if any(keyword in issue_lower for keyword in ["type theory", "algorithm", "optimization"]):
            score += 20  # Advanced
        elif any(keyword in issue_lower for keyword in ["implement", "design"]):
            score += 10  # Intermediate

        return min(score, 200)

    def _calculate_roi(self, impact: int, urgency: int, complexity: int) -> float:
        """ROI 계산"""
        w = self.config["weights"]
        numerator = (impact * w["impact"]) * (urgency * w["urgency"])
        denominator = max(complexity * w["complexity"], 1)
        return numerator / denominator

    def _determine_priority(self, roi: float) -> str:
        """ROI 기반 우선순위 결정"""
        if roi >= self.config["p0_threshold"]:
            return "P0"
        elif roi >= self.config["p1_threshold"]:
            return "P1"
        else:
            return "P2"

    def _generate_reasoning(
        self, impact: int, urgency: int, complexity: int, roi: float, priority: str
    ) -> str:
        """의사결정 근거 생성"""
        return f"""
Analysis:
- Impact: {impact}/200 ({self._score_label(impact, 200)})
- Urgency: {urgency}/100 ({self._score_label(urgency, 100)})
- Complexity: {complexity}/200 ({self._complexity_label(complexity)})
- ROI: {roi:.1f}

Priority: {priority}

Recommendation:
{self._get_recommendation(priority)}
"""

    def _score_label(self, score: int, max_score: int) -> str:
        """점수 레이블"""
        ratio = score / max_score
        if ratio >= 0.7:
            return "HIGH"
        elif ratio >= 0.4:
            return "MEDIUM"
        else:
            return "LOW"

    def _complexity_label(self, complexity: int) -> str:
        """복잡도 레이블 (낮을수록 좋음)"""
        if complexity >= 100:
            return "COMPLEX (hard)"
        elif complexity >= 50:
            return "MODERATE"
        else:
            return "SIMPLE (easy)"

    def _get_recommendation(self, priority: str) -> str:
        """우선순위별 추천 액션"""
        if priority == "P0":
            return "→ DO NOW: Critical blocker, stop current work and fix immediately"
        elif priority == "P1":
            return "→ SCHEDULE SOON: Important, complete within 1-2 days"
        else:
            return "→ DEFER: Low priority, can wait until Week 2+"


class DecisionEngine:
    """자동 의사결정 엔진"""

    def decide(self, analysis: IssueAnalysis, context: dict | None = None) -> Decision:
        """
        분석 결과를 바탕으로 액션 결정

        Args:
            analysis: IssueAnalysis 객체
            context: 현재 컨텍스트

        Returns:
            Decision 객체
        """
        context = context or {}

        if analysis.priority == "P0":
            return Decision(
                action="DO_NOW",
                reason="Critical blocker - stops progress",
                estimated_hours=analysis.complexity / 10,
                auto_execute=True,
            )

        elif analysis.priority == "P1":
            # Current task가 거의 끝났으면 지금, 아니면 다음으로
            if context.get("current_task_progress", 0) > 0.8:
                action = "DO_NOW"
                reason = "Current task almost done, can fit this in"
            else:
                action = "SCHEDULE_NEXT"
                reason = "Important but not blocking, schedule for next slot"

            return Decision(
                action=action,
                reason=reason,
                estimated_hours=analysis.complexity / 10,
                auto_execute=False,  # Needs approval
            )

        else:  # P2
            return Decision(
                action="DEFER",
                reason="Low ROI - defer to Week 2+",
                estimated_hours=analysis.complexity / 10,
                auto_execute=False,
            )


def main():
    """CLI 진입점"""
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    command = sys.argv[1]

    if command == "analyze":
        if len(sys.argv) < 3:
            print("Usage: auto_priority.py analyze <issue_description>")
            sys.exit(1)

        issue_desc = sys.argv[2]

        # 분석 실행
        analyzer = IssueAnalyzer()
        analysis = analyzer.analyze(issue_desc)

        # 결과 출력
        print("\n" + "=" * 60)
        print("AUTO PRIORITY ANALYSIS")
        print("=" * 60)
        print(f"\nIssue: {issue_desc}")
        print(analysis.reasoning)

        # 의사결정
        engine = DecisionEngine()
        decision = engine.decide(analysis)

        print("\nDecision:")
        print(f"  Action: {decision.action}")
        print(f"  Reason: {decision.reason}")
        print(f"  Estimated time: {decision.estimated_hours:.1f}h")
        print(f"  Auto-execute: {decision.auto_execute}")
        print("\n" + "=" * 60)

    elif command == "reorder":
        print("TODO: Implement TODO list reordering")
        # Future: Read current TODOs, reorder based on priority

    else:
        print(f"Unknown command: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
