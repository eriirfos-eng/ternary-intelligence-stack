import os
import requests
import json

# GitHub API configuration
GITHUB_TOKEN = os.getenv('GITHUB_TOKEN')
HEADERS = {'Authorization': f'token {GITHUB_TOKEN}'} if GITHUB_TOKEN else {}

# Representative repositories for our 13-language strategy
REPOS = {
    "rust": "rust-lang/rust",
    "python": "python/cpython",
    "go": "golang/go",
    "java": "openjdk/jdk",
    "cpp": "llvm/llvm-project",
    "typescript": "microsoft/TypeScript",
    "javascript": "nodejs/node",
    "ruby": "ruby/ruby",
    "php": "php/php-src",
    "csharp": "dotnet/runtime",
    "swift": "apple/swift",
    "kotlin": "JetBrains/kotlin",
    "dart": "dart-lang/sdk"
}

def fetch_issues(repo_name, language):
    url = f"https://api.github.com/repos/{repo_name}/issues?state=closed&per_page=5"
    response = requests.get(url, headers=HEADERS)
    if response.status_code == 200:
        issues = response.json()
        for issue in issues:
            if 'pull_request' not in issue: # Focus on genuine issues
                path = f"data/corpus/engineering_reasoning/{language}/{issue['number']}.json"
                os.makedirs(os.path.dirname(path), exist_ok=True)
                with open(path, 'w') as f:
                    json.dump(issue, f, indent=2)
                print(f"Ingested issue #{issue['number']} from {repo_name}")

def main():
    for lang, repo in REPOS.items():
        print(f"Fetching issues for {lang} ({repo})...")
        fetch_issues(repo, lang)

if __name__ == "__main__":
    main()
