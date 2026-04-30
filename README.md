# rust_multitul

Десктоп-приложение на [iced 0.14](https://iced.rs) — сборник небольших экспериментов и мини-игр в одном окне. Под капотом: собственный парсер git-репозитория с визуализацией графа коммитов, индикатор состояния сети, кооперативная игра по WebSocket и теперь одиночный сапер с настраиваемой сложностью.

Платформа разработки — Windows. Код кроссплатформенный, но на Windows уже запущенный `rust_multitul.exe` может держать lock на `target/`, из-за чего `cargo build` или `cargo test` иногда падают по артефактам сборки. Перед пересборкой лучше закрыть предыдущий запуск приложения.

## Что внутри

Главное меню — точка входа во все экраны. Каждый экран самодостаточен: у него своё состояние, свои сообщения и собственные `update` / `view` / `subscription`.

- **01 - Counter** — минимальный пример состояния: инкремент и декремент счётчика.
- **02 - Wordly** — клон Wordle на русских словах. Слова берутся из `assets/words_5_ru.txt`, а валидация идёт по `assets/all_nouns_ru.txt`. Поддерживаются мышь и клавиатура.
- **03 - One Brain** — кооперативная игра по WebSocket. Два игрока входят в одну комнату, синхронно отправляют слова-ассоциации, общаются в чате и смотрят историю раундов.
- **04 - Minesweeper** — одиночный сапер с меню сложности. Есть пресеты `9x9x10`, `16x16x40`, `30x16x99` и режим `Custom` с выбором `width / height / mines`.

### Возможности сапера

- Safe first click: первая открытая клетка никогда не содержит мину.
- Флаги по правому клику.
- Автооткрытие пустых областей.
- Повторное открытие соседей у числа, если рядом уже выставлено нужное число флагов.
- Таймер партии.
- Быстрый `Restart` на той же конфигурации.
- Возврат к выбору сложности без перезапуска приложения.

Под основным экраном всегда видна нижняя панель:

- **Git graph** — собственный парсер `.git/` без `libgit2`, строящий layout и рисующий граф через `iced::canvas`.
- **Sign** — подпись авторов проекта.

В правом верхнем углу окна расположен overlay-индикатор сети на базе периодического ping и SVG-иконок из `assets/icons/`.

Между основной областью и нижней панелью — перетаскиваемый splitter. Высота нижней панели ограничена диапазоном от 80 до 800 px.

## Сборка и запуск

```bash
cargo run            # debug
cargo run --release  # release
cargo check          # быстрая проверка без линковки
cargo test           # unit-тесты
```

Требования:

- Rust edition 2024.
- Рабочий git-репозиторий в текущем каталоге для нижней git-панели.

Если репозиторий не найден или повреждён, нижняя панель покажет ошибку парсинга git-графа, но само приложение продолжит работать.

## Тесты

Сейчас unit-тестами покрыто игровое ядро сапера из `src/games/minesweeper/model.rs`.

Проверяются такие сценарии:

- первый клик всегда безопасен;
- flood fill корректно открывает пустую область;
- постановка и снятие флагов обновляют счётчик;
- chord/open-neighbors срабатывает при достаточном числе флагов;
- открытие мины переводит игру в проигрыш;
- невалидная конфигурация поля отклоняется.

Ожидаемый результат для нового набора тестов сапера: `6 passed, 0 failed`.

## Сервер для One Brain

Игра One Brain по умолчанию подключается к `ws://185.200.176.8:8765`. Локальный сервер лежит в `scripts/server/`.

```bash
cd scripts/server
docker compose up --build
# сервер: ws://localhost:8765
# dashboard: http://localhost:8080
```

Без Docker:

```bash
cd scripts/server
pip install -r requirements.txt
python server.py
```

Тесты сервера находятся в `scripts/server/tests/`.

## Структура репозитория

```text
src/
├── main.rs                 корневой App: Screen, Message, layout, splitter, overlay
├── core/                   общие виджеты, видимые на всех экранах
│   ├── git/widget.rs       нижняя git-панель на canvas
│   ├── sign.rs             подпись авторов
│   └── network/            ping + индикатор сети
├── games/                  экраны-игры
│   ├── minesweeper/        сапер: UI, сложности, игровое ядро, unit-тесты
│   ├── wordly/             Wordle на русском
│   └── one_brain/          кооператив по WebSocket
├── utils/
│   ├── style.rs            общая палитра и style helpers
│   └── git/                парсер .git/, граф и layout
└── macros/                 декларативные макросы

assets/
├── words_5_ru.txt          словарь Wordly
├── all_nouns_ru.txt        словарь для валидации
└── icons/                  SVG-иконки сети

scripts/
├── server/                 Python-сервер One Brain + Dockerfile + dashboard
└── word_gen.py             вспомогательный скрипт для словарей
```

Подробные заметки по архитектуре и особенностям `iced 0.14` лежат в [`CLAUDE.md`](./CLAUDE.md).

## Стек

- [`iced`](https://crates.io/crates/iced) 0.14 (`canvas`, `svg`, `tokio`)
- [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite) — WebSocket-клиент One Brain
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) — протокол One Brain
- [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) — корректная работа с кириллицей в Wordly
- [`flate2`](https://crates.io/crates/flate2) — распаковка git-объектов
- [`ping-rs`](https://crates.io/crates/ping-rs) — индикатор сети
- сервер One Brain: Python 3 + [`websockets`](https://pypi.org/project/websockets/)

## Codex

Мини-игра **Minesweeper**, её игровое ядро, экран выбора сложности, интеграция в главное меню и набор unit-тестов для логики поля были добавлены с помощью Codex.
