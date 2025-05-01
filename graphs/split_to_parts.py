import argparse

def split_to_parts(name, ext, dir_name, lines):
    """
    Split the lines into parts based on the run and input values.
    """
    last_run, prev_inp, fileLines = 0, 0, []
    for line in lines:
        if "Run" in line and "Input" in line:
            run, inp = getRunInput(line)

            if prev_inp == 0:
                prev_inp = inp

            if last_run > run or prev_inp != inp:
                save_file(name, ext, dir_name, prev_inp, fileLines)
                fileLines.clear()
            
            last_run, prev_inp = run, inp
        fileLines.append(line)
    
    # Save the last part
    save_file(name, ext, dir_name, prev_inp, fileLines)

def save_file(name, ext, dir_name, inp, lines):
    """
    Save the lines to a file with the given name and extension.
    """
    filename = f"{dir_name}/{name}_{inp}.{ext}"
    with open(filename, "w") as f:
        for line in lines:
            f.write(line)

def getRunInput(line):
    """
    Extract the `Run` and `Input` values from a line.
    """
    line = line.strip().split(" ")
    return int(line[1].replace(";", "")), int(line[3])


if __name__ == "__main__":
    """
    Split a file into multiple parts based on the run and input values.
    """
    parser = argparse.ArgumentParser(description="Split a file into multiple parts.")
    parser.add_argument("input_file", help="The input file to split.")
    parser.add_argument("output_dir", help="The input file to split.")
    args = parser.parse_args()

    name = args.input_file.split(".")[0]
    ext = args.input_file.split(".")[1]
    dir_name = args.output_dir
    
    # Read the input file
    with open(args.input_file, "r") as f:
        lines = f.readlines()

        split_to_parts(name, ext, dir_name, lines)
