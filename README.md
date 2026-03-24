# `pure_mbr`
A Rust library to parse the [master boot record (MBR)](https://en.wikipedia.org/wiki/Master_boot_record).

## Features
- `no_std` without `alloc`
- No `unsafe` code
- Very minimal

## Usage
- Read the first 512 bytes of the disk
- Use `zerocopy` to safely transmute the 512 bytes into a `GenericMbr`
- Access the `partition_entries` field
- Use the `is_empty` and `partition_type` methods to check if the partition exists and is the format you are expecting (typically FAT)
- Use the `start_sector` and `n_sectors` methods to know the range of the partition
- Optional: you can use the [`pure_fat`](https://github.com/ChocolateLoverRaj/pure_fat) library to read folders and files from the partition

## Use cases
- Reading an SD card formatted with MBR in embedded
