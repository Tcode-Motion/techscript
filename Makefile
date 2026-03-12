.PHONY: install dev test lint clean build pyinstaller help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

install: ## Install TechScript (user)
	pip install .

dev: ## Install TechScript in editable mode with dev deps
	pip install -e ".[dev]"

test: ## Run all tests
	python -m pytest tests/ -v

test-cov: ## Run tests with coverage
	python -m pytest tests/ -v --cov=techscript --cov-report=term-missing

lint: ## Lint source code
	python -m flake8 src/techscript/ --max-line-length=120 --ignore=E501,W503

fmt: ## Auto-format code with black
	python -m black src/ tests/

build: ## Build distributable package
	python -m build

clean: ## Remove build artifacts
	rm -rf build/ dist/ *.egg-info src/*.egg-info __pycache__ .pytest_cache .coverage htmlcov
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true

pyinstaller: ## Build single binary with PyInstaller
	pip install pyinstaller
	pyinstaller --onefile --name tech src/techscript/__main__.py

run-hello: ## Run the hello example
	python -m techscript run examples/hello.txs

repl: ## Start REPL
	python -m techscript repl
