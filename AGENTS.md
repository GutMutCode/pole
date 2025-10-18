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

**Never assume priority** - always verify against the roadmap and ask the user.

## Before Starting Any Task (Critical Checklist)

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
