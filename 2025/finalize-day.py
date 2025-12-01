#!/bin/env python3

# run with sudo -E

import subprocess, os, sys, re, json, tempfile
import zlib


day = os.path.basename(os.getcwd())
binary_path = os.path.join("./target/release/", day)


def build_release():
    result = subprocess.run(["cargo", "build", "-r"], capture_output=True, text=True)
    result.check_returncode()


def sizeof_fmt(num, suffix="B"):
    for unit in ("", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"):
        if abs(num) < 1024.0:
            return f"{num:3.0f}{unit}{suffix}"
        num /= 1024.0
    return f"{num:.1f}Yi{suffix}"


def get_silver_gold(input_filename):
    with open(input_filename, "r") as f:
        result = subprocess.run([binary_path], stdin=f, capture_output=True, text=True)
    silver_match = re.search(r"silver: *(\d+)", result.stdout, re.IGNORECASE)
    gold_match = re.search(r"gold: *(\d+)", result.stdout, re.IGNORECASE)
    if not silver_match or not gold_match:
        raise Exception("No silver or gold output")
    silver = silver_match.groups(1)[0]
    gold = gold_match.groups(1)[0]
    return silver, gold


def extract_data(tmpfilename):
    with open(tmpfilename, "r") as f:
        dataj = json.load(f)
    results = dataj["results"][0]
    return results["mean"] * 1000, results["stddev"] * 1000


def cold_cache_hyperfine(input_filename):
    tmpfile = tempfile.mktemp()
    subprocess.run(
        [
            "hyperfine",
            "--input",
            input_filename,
            "--export-json",
            tmpfile,
            "--prepare",
            "sync; echo 3 | tee /proc/sys/vm/drop_caches",
            binary_path,
        ],
        capture_output=True,
        text=True,
    )
    return extract_data(tmpfile)


def warm_cache_hyperfine(input_filename):
    tmpfile = tempfile.mktemp()
    subprocess.run(
        [
            "hyperfine",
            "--export-json",
            tmpfile,
            "--input",
            input_filename,
            "-N",
            "-w",
            "5",
            binary_path,
        ],
        capture_output=True,
        text=True,
    )
    return extract_data(tmpfile)


def get_file_line_count(input_filename):
    with open(input_filename, "r") as f:
        return sum(1 for _ in f)


def get_file_size(input_filename):
    return sizeof_fmt(os.path.getsize(input_filename))


def get_file_crc32(input_filename):
    prev = 0
    for line in open(input_filename, "rb"):
        prev = zlib.crc32(line, prev)
    return f"{prev:X}"


def get_hyperfine_version():
    result = subprocess.run(["hyperfine", "--version"], capture_output=True, text=True)
    return result.stdout.strip()


def get_human_readable_large_number(number):
    suffixes = ["", "k", "M", "B"]
    index = 0
    while index < len(suffixes) - 1:
        if number % 1000 == 0:
            number //= 1000
            index += 1
        else:
            break
    return f"{number}{suffixes[index]}"


def main():
    # could be improved to format the markdown table
    input_files = sys.argv[1:]
    build_release()
    print(f"# {day}\n")
    print(
        "| description | download | CRC32 | size | lines | silver | gold | CCM [ms] | WCM [ms] |"
    )
    print(
        "| ----------- | -------- | ----- | ---- | ----- | ------ | ---- | -------- | -------- |"
    )
    for file in input_files:
        (silver, gold) = get_silver_gold(file)
        (wcm, wcstd) = warm_cache_hyperfine(file)
        (ccm, ccstd) = cold_cache_hyperfine(file)
        size = get_file_size(file)
        lines = get_file_line_count(file)
        crc = get_file_crc32(file)
        print(
            f"| {file} | -       | {crc} | {size} | {get_human_readable_large_number(lines)} | {silver} | {gold} | {ccm:.2f} ± {ccstd:.2f} | {wcm:.2f} ± {wcstd:.2f} |"
        )
    print("")
    print("- CCM = Cold-Cache Mean")
    print("- WCM = Warm-Cache Mean")
    print("")

    print("## Benchmarks")
    print(get_hyperfine_version())
    print("")
    print("Cold-cache:")
    print("```bash")
    print(
        "hyperfine --input input.txt --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' ./solution"
    )
    print("```")
    print("Warm-cache:")
    print("```bash")
    print("hyperfine --input input.txt -N -w 5 ./solution")
    print("```")


if __name__ == "__main__":
    main()
