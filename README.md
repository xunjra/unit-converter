# UNIT CONVERTER 
A fast, lightweight, offline unit converter built with Rust without the use of any external frameworks.

### Tech Stack 

- Rust
- ZED (IDE)

### Screenshots
<img width="1115" height="628" alt="image" src="https://github.com/user-attachments/assets/6ec441ec-d561-4069-b990-1aa9fe6bff65" />
<img width="1115" height="628" alt="image" src="https://github.com/user-attachments/assets/2f51139f-fd3d-49bf-ba82-10c67f149b73" />
<img width="1115" height="628" alt="image" src="https://github.com/user-attachments/assets/77b73571-61f4-4dd8-85de-e589c5fb4727" />



### Why was it built ? 
The project was built as a first project to start out to journey of rust since watching turtorials is not ENOUGH. The project helped me to learn the basic syntax and overall working with rust. On the project I have implemented REPL Loop, Handled Exceptions, Case-insensitive inputs,EOF handling, Validation with token count, Units check, Implemented physical limits exceptions such as absolute zero or negative length and weight, Seperated conversion logic into individual function & used an HUB based approach which allows to make just 2N functions for conversion instead of N^2 functions for same number of units to convert.

### Features

- Units Validation
- Exception Handeling
- Hub Based conversion approach 
- Great Performance ( Thanks to RUST )
- Case insensitive inputs handeling
- REPL Loop

## Performance

Measured with `/usr/bin/time -v` on a release build (`cargo build --release`), run under WSL2. The kernel records peak resident set size (RSS) for the process, so the numbers cover only this program, not the terminal.

| Scenario | Peak RSS | Wall time |
|---|---|---|
| 3 conversions, then `quit` | 2,320 KB (~2.3 MB) | instant |
| 1,000,000 conversions | 2,368 KB (~2.3 MB) | 0.96 s |
| One 1 MB input line | _TBD_ | _TBD_ |

**Highlights**

- **~2.3 MB peak memory**, about 10x under a 25 MB budget.
- **Flat memory use:** a million conversions added only ~48 KB over the 3-conversion run, so nothing accumulates between inputs.
- **~1 million conversions per second**, including parsing, validation, and output.
- **Standard library only**, with zero external dependencies.

**Notes**

- Peak RSS includes the binary and shared system libraries, so it slightly overstates the program's own heap use.
- The only input-dependent allocation is the line buffer, since `read_line` stores the full line.

**Reproduce**

```bash
cargo build --release

# 3 conversions
printf "km 5 mile\ncel 100 fah\nquit\n" | /usr/bin/time -v ./target/release/unit-converter > /dev/null

# 1,000,000 conversions
yes "km 5 mile" | head -n 1000000 | /usr/bin/time -v ./target/release/unit-converter 2>&1 >/dev/null | grep -E "Maximum resident|Elapsed"
```

Look for `Maximum resident set size (kbytes)` in the output.

## How to Use

### Download
Grab the file for your system from the [Releases page](https://github.com/xunjra/unit-converter/releases/tag/Rust):

| System | File |
|---|---|
| Windows (64-bit) | `unit-converter-windows-x86_64.exe` |

No installation or dependencies needed. It is a single standalone file.

### Run

**Windows**

Open PowerShell or Command Prompt (Terminal on Windows 11) in the folder with the file and run:
```powershell
.\unit-converter-windows-x86_64.exe
```


### Input format
Type one conversion per line at the `>>` prompt:

```
<from_unit> <value> <to_unit>
```

Example session:
```
>> km 5 mile
5 km = 3.11 mile
>> cel 100 fah
100 cel = 212.00 fah
>> quit
```

Type `help` for the full list of commands and units.

## Contributing

Contributions are welcome!
1. Fork the repository.
2. Make your changes.
3. Submit a pull request.



### License

This project is licensed under the MIT License.

##  Author
  Arjun Khanal (Xunjra)
- GitHub: https://github.com/xunjra
- Email: hello@xunjra.com
