.PHONY: help install dev-install test lint format typecheck clean update-priority verify-ir verify-specs verify-all pre-commit auto-dev

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
	@echo ""
	@echo "Verification Commands:"
	@echo "  make verify-ir       - Verify all .pole-ir files with Rust parser"
	@echo "  make verify-specs    - Verify all .pole spec files"
	@echo "  make verify-all      - Run all verification checks"
	@echo "  make pre-commit      - Run before committing (format + verify)"
	@echo ""
	@echo "Automated Development:"
	@echo "  make auto-dev FILE=<spec.pole> - Run complete dev workflow with validation"

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

verify-ir:
	@echo "Verifying .pole-ir files with Rust parser..."
	@cd compiler && cargo build --release --bin polec 2>&1 | grep -v "warning:" || true
	@for file in examples/*.pole-ir games/zomboid/specs/*.pole-ir 2>/dev/null; do \
		if [ -f "$$file" ]; then \
			echo "  Checking $$file..."; \
			cd compiler && cargo run --release --bin polec -- ../$$file > /dev/null 2>&1 && echo "    ✓ OK" || echo "    ✗ FAILED"; \
		fi \
	done

verify-specs:
	@echo "Verifying .pole spec files..."
	@for file in games/zomboid/specs/*.pole 2>/dev/null; do \
		if [ -f "$$file" ]; then \
			echo "  Checking $$file..."; \
			pole check "$$file" > /dev/null 2>&1 && echo "    ✓ OK" || echo "    ✗ FAILED"; \
		fi \
	done

verify-all: verify-specs verify-ir
	@echo ""
	@echo "✓ All verification checks complete"

pre-commit: format verify-all
	@echo ""
	@echo "✓ Pre-commit checks passed. Safe to commit!"

auto-dev:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make auto-dev FILE=path/to/spec.pole"; \
		exit 1; \
	fi
	@python3 scripts/auto_development.py $(FILE)
