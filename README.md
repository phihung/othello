---
title: Othello
emoji: 🐢
colorFrom: purple
colorTo: blue
sdk: docker
pinned: false
license: apache-2.0
app_port: 5001
---

# Othello Game

An educational Othello game implemented in Rust and Python.

**Play online:**

- [Hugging Face](https://huggingface.co/spaces/phihung/othello)
- [Alternate Link](https://phihung-othello.hf.space/)

The game's core and AI are written in Rust. A Python API is provided using [pyo3](https://pyo3.rs/), and the UI is built with [fasthtml](https://docs.fastht.ml/) and Tailwind CSS.

## Getting Started

**Run Locally:**

```bash
uv sync
maturin develop
othello-ui
```

**Run with Docker:**

```bash
docker build -t othello .
docker run --rm -p 5001:5001 -it othello
```

## Credits

Board implementation is based on the [rust-othello](https://gitlab.com/rust-othello/8x8-othello) project.
