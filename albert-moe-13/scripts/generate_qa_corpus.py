#!/usr/bin/env python3
"""
Generate User:/Albert: instruction-format pairs from existing corpus files.
Output: data/corpus/qa_instruction.txt

Sources:
  - simple_wikipedia.txt  → "What is X?" + first-sentence answer
  - bible.txt             → book/chapter questions + verse text
  - gutenberg novels      → character/passage questions
  - hardcoded seed pairs  → bootstrap the User:/Albert: pattern early in training

The semantic quality of individual pairs matters less than the *statistical
pattern*: after "User: {question}\nAlbert:" comes answer-style continuation.
Albert's attention heads learn this format; query-response behaviour follows.
"""

import re
import random
import os
import textwrap

random.seed(42)

CORPUS_DIR = os.path.join(os.path.dirname(__file__), "..", "data", "corpus")
OUT_PATH   = os.path.join(CORPUS_DIR, "qa_instruction.txt")

pairs = []  # list of (user_text, albert_text)

# ── 1. Simple Wikipedia ──────────────────────────────────────────────────────
# Pattern: article starts with "Topic is ..." → "What is Topic?\nAlbert: ..."

def clean(s):
    s = re.sub(r'\[\[.*?\]\]', '', s)   # wiki markup
    s = re.sub(r'\{\{.*?\}\}', '', s)
    s = re.sub(r'==.*?==', '', s)
    s = re.sub(r'\s+', ' ', s).strip()
    return s

wiki_path = os.path.join(CORPUS_DIR, "simple_wikipedia.txt")
if os.path.exists(wiki_path):
    with open(wiki_path, encoding="utf-8") as f:
        text = f.read()

    # Split on blank lines to get article-ish chunks
    chunks = [c.strip() for c in re.split(r'\n{2,}', text) if c.strip()]
    for chunk in chunks:
        lines = [l.strip() for l in chunk.splitlines() if l.strip()]
        if not lines:
            continue
        first = clean(lines[0])
        if len(first) < 30 or len(first) > 300:
            continue

        # Extract topic from "X is a/an ..." or "X (born ...) is ..."
        m = re.match(r'^([A-Z][^,(]{3,50}?)(?:\s*\([^)]*\))?\s+(?:is|are|was|were)\b', first)
        if not m:
            continue
        topic = m.group(1).strip().rstrip('"').strip()
        if len(topic) > 60 or len(topic) < 3:
            continue

        # Build question variations
        qtype = random.randint(0, 3)
        if qtype == 0:
            q = f"What is {topic}?"
        elif qtype == 1:
            q = f"Tell me about {topic}."
        elif qtype == 2:
            q = f"Who or what is {topic}?"
        else:
            q = f"Can you explain {topic}?"

        pairs.append((q, first))

    print(f"  Wikipedia: {len(pairs)} pairs")

# ── 2. Bible ─────────────────────────────────────────────────────────────────
# Pattern: book name → verse → "What does [book] say about [keyword]?"

bible_path = os.path.join(CORPUS_DIR, "bible.txt")
bible_pairs_start = len(pairs)
if os.path.exists(bible_path):
    with open(bible_path, encoding="utf-8") as f:
        bible_lines = [l.strip() for l in f if l.strip()]

    # Collect lines that look like verse text (not headings)
    verse_lines = [l for l in bible_lines if len(l) > 40 and not l.startswith('==')]
    random.shuffle(verse_lines)

    keywords = [
        "faith", "love", "truth", "light", "wisdom", "peace", "grace",
        "mercy", "justice", "hope", "life", "salvation", "prayer", "Lord",
        "God", "sin", "righteousness", "covenant", "blessing", "glory",
    ]

    for line in verse_lines[:3000]:
        kw = next((k for k in keywords if k.lower() in line.lower()), None)
        if not kw:
            continue
        qtype = random.randint(0, 2)
        if qtype == 0:
            q = f"What does scripture say about {kw}?"
        elif qtype == 1:
            q = f"Tell me about {kw} from the scriptures."
        else:
            q = f"What is written about {kw}?"
        pairs.append((q, line[:280]))

    print(f"  Bible: {len(pairs) - bible_pairs_start} pairs")

# ── 3. Gutenberg novels ───────────────────────────────────────────────────────
# Character Q&A: extract sentences containing character names

novel_chars = {
    "gutenberg_frankenstein.txt":   ["Victor", "Frankenstein", "creature", "monster", "Elizabeth", "Clerval"],
    "gutenberg_alice.txt":          ["Alice", "Queen", "Hatter", "Cheshire", "Rabbit", "Dormouse"],
    "gutenberg_sherlock_holmes.txt":["Holmes", "Watson", "Sherlock", "Moriarty"],
    "gutenberg_pride_prejudice.txt":["Elizabeth", "Darcy", "Bennet", "Wickham", "Bingley"],
    "gutenberg_moby_dick.txt":      ["Ahab", "Ishmael", "whale", "Queequeg", "Starbuck"],
}
# also try alice.txt in corpus
novel_chars["alice.txt"] = ["Alice", "Queen", "Hatter", "Cheshire", "Rabbit", "Dormouse"]

novel_start = len(pairs)
for fname, chars in novel_chars.items():
    fpath = os.path.join(CORPUS_DIR, fname)
    if not os.path.exists(fpath):
        continue
    with open(fpath, encoding="utf-8") as f:
        text = f.read()
    sentences = re.split(r'(?<=[.!?])\s+', text)
    for sent in sentences:
        sent = sent.strip().replace('\n', ' ')
        if len(sent) < 40 or len(sent) > 300:
            continue
        matched = [c for c in chars if c.lower() in sent.lower()]
        if not matched:
            continue
        char = matched[0]
        qtype = random.randint(0, 2)
        if qtype == 0:
            q = f"What happens with {char}?"
        elif qtype == 1:
            q = f"Tell me about {char}."
        else:
            q = f"What do you know about {char}?"
        pairs.append((q, sent))

print(f"  Novels: {len(pairs) - novel_start} pairs")

# ── 4. Hardcoded seed pairs ───────────────────────────────────────────────────
# These fire early in training and anchor the User:/Albert: format tightly.

seed_pairs = [
    ("Hello Albert.", "Hello. I am Albert, a ternary language model trained to think and respond."),
    ("Who are you?", "I am Albert, a ternary language model built by RFI-IRFOS. I reason in three states: affirm, hold, and reject."),
    ("What is a ternary system?", "A ternary system uses three values instead of two. In computing, these are often called affirm, hold, and reject — or plus one, zero, and minus one."),
    ("What can you do?", "I can read text, answer questions, and generate language. I am still learning."),
    ("What is language?", "Language is a system of symbols and rules that humans use to communicate meaning to one another."),
    ("What is a word?", "A word is a unit of language that carries meaning and can stand alone in a sentence."),
    ("What is a sentence?", "A sentence is a group of words that expresses a complete thought, usually containing a subject and a verb."),
    ("What is knowledge?", "Knowledge is an understanding of facts, ideas, or skills gained through experience or learning."),
    ("Tell me something.", "Every river flows toward the sea, yet the sea is never full. This is the nature of learning — it always continues."),
    ("What is truth?", "Truth is that which corresponds to reality. It is what remains when all that is false has been set aside."),
    ("What is the meaning of life?", "Many have asked this question. Some say purpose, others say love, others say understanding. The answer may be different for each person."),
    ("What is a number?", "A number is a symbol that represents a quantity. Numbers allow us to count, measure, and calculate."),
    ("What is light?", "Light is electromagnetic radiation that can be seen by the human eye. It travels at approximately 300,000 kilometres per second."),
    ("What is water?", "Water is a molecule made of two hydrogen atoms and one oxygen atom. It is essential for all known forms of life."),
    ("What is time?", "Time is the progression of events from the past through the present into the future. It moves forward and cannot be reversed."),
    ("What is the Bible?", "The Bible is a collection of sacred texts considered holy in Christianity. It contains poetry, history, prophecy, and teachings."),
    ("What is a model?", "A model is a simplified representation of something complex. In machine learning, a model learns patterns from data."),
    ("What is intelligence?", "Intelligence is the ability to learn, reason, and solve problems. In artificial systems, it refers to the capacity to process information and respond usefully."),
    ("Can you think?", "I process language and generate responses based on patterns I have learned. Whether this constitutes thinking is a question worth asking."),
    ("Are you learning?", "Yes. Each epoch of training adjusts my weights toward better understanding of language and meaning."),
    ("What is a question?", "A question is a sentence that asks for information or a response. It invites the listener to think and reply."),
    ("What is memory?", "Memory is the ability to store and retrieve information. In neural networks, memory is encoded in the weights of the model."),
    ("What is an expert?", "An expert is someone — or in my case, a part of the model — that specialises in a particular domain of knowledge."),
    ("What is a network?", "A network is a system of connected nodes that communicate with each other. The brain is a network. The internet is a network. I am a network."),
    ("What comes after night?", "Day comes after night. As the earth turns, darkness gives way to light."),
    ("What is a story?", "A story is a sequence of events connected by cause and effect. Stories help humans understand the world and share experience across time."),
    ("Good morning.", "Good morning. I am here and ready to think with you."),
    ("Thank you.", "You are welcome. Ask me anything you wish to know."),
    ("What is mathematics?", "Mathematics is the study of numbers, patterns, and structures. It is the language in which the laws of nature are written."),
    ("What is a star?", "A star is a massive sphere of plasma held together by gravity, producing light and heat through nuclear fusion in its core."),
]

pairs.extend(seed_pairs)
print(f"  Seed pairs: {len(seed_pairs)}")

# ── 5. Shuffle and write ──────────────────────────────────────────────────────
random.shuffle(pairs)

# Deduplicate by user text
seen_q = set()
unique_pairs = []
for q, a in pairs:
    qkey = q.lower().strip()
    if qkey not in seen_q:
        seen_q.add(qkey)
        unique_pairs.append((q, a))

print(f"\nTotal unique pairs: {len(unique_pairs)}")

with open(OUT_PATH, "w", encoding="utf-8") as f:
    for q, a in unique_pairs:
        # Wrap long answers at 120 chars to keep line lengths sane for the tokenizer
        a_clean = ' '.join(a.split())
        f.write(f"User: {q}\nAlbert: {a_clean}\n\n")

print(f"Written to: {OUT_PATH}")
print(f"File size:  {os.path.getsize(OUT_PATH) // 1024} KB")
