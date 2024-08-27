FROM docker.io/library/python:3.11-slim-bullseye

RUN apt-get update

RUN useradd -m -u 1000 user
USER user
WORKDIR /home/user/app

RUN pip install --user --upgrade pip
RUN pip install --user --no-cache python-fasthtml

ENV PATH="/home/user/.local/bin:$PATH"

COPY --chown=user target/wheels/othello-0.1.0-cp311-cp311-manylinux_2_17_aarch64.manylinux2014_aarch64.whl .
RUN pip install --user othello-0.1.0-cp311-cp311-manylinux_2_17_aarch64.manylinux2014_aarch64.whl
# COPY --chown=user python/othello/__init__.py ./othello/__init__.py
# COPY --chown=user python/othello/ui.py ./othello/ui.py

ENTRYPOINT ["othello-ui"]
# ENTRYPOINT ["uvicorn"]
# CMD ["pkg.ui:create_app", "--host", "0.0.0.0", "--port", "8000", "--factory"]%