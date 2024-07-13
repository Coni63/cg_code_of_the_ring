import sys
import datetime
import subprocess
import time
from pathlib import Path

#  python full_bench.py rust
#  python full_bench.py rust_release
kind = sys.argv[-1]

if kind == "rust":
    subprocess.run(["cargo", "build"])
    cmd = ['target/debug/cg_code_of_the_ring.exe']
elif kind == "rust_release":
    subprocess.run(["cargo", "build", "--release"])
    cmd = ['target/release/cg_code_of_the_ring.exe']
else:
    print("Invalid argument")
    sys.exit(1)


test_files = [f"tests/{i}.txt" for i in range(1, 25)]

total_score = 0
for i, test_file in enumerate(test_files):
    start_time = time.time()

    result = subprocess.run(cmd, stdin=open(test_file, 'r'), capture_output=True, text=True)
    output_string = result.stdout.strip().split(" ")[0]
    
    end_time = time.time()

    score = len(output_string)
    time_limit_exceeded = end_time - start_time >= 5

    if score > 0 and not time_limit_exceeded:
        total_score += score
        print(f"Test {i+1}: 'OK' ({end_time - start_time:0.3f}) - {score} pts")
    elif score > 0 and time_limit_exceeded:
        total_score += score
        print(f"Test {i+1}: 'TIMEOUT' ({end_time - start_time:0.3f}) - {score} pts")
    else:
        print(f"Test {i+1}: 'FAIL'")

# Print the total length of all output strings
print(f"\n{len(test_files)} tests passed - Total score: {total_score} pts")