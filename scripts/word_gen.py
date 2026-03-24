# /// script
# requires-python = ">=3.10"
# dependencies = ["pymorphy3", "wordfreq"]
# ///


from __future__ import annotations

import re
from collections import OrderedDict

from pymorphy3 import MorphAnalyzer
from wordfreq import top_n_list, zipf_frequency


OUTPUT_FILE = "wordly_ru.txt"
TARGET_COUNT = 2000
SOURCE_POOL = 300000  # чем больше, тем лучше выборка

morph = MorphAnalyzer()

CYRILLIC_RE = re.compile(r"^[а-яё]+$", re.IGNORECASE)

# Части речи, которые обычно хорошо подходят для wordly-подобной игры
ALLOWED_POS = {
    "NOUN",   # существительное
#     "ADJF",   # полное прилагательное
#     "INFN",   # инфинитив
#     "VERB",   # личная форма глагола
}

# Слова, которые обычно лучше убрать
BANNED_WORDS = {
    "очень", "снова", "почти", "своё", "твоё", "моё",
    "всего", "себя", "меня", "тебя", "сюда", "туда",
}

# Нежелательные граммемы
BANNED_GRAMMEMES = {
    "Name",   # имя
    "Surn",   # фамилия
    "Patr",   # отчество
    "Geox",   # топоним
    "Orgn",   # организация
    "Trad",   # торговая марка
    "Abbr",   # аббревиатура
    "Slng",   # сленг
    "Arch",   # архаизм
    "Litr",   # литературный спец.
    "Erro",   # искажённое
    "Dist",   # искажённая форма
    "Ques",   # вопросит.
    "Dmns",   # простореч.
}

# Предпочтительные формы:
# - именительный падеж
# - единственное число
PREFERRED_GRAMMEMES = {"nomn", "sing"}


def is_good_surface_form(word: str) -> bool:
    word = word.lower().strip()

    if len(word) != 5:
        return False

    if not CYRILLIC_RE.fullmatch(word):
        return False

    if "ъ" in word:
        return False

    if word in BANNED_WORDS:
        return False

    # Слишком много одинаковых букв подряд / странные формы
    if any(ch * 3 in word for ch in set(word)):
        return False

    return True


def score_parse(word: str, parse) -> tuple[int, float]:
    """
    Чем больше score, тем охотнее берём parse как 'основной'.
    """
    tag = parse.tag
    score = 0

    if tag.POS in ALLOWED_POS:
        score += 50

    if "nomn" in tag:
        score += 20

    if "sing" in tag:
        score += 10

    if parse.normal_form == word:
        score += 25

    # pymorphy вероятность разбора
    score += int(parse.score * 100)

    # частотность самого слова как небольшой бонус
    freq = zipf_frequency(word, "ru")
    return score, freq


def choose_best_parse(word: str):
    parses = morph.parse(word)
    parses = sorted(parses, key=lambda p: score_parse(word, p), reverse=True)
    return parses[0] if parses else None


def is_good_word(word: str) -> bool:
    if not is_good_surface_form(word):
        return False

    parse = choose_best_parse(word)
    if parse is None:
        return False

    tag = parse.tag

    if tag.POS not in ALLOWED_POS:
        return False

    if any(g in tag for g in BANNED_GRAMMEMES):
        return False

    # Убираем слишком экзотические формы:
    # причастия, деепричастия, краткие формы, сравнительные и т.п.
    if tag.POS in {"PRTF", "PRTS", "GRND", "COMP", "ADJS"}:
        return False

    # Для существительных и прилагательных стараемся оставить словарную форму
    if tag.POS in {"NOUN", "ADJF"}:
        if "nomn" not in tag:
            return False
        if "sing" not in tag:
            return False
        if parse.normal_form != word:
            return False

    # Для глаголов чаще оставляем либо инфинитив,
    # либо частотные личные формы без экзотики
    if tag.POS == "INFN":
        if parse.normal_form != word:
            return False

    if tag.POS == "VERB":
        if any(g in tag for g in {"impr", "pssv"}):
            return False

    # Минимальная частотность — убираем совсем редкие слова
    if zipf_frequency(word, "ru") < 2.8:
        return False

    return True


def build_word_list(limit: int = TARGET_COUNT) -> list[str]:
    # Берём частотный пул слов
    candidates = top_n_list("ru", SOURCE_POOL)

    # OrderedDict — чтобы сохранить порядок и убрать дубли
    result = OrderedDict()

    for raw in candidates:
        word = raw.lower().strip().replace("ѐ", "ё")

        if word in result:
            continue

        if is_good_word(word):
            result[word] = None

        if len(result) >= limit:
            break

    return list(result.keys())


def main():
    words = build_word_list(TARGET_COUNT)

    if len(words) < TARGET_COUNT:
        print(
            f"Предупреждение: удалось собрать только {len(words)} слов. "
            f"Попробуй увеличить SOURCE_POOL."
        )

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        for word in words:
            f.write(word + "\n")

    print(f"Готово: {len(words)} слов записано в {OUTPUT_FILE}")


if __name__ == "__main__":
    main()