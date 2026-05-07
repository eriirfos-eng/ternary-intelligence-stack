import os
import requests
import json

# Taxonomy and target domains
DOMAINS = ["technical_debugging", "scientific_method", "theological_inquiry"]
BASE_PATH = "data/corpus/dataset_v2"
LANGUAGE = "en"

# Mapping domains to representative Wikipedia search terms to seed our base knowledge
DOMAIN_SEED_TERMS = {
    "technical_debugging": ["Debugging", "Computer_programming", "Software_testing", "Race_condition", "Memory_leak"],
    "scientific_method": ["Scientific_method", "Empirical_evidence", "Hypothesis", "Peer_review", "Scientific_theory"],
    "theological_inquiry": ["Theology", "Hermeneutics", "Comparative_religion", "Exegesis", "Philosophy_of_religion"]
}

def fetch_wiki_content(term):
    """Fetches clean text from Wikipedia using the REST API."""
    url = f"https://{LANGUAGE}.wikipedia.org/api/rest_v1/page/summary/{term}"
    try:
        response = requests.get(url)
        if response.status_code == 200:
            data = response.json()
            return f"# {data['title']}\n\n{data.get('extract', '')}"
    except Exception as e:
        print(f"Error fetching {term}: {e}")
    return None

def main():
    print("Initiating foundational dataset ingestion...")
    for domain in DOMAINS:
        domain_path = os.path.join(BASE_PATH, domain, LANGUAGE)
        os.makedirs(domain_path, exist_ok=True)
        
        for term in DOMAIN_SEED_TERMS[domain]:
            print(f"Fetching {term} for {domain}...")
            content = fetch_wiki_content(term)
            if content:
                file_path = os.path.join(domain_path, f"{term}.txt")
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write(content)
    print("Foundational ingestion complete.")

if __name__ == "__main__":
    main()
