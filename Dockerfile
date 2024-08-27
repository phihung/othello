FROM ghcr.io/pyo3/maturin AS builder

ADD . .
RUN /usr/bin/maturin build --release --interpreter python3.11

FROM docker.io/library/python:3.11-slim-bullseye

RUN apt-get update

RUN useradd -m -u 1000 user
USER user
WORKDIR /home/user/app

RUN pip install --user --upgrade pip
RUN pip install --user --no-cache python-fasthtml

ENV PATH="/home/user/.local/bin:$PATH"

COPY --chown=user --from=builder /io/target/wheels/othello-0.1.0-cp311-cp311-manylinux*.whl .
RUN pip install --user *.whl

ENTRYPOINT ["othello-ui"]