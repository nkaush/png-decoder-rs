# png-decoder-rs
A PNG image encoder/decoder used to hide messages in PNG images.

1. [Usage Instructions](#Usage-Instructions)
2. [Example Usage](#Example-Usage)
3. [Build Instructions](#Build-Instructions)

# Usage Instructions
The executable this project produces has the capability to...
* [`encode`](#Encode) a secret message within a PNG image using a secret chunk type code
* [`decode`](#Decode) all secret messages within a PNG image using a secret chunk type code
* [`remove`](#Remove) all secret messages within a PNG image that use some secret chunk type code
* [`print`](#Print) each chunk within a PNG image

These functionalities correspond to the [`encode`](#Encode), [`decode`](#Decode), [`remove`](#Remove), and [`print`](#Print)
subcommands, respectively. The following blocks indicate how to use each subcommand.

## Encode:
```
USAGE:
    ./png encode <PATH> <CHUNK_TYPE> <MESSAGE>

ARGS:
    <PATH>          The path to the PNG image to encode a message within
    <CHUNK_TYPE>    The 4-byte chunk type code to use to add messages under
    <MESSAGE>       The message to encode

OPTIONS:
    -h, --help    Print help information
```

## Decode:
```
USAGE:
    ./png decode <PATH> <CHUNK_TYPE>

ARGS:
    <PATH>          The path to the PNG image to decode messages from
    <CHUNK_TYPE>    The 4-byte chunk type code to use to search for messages to decode

OPTIONS:
    -h, --help    Print help information
```

## Remove:
```
USAGE:
    ./png remove <PATH> <CHUNK_TYPE>

ARGS:
    <PATH>          The path to the PNG image to remove encoded messages from
    <CHUNK_TYPE>    The 4-byte chunk type code to use to search for messages to remove

OPTIONS:
    -h, --help    Print help information
```

## Print:
```
USAGE:
    ./png print <PATH>

ARGS:
    <PATH>    The path to the PNG image to print chunks for

OPTIONS:
    -h, --help    Print help information
```

# Example Usage
```
$ ./png decode images/dice.png ruSt
$ ./png encode images/dice.png ruSt 'This is a secret message.'
$ ./png decode images/dice.png ruSt                            
This is a secret message.
$ ./png print images/dice.png                                  
Chunk {
  Length: 13
  Type: IHDR
  Data: 13 bytes
  Crc: 804134823
}

...

Chunk {
  Length: 25
  Type: ruSt
  Data: 25 bytes
  Crc: 3261152786
}

Chunk {
  Length: 0
  Type: IEND
  Data: 0 bytes
  Crc: 2923585666
}

$ ./png remove images/dice.png ruSt                            
$ ./png decode images/dice.png ruSt
$ 
```

# Build Instructions

1. Clone this repository
2. Ensure Rust (and Cargo) are installed in your environment
3. Build the executable. A `Makefile` has been provided for your convenience.

```
git clone https://github.com/nkaush/png-decoder-rs.git
make
```

The following instructions will generate an executable called `./png` which you can use as shown in the sections above.