from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer
from tokenizers.pre_tokenizers import Whitespace

# Initialize a tokenizer
tokenizer = Tokenizer(BPE(unk_token="[UNK]"))
tokenizer.pre_tokenizer = Whitespace()

# Initialize a trainer
trainer = BpeTrainer(vocab_size=8000, min_frequency=2, special_tokens=["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])

# Train the tokenizer
files = ["albert-moe-13/data/corpus/bible.txt"]
tokenizer.train(files, trainer)

# Save the tokenizer
import os
os.makedirs("data", exist_ok=True)
tokenizer.save("data/vocab.json")
print("Tokenizer trained on Bible corpus and saved to data/vocab.json")
