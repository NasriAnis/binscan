Thank you for your interest in contributing to `binscan`.

### Project structure:
```
binscan
├── Cargo.toml
├── crates
│   ├── cli
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── commands
│   │       │   ├── analyzer_cli.rs
│   │       │   ├── extractor_cli.rs
│   │       │   ├── mod.rs
│   │       │   ├── parsing.rs
│   │       │   ├── process_cli.rs
│   │       │   ├── request_cli.rs
│   │       │   └── response_cli.rs
│   │       └── main.rs
│   └── library
│       ├── Cargo.toml
│       └── src
│           ├── analyzer
│           │   ├── analyze
│           │   │   ├── analyze_strings.rs
│           │   │   └── detect_compiler.rs
│           │   ├── analyze.rs
│           │   └── mod.rs
│           ├── db
│           │   ├── json_req.rs
│           │   ├── make_req.rs
│           │   ├── mod.rs
│           │   ├── request.rs
│           │   └── response.rs
│           ├── extractor
│           │   ├── extract
│           │   │   ├── elf.rs
│           │   │   ├── pe.rs
│           │   │   └── string.rs
│           │   ├── extract.rs
│           │   └── mod.rs
│           └── lib.rs
└── README.md
```

For some files name there it is obvious what they do, but for others more informations has to be given:

`./crate/library/src/db`: this directory contains the functions necessary to communicate with the OSV API. It create the requests send then and receive the response.

`./crate/library/src/db/json_req.rs`: this file contain a function that takes the data output from the analyzer and pack them into a json format the API understands.

`./crate/cli/src/commands`: this directory contains wrapper functions for the library fuctions

### Contributing requirement:
1. No AI slop is accepted
2. Every pull request has to be documented and only fix one thing at a time.
3. These two commands have to be executed before opening the pull request:
```
# cargo fmt --all // fix the code formatting
# cargo clippy --workspace --all-targets --all-features -- -D warnings // and do the fixes it suggest
```