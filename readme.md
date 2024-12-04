# Advent of Code Solutions

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) challenges. Each year is
organized into its own folder, with the corresponding problems and solutions.

## Structure

- `year/2024/`: Folder for each year.
    - `day1/`, `day2/`, ...: Solutions for each day.
        - `testdata/` - Test input files
        - `go/`, `ts/`, `rust/` - Implementations
- Example: [Day 1](year/2024/day1)
    
## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/illiafox/adventofcode.git
   ```
2. Navigate to the appropriate folder:
   ```bash
   cd year/2024/day1
   ```
3. (Optional) Add your inputs into `testdata` folder
    ```bash
    cp /path/myinput.txt testdata/
    ```
4. Choose and run any available implementation:
    ```bash
   # For golang 
   go run go/main.go testdata/input.txt # or testdata/myinput.txt
    ```

# Contributing
Please refer to [CONTRIBUTING.md](CONTRIBUTING.md)
### Todo:
- [ ] Tests

## License
This repository is open-source and available under the [BSL-1.0 license](LICENSE).
