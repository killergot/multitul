# rust_multitul

Десктоп-приложение на [iced 0.14](https://iced.rs) — сборник небольших экспериментов и мини-игр в одном окне. Под капотом: собственный парсер git-репозитория с визуализацией графа коммитов, индикатор состояния сети, игра на двоих с передачей по WebSocket.

Платформа разработки — Windows; код кроссплатформенный, но в Cargo-кэше при работающем `rust_multitul.exe` Windows блокирует пересборку — закройте предыдущий запуск перед `cargo build`.

## Что внутри

Главное меню — точка входа во все экраны. Каждый экран самодостаточен: своё состояние, свои сообщения, свой `update`/`view`/`subscription`.

- **01 · Counter** — минимальный пример состояния (инкремент/декремент).
- **02 · Wordly** — клон Wordle на русских словах. Слова берутся из `assets/words_5_ru.txt` (5-буквенные) и `assets/all_nouns_ru.txt` (для валидации). Управление — мышью или физической клавиатурой (стрелки, Backspace, Enter).
- **03 · One Brain** — кооперативная игра «одна голова на двоих» по WebSocket. Два игрока подключаются в одну комнату, в каждом раунде одновременно отправляют слово-ассоциацию; раунд считается выигранным, если слова совпали. Есть чат, история раундов, индикатор готовности.

Под основным экраном — нижняя панель с двумя постоянными виджетами:

- **Git graph** — собственный парсер `.git/` (без зависимости от libgit2), строит топологический layout и рисует граф коммитов через `iced::canvas`. Видим всегда, независимо от выбранного экрана.
- **Sign** — подпись авторов проекта.

В правом верхнем углу окна — overlay-индикатор сети: периодический ping и SVG-иконка уровня сигнала (`assets/icons/wifi_*.svg`).

Между основной областью и нижней панелью — перетаскиваемый сплиттер (высота нижней панели от 80 до 800 px).

## Сборка и запуск

```bash
cargo run            # debug
cargo run --release  # release
cargo check          # быстрая проверка без линковки
```

Требования: Rust edition 2024 (свежая стабильная toolchain), Git-репозиторий в рабочем каталоге (для нижней панели). Если репозиторий не найден или повреждён, нижняя панель покажет «Ошибка при парсинге гит графа», но приложение продолжит работать.

## Сервер для One Brain

Игра One Brain по умолчанию подключается к `ws://185.200.176.8:8765` — поле адреса редактируется в UI. Локальный сервер находится в `scripts/server/` (Python + `websockets`, плюс HTML-дашборд на 8080).

```bash
cd scripts/server
docker compose up --build
# сервер: ws://localhost:8765
# дашборд: http://localhost:8080
```

Без Docker:

```bash
cd scripts/server
pip install -r requirements.txt
python server.py
```

Тесты сервера — `scripts/server/tests/`.

## Структура репозитория

```
src/
├── main.rs             корневой App: Screen, Message, layout, splitter, overlay
├── core/               общие виджеты, видимые на всех экранах
│   ├── git/widget.rs       нижняя git-панель (canvas)
│   ├── sign.rs             подпись авторов
│   └── network/            ping + индикатор сети
├── games/              экраны-игры (каждая = mod + menu + styles + …)
│   ├── wordly/             Wordle на русском
│   └── one_brain/          кооп по WebSocket (protocol.rs, ws.rs)
├── utils/
│   ├── style.rs            общие helpers: mix(), soft_shadow(), палитра
│   └── git/                парсер .git/, построение графа и layout
└── macros/             декларативные макросы (string newtype и т. п.)

assets/
├── words_5_ru.txt          словарь Wordle
├── all_nouns_ru.txt        словарь для валидации
└── icons/                  SVG-иконки сети

scripts/
├── server/                 Python-сервер One Brain + Dockerfile + dashboard
└── word_gen.py             вспомогательный скрипт для словарей
```

Подробные заметки по архитектуре и подводным камням iced 0.14 (padding, mouse_area, subscription, стили) — в [`CLAUDE.md`](./CLAUDE.md).

## Стек

- [`iced`](https://crates.io/crates/iced) 0.14 (`canvas`, `svg`, `tokio`)
- [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite) — WebSocket-клиент One Brain
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) — протокол One Brain
- [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) — корректная работа с кириллицей в Wordle
- [`flate2`](https://crates.io/crates/flate2) — распаковка git-объектов
- [`ping-rs`](https://crates.io/crates/ping-rs) — индикатор сети
- сервер: Python 3 + [`websockets`](https://pypi.org/project/websockets/)
