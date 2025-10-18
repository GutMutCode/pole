import os
from typing import Protocol, Any


class LLMClient(Protocol):
    def complete(self, prompt: str, system_prompt: str | None = None) -> str: ...


class OpenRouterClient:
    def __init__(self, api_key: str | None = None, model: str = "anthropic/claude-3.5-sonnet"):
        self.api_key = api_key or os.getenv("OPENROUTER_API_KEY")
        if not self.api_key:
            raise ValueError(
                "OpenRouter API key not found. "
                "Set OPENROUTER_API_KEY environment variable or pass api_key parameter."
            )
        self.model = model
        self._client: Any = None

    def _get_client(self) -> Any:
        if self._client is None:
            try:
                from openai import OpenAI  # type: ignore
            except ImportError:
                raise ImportError("openai package not installed. Run: pip install openai")

            self._client = OpenAI(
                base_url="https://openrouter.ai/api/v1",
                api_key=self.api_key,
            )
        return self._client

    def complete(self, prompt: str, system_prompt: str | None = None) -> str:
        client = self._get_client()

        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})

        response = client.chat.completions.create(
            model=self.model,
            messages=messages,
        )

        return response.choices[0].message.content or ""


class MockLLMClient:
    """Mock LLM client for testing without API calls"""

    def __init__(self):
        self.mock_response = None

    def complete(self, prompt: str, system_prompt: str | None = None) -> str:
        if self.mock_response:
            return self.mock_response

        if "Type Definition" in prompt:
            if "User" in prompt:
                return """
type User = {
  name: String,
  age: Int,
  email: String
}
""".strip()
            return """
type ExampleType = {
  field1: String,
  field2: Int
}
""".strip()

        if "factorial" in prompt.lower():
            return """
@source("test.pole", line=1)
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  if n == 0 then
    1
  else
    n * factorial(n - 1)
""".strip()

        if "divide" in prompt.lower():
            return """
@source("test.pole", line=1)
@test_case(input=(10, 2), expected=5)
func divide (numerator: Float64, denominator: Float64) -> Float64
  requires denominator != 0.0
  ensures result == numerator / denominator
:
  numerator / denominator
""".strip()

        return """
@source("test.pole", line=1)
func test_function (x: Int) -> Int
  requires x >= 0
:
  x + 1
""".strip()
