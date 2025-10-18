import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from pole.parser.ast_nodes import FunctionDef, Specification, TypeDef  # type: ignore

from .llm_client import LLMClient, MockLLMClient, OpenRouterClient


class SpecificationTransformer:
    def __init__(self, llm_client: LLMClient | None = None):
        self.llm_client = llm_client or MockLLMClient()

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

        ir_code = self.llm_client.complete(prompt, system_prompt)
        return self._clean_ir_code(ir_code)

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

Pole IR Syntax Rules:
1. Function definition:
   func <name> (param: Type, ...) -> ReturnType
     requires <precondition>
     ensures <postcondition>
   :
     <body>

2. Type definition:
   type <Name> = {
     field1: Type1,
     field2: Type2
   }
   
   or
   
   type <Name> =
     | Constructor1(Type1)
     | Constructor2(Type2)

3. Annotations:
   @source("file.pole", line=X)
   @test_case(input=X, expected=Y)
   @generated_from("description")
   @reasoning("explanation")

4. Types:
   - Int, Nat, Float64, Bool, String, Unit
   - Option<T>, Result<T, E>, List<T>
   - (T1, T2) for tuples
   - {field: Type} for records

5. Expressions:
   - Literals: 42, 3.14, true, "hello", ()
   - if...then...else
   - match...with | Pattern -> Expr
   - let x = e1 in e2
   - \x -> e (lambda)

Important:
- Be precise and type-safe
- Infer appropriate types from descriptions
- Generate correct requires/ensures clauses
- Choose efficient algorithms
- Output ONLY the Pole IR code, no explanations
- Do NOT include markdown code blocks (no ```), just raw IR code
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


def transform_specification(
    spec: Specification,
    source_file: str = "unknown.pole",
    llm_client: LLMClient | None = None,
) -> str:
    transformer = SpecificationTransformer(llm_client)
    return transformer.transform(spec, source_file)
