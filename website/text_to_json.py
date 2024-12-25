# process_results.py
import json
import glob
import os
import re
import pprint



def format_value(value_str):
    # Split value and unit
    num = ""
    unit = ""
    
    # Special case for 'units' to prevent treating it as time unit
    if ' units' in value_str:
        parts = value_str.split()
        num = parts[0]
        unit = 'units'
    # Handle time units
    elif value_str.endswith(('s', 'ms')):
        if value_str.endswith('ms'):
            unit = 'ms'
            num = value_str[:-2]
        else:
            unit = 's'
            num = value_str[:-1]
    # Handle other units (KB, etc)
    else:
        parts = value_str.split()
        num = parts[0]
        unit = parts[1] if len(parts) > 1 else ''

    # Format number
    try:
        if '.' in num:
            formatted_num = f"{float(num):.2f}"
        else:
            formatted_num = num
    except ValueError:
        return value_str

    return f"{formatted_num} {unit}".strip()



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
            temp_dict = {}
            for result_file in glob.glob("../results/*_benchmark_results"):
                parsed_data = parse_proof_data(result_file)
                if category.lower() in result_file.lower():
                    parts = result_file.split("_")
                    zkvm = parts[0].split("/")[2]
                    specific_test = parts[1]
                    if specific_test in temp_dict:
                        temp_dict[specific_test].update({zkvm: parsed_data[metric]})
                    else:
                        temp_dict[specific_test] = {zkvm: parsed_data[metric]}
            
            # Sort the inner dictionary based on numeric values in keys
            sorted_items = sorted(temp_dict.items(), 
                                key=lambda x: int(''.join(filter(str.isdigit, x[0]))) if any(c.isdigit() for c in x[0]) else 0)
            data[metric][category] = dict(sorted_items)

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
            parsed_data[key] = format_value(match.group(1))

    return parsed_data



process_results()