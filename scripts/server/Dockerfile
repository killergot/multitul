FROM python:3.12-slim

WORKDIR /app

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY . .

RUN useradd --system --uid 1000 --no-create-home --shell /usr/sbin/nologin app \
    && chown -R app:app /app

USER app

EXPOSE 8765 8080

CMD ["python", "server.py"]
