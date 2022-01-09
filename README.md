# png-decorder-rs
A PNG image encoder/decoder used to hide messages in PNG images.

# Usage
The executable this project produces has the capability to...
* [`encode`](#Encode) a secret message within a PNG image using a secret chunk type code
* [`decode`](#Decode:) all secret messages within a PNG image using a secret chunk type code
* [`remove`](#Remove:) all secret messages within a PNG image that use some secret chunk type code
* [`print`](#Print:) each chunk within a PNG image

These functionalities correspond to the [`encode`](#Encode), [`decode`](#Decode:), [`remove`](#Remove:), and [`print`](#Print:)
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
    png print <PATH>

ARGS:
    <PATH>    The path to the PNG image to print chunks for

OPTIONS:
    -h, --help    Print help information
```