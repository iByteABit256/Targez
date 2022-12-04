# Targez
## A minimalistic version of the GNU tar command (gzip always enabled)


### Usage

```targez [OPTIONS] <MODE> [FILES]...```

### Arguments
- [MODE] What mode to run the program in

    Possible values:
    - extract:  Extract files from tar.gz file
    - compress: Compress files into tar.gz file

- [FILES] Input files/directories

### Options
- `-t`, `--target`: Target file

- `-h`, `--help`: Print help information

- `-V`, `--version`: Print version information

### Examples

Compression: ```targez -t myfiles.tar.gz compress ./file1.txt ./file2.java ./file3.json```

Extraction: ```targez -t . extract myfiles.tar.gz```