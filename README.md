**BinScan** is a lightweight internal utility designed to quickly scan, parse, and analyze `firmwares, binaries`, providing fast, reliable output with minimal overhead.

### Installation:

This proect can be installed by downloading the standalone binary for your operating system in the releases page : https://github.com/NasriAnis/binscan/releases, then unziping it using this command :
```
# tar -xf binscan-*.tar.gz
```

Or if you wanna get the latest fixes clone the repo and build it locally using these commands:
```
# git clone https://github.com/NasriAnis/binscan.git  
# cd binscan
# cargo build --release
```

The standalone binary will be found in this location : `target/release/binscan`, In linux you can copy it tinto your `bin` folder using the command :
```
# cp target/release/binscan ~/.local/bin/
```

And use it from anywhere.

### Usage:

You can get help using this command : `binscan --help`, All the flags will be listed.

example:
https://github.com/user-attachments/assets/f6cc3b0e-d56a-46d5-874d-c2d88b893156

Note that the severity flag is not working yet.