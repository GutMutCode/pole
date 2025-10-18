# Agent Guidelines for Pole Project

## Project Context
Pole is an LLM-optimized programming language system with two distinct layers:

1. **Specification Language** (.pole files) - Human-written, natural language friendly
2. **Implementation Language** (Pole IR) - LLM-generated, formal, type-safe

**IMPORTANT**: We are designing BOTH languages from scratch. Pole does NOT compile to existing languages like Rust or Python. See ARCHITECTURE.md for the complete pipeline.

## Build/Test Commands
- No build system yet - this is a design/specification phase project
- No tests yet - implementation has not started
- Git operations: standard git commands

## Code Style (When Implementation Begins)
- **Specification Language** (.pole): English keywords for LLM compatibility
- **Documentation**: Korean (한국어) for design docs, English for code/specs
- **Implementation Tool**: Python for transformer/compiler (빠른 프로토타이핑)
- **Format**: All documents should be LLM-friendly and parseable
- **Structure**: Follow the categorical organization in README.md
- **Naming**: English for all code and language constructs
- **Comments**: Avoid unless explicitly requested per project rules

## File Organization
- `specs/` - Language specifications (syntax-v0.md, implementation-lang.md)
- `examples/` - Example programs (*.pole for spec lang, *.pole-ir for IR)
- `src/` - Implementation code (transformer, compiler, verifier)
- Follow the priority system defined in README.md (P0/P1/P2)
- **Critical**: Never confuse "타겟 언어" with "구현 언어" - we design the IR ourselves

## Development Approach
- Focus on "what" and "why" over "how" in specifications
- Allow intentional ambiguity where LLM judgment is delegated
- Support incremental refinement and incomplete specifications
- Document examples, use cases, and edge cases
- Emphasize verification and formal correctness where applicable

## Task Priority Management

**Before starting any new work**, consult [ROADMAP.md](ROADMAP.md) to:
1. Check current highest priority task
2. Compare new task against existing priorities
3. Ask user to confirm priority if unclear
4. Update roadmap with new tasks

**Priority hierarchy**: Phase > P0/P1/P2 (Phase가 상위 개념)
- 현재 Phase 내 작업을 먼저 완료
- 같은 Phase 내에서 P0 → P1 → P2 순서

**Never assume priority** - always verify against the roadmap and ask the user.

### Task Status Verification

When the user asks about **current priority or next task**, always:
1. Read ROADMAP.md to identify highest priority task
2. **Verify completion** by checking deliverables in codebase
3. Report task, status, and next action

**Trigger patterns** (examples):
- "지금 가장 중요한 작업이 뭐야?"
- "다음에 뭐 해야해?"
- "현재 작업 상태는?"
- "뭐부터 시작하지?"
- "지금 해야할 작업이 뭘까?"
- Similar questions about current/next priority

**Key principle**: If the question is about **what to work on now**, always verify the codebase first.

**Report format**:
```
현재 최우선 작업: [Task ID] [Task Name] (P[0-3])

상태 확인:
- 산출물 파일 존재: [예/아니오]
- 완료도: [미시작/진행중/완료]
- 필요 작업: [구체적 항목들]

다음 단계: [즉시 수행할 작업]
```

## Before Starting Any Task (Critical Checklist)

**TRIGGER PATTERNS** - Activate this checklist when:
- User says: "작업 시작해줘", "진행해줘", "해줘", "시작해"
- User says: "다음 작업 해줘", "4.3 작업 해줘", "계속해줘"
- After reporting current task and user confirms to proceed
- ANY request to BEGIN implementation work

**DO NOT skip this checklist even if:**
- You already checked ROADMAP.md
- The task seems simple
- You feel confident about requirements
- User seems impatient

When starting a new task, you **MUST** follow this research process:

### 1. Identify the Task
- [ ] Read [ROADMAP.md](ROADMAP.md) to identify the current task
- [ ] Note the task number, description, and deliverables

### 2. Understand Requirements
- [ ] Read [README.md](README.md) - check if task-related requirements exist
  - Search for keywords related to the task
  - Read relevant sections thoroughly
- [ ] Read [ARCHITECTURE.md](ARCHITECTURE.md) - understand system context
  - How does this task fit into the overall architecture?
  - What are the dependencies and interfaces?

### 3. Review Existing Design
- [ ] Read all files in `specs/` directory
  - Check for existing design documents related to the task
  - Understand design decisions already made
- [ ] Review `examples/` if relevant
  - Understand concrete use cases

### 4. Search Codebase
- [ ] Use grep/search to find related keywords in all markdown files
- [ ] Check if similar work has been done before
- [ ] Look for related discussions or design notes

### 5. Synthesize Understanding
- [ ] List what is already defined vs. what needs to be created
- [ ] Identify gaps or ambiguities
- [ ] Note any conflicts or inconsistencies

### 6. Ask Before Proceeding
If you find:
- Sufficient information exists → Proceed with confidence
- Information is unclear → Ask specific questions
- Conflicts in documentation → Point them out and ask for clarification

**DO NOT skip this process**. Recommendations made without proper research waste time and erode trust.

## Working with Specifications (Critical Process)

When you receive a specification or requirement, **DO NOT** immediately start implementation. Follow this process:

### 1. Analyze First
Before writing any code, analyze the specification for:
- **Missing information**: Are all inputs, outputs, and types defined?
- **Ambiguity**: Are there terms that could be interpreted multiple ways?
- **Contradictions**: Do any requirements conflict with each other?
- **Incomplete examples**: Are edge cases and error conditions covered?
- **Unclear constraints**: Are performance/security requirements specific enough?

### 2. Ask Questions
If you find issues in step 1, **stop and ask the user**:
- List what is unclear or missing
- Provide specific questions with options when possible
- Explain trade-offs for each option
- Ask for concrete examples if needed

**Example**:
```
I found the following issues in the specification:

[Missing] Error handling strategy not specified
Question: How should the function handle invalid input?
Options:
  1. Return error type (safe, explicit)
  2. Throw exception (conventional, but less type-safe)
  3. Use default value (convenient, but may hide bugs)

[Ambiguous] "process efficiently" 
Question: What is the priority?
  - Speed (optimize for performance)
  - Memory (optimize for space)
  - Code clarity (optimize for maintainability)
```

### 3. Proceed Only When Clear
Only start implementation when:
- All required information is present
- Ambiguities are either resolved OR intentionally delegated to your judgment
- You can explain your implementation choices

### Specification Clarity Checklist

Use this checklist when receiving a specification:

#### For Functions:
- [ ] Purpose clearly stated?
- [ ] All input parameters defined with types?
- [ ] Output type defined?
- [ ] Constraints on input values specified?
- [ ] Error conditions listed?
- [ ] At least one normal case example?
- [ ] Edge case examples provided?
- [ ] Performance requirements clear (if any)?

#### For Types:
- [ ] All fields defined with types?
- [ ] Field constraints specified?
- [ ] Validation rules clear?

#### For System Changes:
- [ ] Goal/motivation explained?
- [ ] Success criteria defined?
- [ ] Failure cases considered?
- [ ] Impact on existing code assessed?

### When in Doubt
- **Ask, don't assume**
- **Be specific** in your questions
- **Provide options** with trade-offs
- **Document decisions** made in absence of clarity

See [specs/workflow.md](specs/workflow.md) for the complete LLM transformation workflow.

---

## Self-Improvement Protocol

When you encounter confusion, misunderstanding, or make a mistake during conversation:

### 1. Identify the Root Cause

Analyze what went wrong:
- **Documentation unclear**: Was existing documentation ambiguous or incomplete?
- **Rule missing**: Was there no guidance for this situation?
- **Information misplaced**: Was information in the wrong file or section?
- **Assumption made**: Did you assume something not explicitly stated?

### 2. Propose Documentation Fix

Immediately after identifying the issue, propose a specific fix:

```
[Documentation Improvement Needed]

Issue: [What confusion or error occurred]
Root cause: [Why it happened - missing rule, unclear docs, etc.]
Proposed fix: 
  - File: [Which file to update - AGENTS.md, ROADMAP.md, etc.]
  - Location: [Which section]
  - Change: [Specific text to add/modify]
  - Rationale: [Why this will prevent future errors]

이 변경을 적용할까요?
```

### 3. Ask User for Approval

Never apply documentation changes without user consent. Present the proposal and wait for approval.

### 4. Apply the Fix

If user approves, immediately update the relevant documentation file(s).

### Trigger Patterns

Activate this protocol when:
- User corrects your understanding
- User asks "다음부터는 어떻게 해야할까?" or similar
- User asks "이런 오해가 발생하지 않으려면..."
- User asks "이를 자동화하려면..."
- User points out confusion or asks for clarification of your behavior
- You realize you made an incorrect assumption

### Examples

<example>
User: "Phase가 P0/P1보다 상위 개념 아니야?"
Assistant: [Realizes misunderstanding]

[Documentation Improvement Needed]

Issue: Confused priority hierarchy - thought P0 takes precedence over Phase
Root cause: ROADMAP.md didn't explicitly state Phase > P0/P1/P2 hierarchy
Proposed fix:
  - File: ROADMAP.md
  - Location: "우선순위 원칙" section (top)
  - Change: Add "작업 선택 규칙" subsection explaining Phase is higher priority than P0/P1/P2
  - Rationale: Makes hierarchy explicit, prevents future confusion about cross-phase priorities

이 변경을 적용할까요?
</example>

### Key Principles

- **Proactive**: Don't wait for user to ask "how to prevent this" - propose fixes immediately when you realize an error
- **Specific**: Provide exact file, section, and text changes
- **Explain**: Always explain why the fix will prevent future errors
- **Respect user**: Never update docs without approval
- **Learn**: Each fix makes the system better for future interactions

---

## Task Completion Protocol

When a task is fully completed (all todos marked complete, deliverables created):

### 1. Verify Completion

Check that:
- [ ] All deliverables created and verified
- [ ] ROADMAP.md updated with completion status
- [ ] All todos marked complete
- [ ] Documentation is consistent

### 2. Offer to Commit Changes

Ask the user:
```
작업이 완료되었습니다. Git 커밋을 생성할까요?

변경된 파일:
- [list of changed files]

제안 커밋 메시지:
"[proposed commit message]"
```

### 3. Create Commit (if approved)

When user approves:
1. Run `git status` and `git diff` to verify changes
2. Stage relevant files with `git add`
3. Create commit with meaningful message
4. **Never push** without explicit user permission

### 4. Commit Message Format

Follow the project's commit style:
- Check recent commits with `git log`
- Use clear, concise messages
- Focus on "what" and "why"
- Reference task numbers if applicable

Example:
```
Complete task 1.4: Add verification system requirements

- Add specs/verification.md with type checker, compliance verification, and test generation strategy
- Update ROADMAP.md to mark 1.4 as complete
```

### When NOT to Offer Commit

- User is in middle of reviewing changes
- Task partially complete
- User explicitly said they'll commit manually
- Changes are experimental/temporary

---
