#!/usr/bin/env bash
set -e

cargo build
uv sync
