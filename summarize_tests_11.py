with open('cargo_test_run_output_11.txt', 'r') as f:
    lines = f.readlines()

filtered = [line for line in lines if 'test result:' in line or 'FAILED' in line or 'failures:' in line]

with open('test_summary_11.txt', 'w') as f_out:
    f_out.writelines(filtered)
