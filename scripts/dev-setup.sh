#!/usr/bin/env bash
set -euo pipefail
python3 -m pip install --upgrade pip pre-commit ruff black
pre-commit install
