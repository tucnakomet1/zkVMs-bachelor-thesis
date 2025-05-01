import re
import csv
import argparse


# Scrape data from log file and return list of dictionaries
def scrape_sp1(log_file: str, snark: bool) -> list:
    results = []
    current_data: dict = {}

    all_data = {
        "Run": "mean",
        "Total cycles": 0,
        "Proof size (bytes)": 0,
        "Time busy prover (ms)": 0,
        "Time busy verifier (ms)": 0,
    }

    
    with open(log_file, 'r', encoding='utf-8') as file:
        for line in file:
            #if "Input" in line:
            if "Run" in line and "Input" in line:
                if current_data:
                    results.append(current_data)

                #line2 = line.split(":")[0]
                current_data = {
                    "Run": int(line.strip().split(" ")[1].replace(";", "")),
                    "Total cycles": 0,
                    "Proof size (bytes)": 0,
                    "Time busy prover (ms)": 0,
                    "Time busy verifier (ms)": 0
                }
            
            elif "Total cycles" in line:
                match = re.search(r'Total cycles: (\d+)', line)
                if match:
                    cyc = int(match.group(1))

                    current_data["Total cycles"] = cyc
                    all_data["Total cycles"] += cyc
            elif "Proof size" in line:
                match = re.search(r'Proof size: (\d+)', line)
                if match:
                    pf_s = int(match.group(1))
                    
                    current_data["Proof size (bytes)"] = pf_s
                    all_data["Proof size (bytes)"] += pf_s
            
            if snark:
                if "Proof size" in line:
                    match = re.search(r'Proof size: (\d+)', line)
                    if match:
                        pf_s = int(match.group(1))
                        
                        current_data["Proof size (bytes)"] = pf_s
                        all_data["Proof size (bytes)"] += pf_s
                
                elif "Verifier time" in line:
                    match = re.search(r'time: (\d+)', line)
                    if match:
                        time_value = int(match.group(1))
                        current_data["Time busy verifier (ms)"] = round(time_value)
                        all_data["Time busy verifier (ms)"] += round(time_value)
            else:
                if "close time.busy" in line:
                    match = re.search(r'close time\.busy=([\d\.]+)([a-zµ]*)', line)
                    if match:
                        time_value = float(match.group(1))
                        unit = match.group(2)
                        
                        if unit == "s":
                            time_value *= 1000  # convert seconds to milliseconds
                        elif "µ" in unit:
                            time_value /= 1000  # convert microseconds to milliseconds
                        
                        if "verify" in line:
                            current_data["Time busy verifier (ms)"] = round(time_value)
                            all_data["Time busy verifier (ms)"] += round(time_value)
                        elif "prove_core" in line:
                            current_data["Time busy prover (ms)"] = time_value
                            all_data["Time busy prover (ms)"] += time_value
    
    # Calculate mean values
    for key in all_data:
        if key != "Run":
            all_data[key] /= (len(results)+1)  # Calculate mean values

            if "cycles" in key or "size" in key:
                all_data[key] = int(all_data[key])
            else:
                all_data[key] = round(all_data[key], 2)

    # Append last data
    if current_data:
        results.append(current_data)
        
        results.append({})  # Empty line between data
        results.append(all_data)
    
    return results

def scrape_risc0(log_file: str) -> list:
    results = []
    all_data = {
        "Run": "mean",
        "Total cycles": 0,
        "Proof size (bytes)": 0,
        "Time busy prover (ms)": 0,
        "Time busy verifier (ms)": 0,
    }

    with open(log_file, 'r', encoding='utf-8') as file:
        log_content = file.read()
        lines = log_content.splitlines()
    
        input_value = None
        total_cycles = None
        proof_size = None
        prover_time = None
        verifier_time = None

        for line in lines:
            if "Run: " in line:
                print(line)
                input_value = int(line.strip().split(" ")[1].replace(";", ""))
            elif "Total Cycles:" in line:
                total_cycles = int(re.search(r'Total Cycles: (\d+)', line).group(1))
            elif "Proof size:" in line:
                proof_size = int(re.search(r'Proof size: (\d+)', line).group(1))
            elif "Prover time:" in line:
                prover_time = float(re.search(r'Prover time: ([\d.]+)', line).group(1))
            elif "Verifier time:" in line:
                verifier_time = float(re.search(r'Verifier time: ([\d.]+)', line).group(1))  # Convert ms to seconds
            
            if all(v is not None for v in [input_value, total_cycles, proof_size, prover_time, verifier_time]):
                results.append({
                    "Run": input_value,
                    "Total cycles": total_cycles,
                    "Proof size (bytes)": proof_size,
                    "Time busy prover (ms)": prover_time,
                    "Time busy verifier (ms)": verifier_time
                })
                
                # update all_data
                all_data["Total cycles"] += total_cycles
                all_data["Proof size (bytes)"] += proof_size
                all_data["Time busy prover (ms)"] += prover_time
                all_data["Time busy verifier (ms)"] += verifier_time

                input_value = total_cycles = proof_size = prover_time = verifier_time = None

            else:
                print(input_value, total_cycles, proof_size, prover_time, verifier_time)

    for key in all_data:
        if key != "Run":
            all_data[key] /= len(results)  # Calculate mean values

            if "cycles" in key or "size" in key:
                all_data[key] = int(all_data[key])
            else:
                all_data[key] = round(all_data[key], 2)
        
    results.append({})  # Prázdný řádek jako oddělovač
    results.append(all_data)

    return results


# Write data to CSV file
def write_to_csv(data, output_file):
    with open(output_file, 'w', newline='', encoding='utf-8') as file:
        writer = csv.DictWriter(file, fieldnames=data[0].keys())
        writer.writeheader()
        writer.writerows(data)


# Parse arguments
def parse_arguments() -> tuple:
    zkvm = ""

    parser = argparse.ArgumentParser(description="Enter input value (int number) and type of function.")
    parser.add_argument("dir", help='Input/output directory.')
    parser.add_argument('-R', '--risc0', action='store_true', help='Run RISC Zero zkVM.')
    parser.add_argument('-S', '--sp1', action='store_true', help='Run SP1 zkVM.')
    parser.add_argument('-s', '--snark', action='store_true', help='Run SP1 zkVM with SNARK proover.')

    args = parser.parse_args()

    print(args)

    if args.risc0:
        zkvm = "risc0"
    elif args.sp1:
        zkvm = "sp1"
    else:
        print(args.R, args.S)
        print("You must provide a zkVM type.")
        exit()
    

    return (args.dir, zkvm, args.snark)


def list_dir(path: str) -> list:
    """
    List all files in the given directory.
    """
    import os
    return os.listdir(path)

if __name__ == "__main__":
    # read values from stdin:
    path, zkvm, snark = parse_arguments()

    print(list_dir(path))

    for file in list_dir(path):
        if ".txt" in file:
            log_path = path + "/" + file
            output_csv = path + "/" + file.replace(".txt", ".csv")

            scraped_data = []
            if zkvm == "sp1":
                scraped_data = scrape_sp1(log_path, snark)
            else:
                scraped_data = scrape_risc0(log_path)
            write_to_csv(scraped_data, output_csv)
            
            print(f"Saved: {output_csv}")    
    
