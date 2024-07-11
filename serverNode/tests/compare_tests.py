def filter_get_commands(input_file):
    filtered_commands = []
    with open(input_file, 'r') as f:
        for idx, line in enumerate(f, start=1):
            line = line.strip()
            if line.startswith('GET'):
                filtered_commands.append((idx, line))
    return filtered_commands


def compare_files(file1, file2):
    commands_file1 = filter_get_commands(file1)
    commands_file2 = filter_get_commands(file2)

    if len(commands_file1) != len(commands_file2):
        print(f"Files {file1} and {file2} have a different number of GET commands.")
        print(f"File {file1} has {len(commands_file1)} GET commands and file {file2} has {len(commands_file2)} get commands")
        return

    for (line_num1, command1), (line_num2, command2) in zip(commands_file1, commands_file2):
        if command1 != command2:
            print(f"Files {file1} and {file2} differ at line {line_num1} and {line_num2}.")
            print(command1)
            print(command2)
            return

    print(f"Files {file1} and {file2} have the exact same GET commands in the same order.")


if __name__ == "__main__":
    file1 = 'test.out'  # Replace with your file paths
    file2 = './tests/test_0.correct'  # Replace with your file paths
    compare_files(file1, file2)
