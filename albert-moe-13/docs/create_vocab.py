import json
words = ["[UNK]", "[CLS]", "[SEP]", "the", "and", "of", "to", "in", "he", "shall", "unto", "i", "his", "lord", "god", "it", "with", "that", "is", "for", "saying", "from", "them", "me", "my", "was", "be", "thou", "thy", "all", "which", "not", "said", "but", "him", "upon", "they", "this", "out", "were", "when", "man", "as", "into", "up", "will", "so", "by", "israel", "there", "then", "their", "no", "if", "more", "she", "her", "hand", "king", "after", "come", "do", "hath", "son", "before", "people", "gave", "went", "day", "good", "great", "earth", "land", "house", "children", "even", "came", "see", "hear", "made", "now", "know", "name", "woman", "heart", "face", "voice", "word", "light", "darkness", "spirit", "life", "death", "soul", "heaven", "hell", "fire", "water", "blood", "flesh"]
vocab = {word: i for i, word in enumerate(words)}
data = {
    "version": "1.0",
    "truncation": None,
    "padding": None,
    "added_tokens": [],
    "normalizer": None,
    "pre_tokenizer": {"type": "Whitespace"},
    "post_processor": None,
    "decoder": None,
    "model": {
        "type": "BPE",
        "dropout": None,
        "unk_token": "[UNK]",
        "continuing_subword_prefix": None,
        "end_of_word_suffix": None,
        "fuse_unk": False,
        "vocab": vocab,
        "merges": []
    }
}
with open("data/vocab.json", "w") as f:
    json.dump(data, f)
