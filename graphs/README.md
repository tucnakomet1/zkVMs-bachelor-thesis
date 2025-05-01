# Graphs

Example usage:

We have [log-risc0.txt](log-risc0.txt). First, we call [split_to_parts.py](split_to_parts.py) using:
```bash
python3 split_to_parts.py example/log-risc0.txt example/out
```
Which splits the log file into parts by the input. Then, we can run [scraper.py](scraper.py) using:
```bash
python3 scraper.py -R example/out
```
Which convert the file into csv only with usable values. Finally, we run [complet_to_csv.py](complet_to_csv.py) using:
```bash
python3 complet_to_csv.py example/out
```
Which merges all the csv files into one file by taking average values.