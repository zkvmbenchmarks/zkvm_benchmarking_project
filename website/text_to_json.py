# process_results.py
import json
import glob
import os
import re
import pprint

def process_results():
    data = {
        "proving_time": {},
        "total_cycles": {},
        "peak_memory_consumption": {},
        "proof_size": {},
        "verification_time": {},
        "memory_leak": {},
        "power_consumption": {},
    }

    main_categories = {"fibonacci", "groth16", "isprime", "sort", "RSA", "SHA-2", "vecSum"};
    
    for metric in data:
        for category in main_categories:
            data[metric][category] = {}
            for result_file in glob.glob("../results/*_benchmark_results"):
                parsed_data = parse_proof_data(result_file)
                if category.lower() in result_file.lower():
                    parts = result_file.split("_")
                    zkvm = parts[0].split("/")[2]
                    specific_test = parts[1]
                    # add the entry zkvm: parsed_data[metric] into the data[metric][category][specific_test]
                    if specific_test in data[metric][category]:
                        data[metric][category][specific_test].update({zkvm: parsed_data[metric]})
                    else:
                        data[metric][category][specific_test] = {zkvm: parsed_data[metric]}

    print(data)
    
    # Write to JSON
    with open("data.json", "w") as f:
        json.dump(data, f, indent=2)
    


def parse_proof_data(result_file):

    #read result file from ../results and save it in a string called text
    with open(result_file, "r") as f:
        text = f.read()

    parsed_data = {}

    # Patterns to extract data
    patterns = {
        "proving_time": r"Proving time: ([\d.]+s)",
        "total_cycles": r"Total cycles: (\d+)",
        "peak_memory_consumption": r"Peak memory consumption during proving: (\d+ KB)",
        "proof_size": r"Proof size: ([\d.]+ KB)",
        "verification_time": r"Verification time: ([\d.]+ms)",
        "memory_leak": r"Total memory leak: (\d+ KB)",
        "power_consumption": r"Total power consumption: (\d+ units)"
    }

    # Extract data using regex
    for key, pattern in patterns.items():
        match = re.search(pattern, text)
        if match:
            parsed_data[key] = match.group(1)

    return parsed_data



process_results()