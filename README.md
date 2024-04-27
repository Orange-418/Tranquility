# Tranquility

Tranquility is a custom stager and packer I wrote in Rust. It's mainly a side project I'm working on while I study for my OSEP certification. The tool is designed with payload encryption in mind, which means you can encrypt your payloads and store the keys on your own server.

I have largely abaondoned it, and am working on a more refined and full kit written in Python.

## Disclaimer

This is just a pet project of mine, so please use it responsibly in your own test environments. Also, if you're testing it out, it'd be great if you could turn off AV sample submissions, at least until 2025 after I have taken my OSEP exam.

## Building Tranquility

building from source:
Building is simple. Just go to the root of the folder and run 'cargo build --release'. Executable is found in target/release.

## Packer Usage:
The packer is made to pack raw/shellcode. For example, you can use an msfvenom payload, with the raw format.
Running tranquility_packer.exe -h displays this menu:

USAGE:
    tranquility_packer.exe --source <source> --target <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>    Source file to compress and encrypt
    -t, --target <target>    Target file for the encrypted output

It's pretty self explanatory. When you process a file, it will generate a random key and iv. the iv will be prepended to the encrypted file, and the key will be saved alongside your new file with ".key" appended to the end.
Just make sure you serve both your key file and encrypted payload into the same folder. You can go ahead and stick both of these files on your http server.


## Stager Usage:
The stager is made to fetch the created payload and execute it in mememory.
First, build the stager as outlined in the top of this readme.

Initial usage on my own machine yielding no AV detections, regardless how aggressive I was with my meterpreter session. Over time, meterpreter began getting killed when dropping into shells or migrating the process. Later, it would remove the stager immediately after compiling. YMMV.
