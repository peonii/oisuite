# oisuite

Your number #1 tool to managing your algo questions!
This software only works on UNIX-based operating systems (macOS, Linux, BSD, etc.)

- [x] Project generation
- [x] Quick project template updates
- [x] Good template with functional Makefile
- [x] Testing capabilities
- [x] Timing answers and time requirements
- [x] Test packages
- [ ] Test package grouping
- [ ] Custom config `oisuite.yml`
- [ ] Multithreading test generation / execution

## Dependencies

For oisuite, you will need:
- Make
- g++
- git
- cargo (for building the program)
- a POSIX-compliant shell

## Installation

You can install oisuite by cloning this repository, and follow the following steps:
- Install [rustup](https://rustup.rs/)
- Change the rust compiler to nightly:
```sh
$ rustup install nightly
$ rustup update
$ rustup default nightly
$ mkdir ~/bin
```
- Add `~/bin` to your PATH
- Run `install.sh`

The project should automatically install onto your machine.

After that, make sure to run `oisuite install` to add all of the required files.


## License

This software is available under the MIT license.
