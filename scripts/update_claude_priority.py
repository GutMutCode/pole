#!/usr/bin/env python3
"""
Update CLAUDE.md Current Priority section based on Week Plan

Usage:
    python scripts/update_claude_priority.py                    # Auto-detect current week/day
    python scripts/update_claude_priority.py --week 1 --day 2   # Manual override
    python scripts/update_claude_priority.py --dry-run          # Preview only
"""

import argparse
import re
from datetime import datetime, timedelta
from pathlib import Path


# Week start dates (Monday)
WEEK_STARTS = {
    1: datetime(2025, 10, 20),
    2: datetime(2025, 10, 27),
    3: datetime(2025, 11, 3),
    4: datetime(2025, 11, 10),
}

# Week 1 Day-by-Day tasks
WEEK1_TASKS = {
    1: {
        "day": "Monday",
        "title": "Player 명세 & 구현",
        "tasks": [
            "Write `games/zomboid/specs/player.pole` specification",
            "Generate IR with LLM (`pole build player.pole`)",
            "Test and verify (`pole test player.pole-ir`)",
        ],
    },
    2: {
        "day": "Tuesday",
        "title": "Zombie 명세 & 구현",
        "tasks": [
            "Write `games/zomboid/specs/zombie.pole` specification",
            "Generate IR with LLM (`pole build zombie.pole`)",
            "Test player + zombie integration",
        ],
    },
    3: {
        "day": "Wednesday",
        "title": "언어 개선 & 렌더링",
        "tasks": [
            "Fix Day 1-2 language issues",
            "Improve LLM prompts",
            "Write `pole_engine/render/sprite.pole` specification",
        ],
    },
    4: {
        "day": "Thursday",
        "title": "통합 데모 제작",
        "tasks": [
            "Write main game loop (`games/zomboid/main.pole`)",
            "Compile and debug",
            "Test native execution",
        ],
    },
    5: {
        "day": "Friday",
        "title": "Pole Engine 리팩토링",
        "tasks": [
            "Extract reusable code to `pole_engine/`",
            "Write documentation for engine modules",
            "Prepare for next week's development",
        ],
    },
    6: {
        "day": "Saturday",
        "title": "테스트 & 최적화",
        "tasks": [
            "Run full integration tests",
            "Profile and optimize performance",
            "Ensure 60 FPS with 1 player + 1 zombie",
        ],
    },
    7: {
        "day": "Sunday",
        "title": "데모 & 리뷰",
        "tasks": [
            "Record YouTube demo video (1 minute)",
            "Write Week 1 completion report",
            "Plan Week 2 priorities",
        ],
    },
}


def detect_current_week_day():
    """Auto-detect current week and day based on today's date"""
    today = datetime.now()

    for week_num, week_start in WEEK_STARTS.items():
        week_end = week_start + timedelta(days=6)
        if week_start <= today <= week_end:
            day_offset = (today - week_start).days
            return week_num, day_offset + 1  # Day 1-7

    # Default to Week 1, Day 1 if outside known range
    return 1, 1


def generate_priority_section(week: int, day: int) -> str:
    """Generate the Current Priority section content"""

    if week == 1:
        week_goal = "1-minute playable demo by 2025-10-26"
        tasks_map = WEEK1_TASKS
    else:
        # Placeholder for future weeks
        week_goal = f"Week {week} goals (see WEEK{week}_PLAN.md)"
        tasks_map = {}

    if day not in tasks_map:
        # Fallback for unknown days
        return f"""## 🎯 Current Priority (Week {week}, 2025-10-20)

**Active Phase:** Week {week} - Pole Zomboid Demo  
**Goal:** {week_goal}

### Today's Task (Day {day})
⚠️ No specific task defined. Check [docs/WEEK{week}_PLAN.md](docs/WEEK{week}_PLAN.md)

**Detailed Plan:** See [docs/WEEK{week}_PLAN.md](docs/WEEK{week}_PLAN.md)

### Priority Rules

**Hierarchy:** Week Plan > P0 > P1 > P2

- **P0**: Critical - Must complete this week
- **P1**: Important - After P0 completion
- **P2**: Optional - Time permitting

**Before starting work:**
1. ✅ Check this file (CLAUDE.md) for current priority
2. ✅ Read related guide (WEEK{week}_PLAN.md)
3. ✅ Confirm with user if unclear
"""

    day_info = tasks_map[day]
    tasks_list = "\n".join(
        f"{i}. {'⭐ ' if i == 1 else ''}**P0** {task}"
        for i, task in enumerate(day_info["tasks"], 1)
    )

    return f"""## 🎯 Current Priority (Week {week}, 2025-10-20)

**Active Phase:** Week {week} - Pole Zomboid Demo  
**Goal:** {week_goal}

### Today's Task (Day {day} - {day_info["day"]})
{tasks_list}

**Detailed Plan:** See [docs/WEEK{week}_PLAN.md](docs/WEEK{week}_PLAN.md)

### Priority Rules

**Hierarchy:** Week Plan > P0 > P1 > P2

- **P0**: Critical - Must complete this week
- **P1**: Important - After P0 completion
- **P2**: Optional - Time permitting

**Before starting work:**
1. ✅ Check this file (CLAUDE.md) for current priority
2. ✅ Read related guide (WEEK{week}_PLAN.md)
3. ✅ Confirm with user if unclear
"""


def update_claude_md(week: int, day: int, dry_run: bool = False) -> bool:
    """Update CLAUDE.md with new priority section"""

    claude_md_path = Path(__file__).parent.parent / "CLAUDE.md"

    if not claude_md_path.exists():
        print(f"❌ CLAUDE.md not found at {claude_md_path}")
        return False

    content = claude_md_path.read_text(encoding="utf-8")

    # Find the priority section
    pattern = r"## 🎯 Current Priority.*?(?=\n## )"
    new_section = generate_priority_section(week, day)

    if not re.search(pattern, content, re.DOTALL):
        print("❌ Priority section not found in CLAUDE.md")
        return False

    new_content = re.sub(pattern, new_section, content, flags=re.DOTALL)

    if dry_run:
        print("📄 Preview of changes:")
        print("=" * 80)
        print(new_section)
        print("=" * 80)
        print(f"✅ Dry run complete. No changes written.")
        return True

    claude_md_path.write_text(new_content, encoding="utf-8")
    print(
        f"✅ Updated CLAUDE.md for Week {week}, Day {day} ({WEEK1_TASKS.get(day, {}).get('day', 'Unknown')})"
    )
    return True


def main():
    parser = argparse.ArgumentParser(description="Update CLAUDE.md Current Priority section")
    parser.add_argument(
        "--week",
        type=int,
        help="Week number (1-4). Auto-detected if not provided.",
    )
    parser.add_argument(
        "--day",
        type=int,
        help="Day number (1-7). Auto-detected if not provided.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview changes without writing",
    )

    args = parser.parse_args()

    # Auto-detect if not provided
    if args.week is None or args.day is None:
        week, day = detect_current_week_day()
        if args.week is None:
            args.week = week
        if args.day is None:
            args.day = day
        print(f"🔍 Auto-detected: Week {args.week}, Day {args.day}")

    success = update_claude_md(args.week, args.day, args.dry_run)
    return 0 if success else 1


if __name__ == "__main__":
    exit(main())
