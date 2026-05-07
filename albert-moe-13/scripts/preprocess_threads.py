import json
import os

def format_thread_for_albert(thread_data):
    """
    Transforms raw Reddit JSON into an 'Instruction-Result' style format 
    suitable for agentic training.
    """
    formatted = {
        "domain": "technical_debugging",
        "title": thread_data['title'],
        "context": thread_data['body'],
        "resolution_path": [],
        "summary": ""
    }
    
    # Simple logic: extract top-voted comments as the resolution path
    sorted_comments = sorted(thread_data['comments'], key=lambda x: x['score'], reverse=True)
    
    for comment in sorted_comments[:5]:
        formatted["resolution_path"].append({
            "author": comment['author'],
            "content": comment['body']
        })
        
    return formatted

def process_scraped_data(input_file, output_dir):
    """Reads raw scraped threads and prepares them for ALBERT's ingestion."""
    with open(input_file, 'r') as f:
        threads = json.load(f)
        
    for thread in threads:
        processed = format_thread_for_albert(thread)
        output_path = os.path.join(output_dir, f"{thread['title'][:10]}.json")
        with open(output_path, 'w') as f_out:
            json.dump(processed, f_out, indent=2)

if __name__ == "__main__":
    print("Ingestion pre-processor initialized.")
