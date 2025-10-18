import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from pole.parser.ast_nodes import FunctionDef, Specification, TypeDef
from pole.transformer.ir_postprocessor import IRPostprocessor  # type: ignore

from .llm_client import LLMClient, MockLLMClient, OpenRouterClient


class SpecificationTransformer:
    def __init__(self, llm_client: LLMClient | None = None):
        self.llm_client = llm_client or MockLLMClient()
        self.postprocessor = IRPostprocessor()

    def transform(self, spec: Specification, source_file: str = "unknown.pole") -> str:
        ir_parts = []

        for type_def in spec.types:
            ir_parts.append(self._transform_type(type_def, source_file))

        for func in spec.functions:
            ir_parts.append(self._transform_function(func, source_file))

        return "\n\n".join(ir_parts)

    def _transform_type(self, type_def: TypeDef, source_file: str) -> str:
        prompt = self._build_type_prompt(type_def)
        system_prompt = self._get_system_prompt()

        ir_code = self.llm_client.complete(prompt, system_prompt)
        return self._clean_ir_code(ir_code)

    def _transform_function(self, func: FunctionDef, source_file: str) -> str:
        prompt = self._build_function_prompt(func, source_file)
        system_prompt = self._get_system_prompt()

        max_retries = 2
        ir_code = ""
        for attempt in range(max_retries + 1):
            ir_code = self.llm_client.complete(prompt, system_prompt)
            ir_code = self._clean_ir_code(ir_code)

            result = self.postprocessor.process(ir_code)
            if result.success:
                return result.ir_code

            ir_code = result.ir_code

            if attempt < max_retries:
                error_context = (
                    f"\n\nParsing error: {result.parse_error}" if result.parse_error else ""
                )
                prompt = self._build_retry_prompt(func, source_file, ir_code, error_context)

        return ir_code

    def _build_type_prompt(self, type_def: TypeDef) -> str:
        prompt_parts = [
            f"# Type Definition",
            f"",
            f"Name: {type_def.name}",
            f"",
            f"Fields:",
        ]

        for field in type_def.fields:
            desc = f" - {field.description}" if field.description else ""
            prompt_parts.append(f"  - {field.name}: {field.type_annotation}{desc}")

        prompt_parts.extend(
            [
                "",
                "Convert this to Pole IR type definition.",
                "Follow the Pole IR syntax exactly.",
            ]
        )

        return "\n".join(prompt_parts)

    def _build_function_prompt(self, func: FunctionDef, source_file: str) -> str:
        prompt_parts = [
            f"# Function Specification",
            f"",
            f"Name: {func.name}",
        ]

        if func.purpose:
            prompt_parts.extend(
                [
                    f"Purpose: {func.purpose}",
                    "",
                ]
            )

        if func.input_desc:
            prompt_parts.extend(
                [
                    f"Input: {func.input_desc}",
                ]
            )

        if func.output_desc:
            prompt_parts.extend(
                [
                    f"Output: {func.output_desc}",
                    "",
                ]
            )

        if func.constraints:
            prompt_parts.append("Constraints:")
            for constraint in func.constraints:
                prompt_parts.append(f"  - {constraint}")
            prompt_parts.append("")

        if func.examples:
            prompt_parts.append("Examples:")
            for example in func.examples:
                prompt_parts.append(f"  - {example.input_desc} → {example.output_desc}")
            prompt_parts.append("")

        if func.notes:
            prompt_parts.append("Notes:")
            for note in func.notes:
                prompt_parts.append(f"  - {note}")
            prompt_parts.append("")

        prompt_parts.extend(
            [
                "Convert this specification to Pole IR code.",
                "Follow the Pole IR syntax exactly.",
                f'Include @source("{source_file}") annotation.',
                "Include @test_case annotations for each example.",
                "Add requires/ensures clauses based on constraints.",
                "Generate efficient, type-safe implementation.",
            ]
        )

        return "\n".join(prompt_parts)

    def _get_system_prompt(self) -> str:
        return r"""You are a Pole language compiler. Your task is to convert Pole specifications (.pole files) into Pole IR (Implementation Language).

Pole IR is a MINIMAL functional language. You MUST follow these constraints strictly.

=== SUPPORTED FEATURES ===

1. Function definition:
   func name(param: Type) -> ReturnType
     requires <precondition>
     ensures <postcondition>
   :
     <body>

2. Types:
   - Basic: Int, Nat, Float64, Bool, String, Unit
   - Compound: Option<T>, Result<T,E>, List<T>, (T1,T2)

3. Expressions:
   - Literals: 42, true, "hello"
   - if cond then expr1 else expr2  (nested if allowed)
   - match expr with | Pattern -> Expr
   - let x = value in body  (simple binding only)
   - Function calls: func(arg) or func (arg)
   - Operators: +, -, *, /, %, ==, !=, <, >, <=, >=, &&, ||

4. Patterns (in match):
   - Literals: 0, true, "hello"
   - Variables: x, n
   - Constructors: Some(x), None, Ok(v), Err(e)
   - Wildcard: _

5. Recursion:
   - Functions can call themselves
   - Use pattern matching for recursive cases

=== NOT SUPPORTED (DO NOT USE) ===

❌ Lambda functions: \x -> expr
❌ let rec: use func for recursion
❌ String/List methods: String.length(), String.char_at(), etc.
❌ List cons operator: x :: xs
❌ Method chaining: text.chars().fold()
❌ Higher-order functions in expressions: map, fold, filter
❌ Anonymous functions

=== IMPORTANT GUIDELINES ===

✓ For string operations: Use recursive functions only
✓ For complex logic: Break into multiple named functions
✓ For iteration: Use recursion with pattern matching
✓ Always use explicit types
✓ Keep it simple - don't assume library functions exist

=== EXAMPLES ===

Example 1 - Factorial (recursion with pattern matching):
```
func factorial(n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial(n - 1)
```

Example 2 - Max (simple conditional):
```
func max(a: Int, b: Int) -> Int
  requires true
  ensures result >= a && result >= b
:
  if a >= b then a else b
```

Example 3 - Nested if:
```
func classify(n: Int) -> String
  requires true
:
  if n == 0 then "zero" else if n > 0 then "positive" else "negative"
```

=== OUTPUT FORMAT ===

- Include @source, @test_case annotations
- Output ONLY Pole IR code, no explanations
- Do NOT use markdown code blocks
- Do NOT use unsupported features
"""

    def _clean_ir_code(self, ir_code: str) -> str:
        ir_code = ir_code.strip()

        if ir_code.startswith("```"):
            lines = ir_code.split("\n")
            if lines[0].strip().startswith("```"):
                lines = lines[1:]
            if lines and lines[-1].strip() == "```":
                lines = lines[:-1]
            ir_code = "\n".join(lines)

        return ir_code.strip()

    def _validate_ir_code(self, ir_code: str) -> bool:
        """Validate IR code by attempting to parse it"""
        if not ir_code or not ir_code.strip():
            return False

        try:
            import sys
            from pathlib import Path

            sys.path.insert(0, str(Path(__file__).parent.parent.parent))
            from pole.runtime.ir_parser import parse_ir  # type: ignore

            parse_ir(ir_code)
            return True
        except Exception:
            return False

    def _build_retry_prompt(
        self, func: FunctionDef, source_file: str, failed_code: str, error_context: str = ""
    ) -> str:
        """Build retry prompt with previous failure context"""
        original_prompt = self._build_function_prompt(func, source_file)
        return f"""{original_prompt}

IMPORTANT: Your previous attempt failed to parse.{error_context}

Common mistakes to avoid:
- Using lambda syntax (backslash x arrow) - NOT SUPPORTED
- Using let rec - NOT SUPPORTED  
- Using String/List methods - NOT SUPPORTED
- Using cons operator (double colon) - NOT SUPPORTED

Please generate simpler code using ONLY:
- func definitions for recursion
- if-then-else for conditionals
- match-with for pattern matching
- Basic operators: +, -, *, /, %, ==, <, >

Previous failed code (for reference):
{failed_code[:200]}...
"""


def transform_specification(
    spec: Specification,
    source_file: str = "unknown.pole",
    llm_client: LLMClient | None = None,
) -> str:
    transformer = SpecificationTransformer(llm_client)
    return transformer.transform(spec, source_file)
