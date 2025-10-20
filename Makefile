.PHONY: help install dev-install test lint format typecheck clean update-priority

help:
	@echo "Pole Development Commands:"
	@echo "  make install         - Install production dependencies"
	@echo "  make dev-install     - Install development dependencies"
	@echo "  make test            - Run tests with pytest"
	@echo "  make lint            - Run linter (ruff)"
	@echo "  make format          - Format code (black + ruff)"
	@echo "  make typecheck       - Run type checker (mypy)"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make update-priority - Update CLAUDE.md with today's priority"

install:
	pip install -e .

dev-install:
	pip install -e ".[dev]"

test:
	pytest tests/ -v --cov=src/pole --cov-report=term-missing

lint:
	ruff check src/ tests/

format:
	black src/ tests/
	ruff check --fix src/ tests/

typecheck:
	mypy src/

clean:
	rm -rf build/ dist/ *.egg-info
	rm -rf .pytest_cache .mypy_cache .ruff_cache
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

update-priority:
	@python3 scripts/update_claude_priority.py
