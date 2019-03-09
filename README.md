# Rust benchmarking on Large File IO

This is a short report on the performance metrics obtained processing large files with a small rust/python script.

In this case the definition of "Large" is files that won't fit in memory easily (e.g. 100GB >) and require streaming / buffers.

## Experiment overview

The experiment is as follows:

* 2GB text file containing text information about objects
* Each 3 consecutive lines has information about one object (i.e. 1 line = one attribute)
* Each object is separated by one blank line

## Objective

Objective is to read file, iterate through lines and write results to CSV/TSV

## Example

#### input file example

OBJECT 1 ATTR 1: CONTENT
OBJECT 1 ATTR 2: CONTENT
OBJECT 1 ATTR 3: CONTENT

OBJECT 2 ATTR 1: CONTENT
OBJECT 2 ATTR 2: CONTENT
OBJECT 2 ATTR 3: CONTENT

...etc

#### expected output file example

OBJECT 1 ATTR1, OBJECT 1 ATTR 2, OBJECT 1 ATTR 3 <br>
OBJECT 2 ATTR1, OBJECT 2 ATTR 2, OBJECT 2 ATTR 3
... etc


# Improvements

Currently there are clear optimisations required for the Rust code, as there are several string operations.

Ideally it would be possible to process the files in rust as u8 (byte) format to save time, which would accelerate the processing, but unfortunately the BufReader class doesn't seem to provide functionality to read the files as bytes directly.

# Results

The results are provided below.
 

## Python:

real    2m16.087s

user    1m4.397s

sys     0m4.352s


## Rust 1.33 main.rs:

real    7m28.602s

user    7m19.379s

sys     0m5.094s


## Rust 1.33 main-vec.rs:

real    8m35.463s                                                                                        

user    8m24.227s                                                                                        

sys     0m5.918s

