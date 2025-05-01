import os
import csv
import argparse

# read last line of csv file
def read_last_line(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        return file.readlines()[-1].replace("\n", "").replace("\r", "")

# Parse arguments
def parse_arguments() -> tuple:
    parser = argparse.ArgumentParser(description="Merge csv files into one.")
    parser.add_argument("input_dir", help="The input/output directory.")
    args = parser.parse_args()

    return args.input_dir


# write data to csv file, append to existing file
def write_to_csv(data, file_path):
    with open(file_path, 'a', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        writer.writerow(data)


def list_csv_files(directory):
    """
    List all CSV files in the given directory.
    """
    import os
    return list(reversed([f for f in os.listdir(directory) if (f.endswith('.csv') and ("log" in f))]))


if __name__ == "__main__":
    log_path = parse_arguments()

    files = list_csv_files(log_path)
    files = sorted(files, key=lambda x: int(x.split('_')[-1].split('.')[0]))
    print(files)

    output_file = log_path + "/complet.csv"
    
    # first remove file if it exists
    if os.path.exists(output_file):
        os.remove(output_file)

    write_to_csv(["Input", "Total cycles", "Proof size (bytes)", "Time busy prover (ms)", "Time busy verifier (ms)"], output_file)

    for file in files:
        const = int(file.split("_")[1].split(".")[0])
        data = read_last_line(log_path + "/" + file)
        data = data.split(",")
        data[0] = str(const)
        
        print(data)
        write_to_csv(data, output_file)
