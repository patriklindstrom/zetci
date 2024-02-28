**tl;dr This cmd program can compare text files and find what rows are the
same or differ**

# Readme zetci

# How to use it
## What problem does it solve?
When you have data in two or more files and want to extract parts of if. In
similar way that you would from a sql database. It could be only the unique
rows in several files or the differences between two files or what has been
added to a file compared to previous version.

## Why not use a sql database ?
Absolutely if you have sql database use that. But if you do not have access to
a database or this is something that is just something you have to do once it
could be simpler solution. It can also be part of an ETL process if you only
want to import new rows and you get 50 GB with 10 million rows datadumps periodically  with all their data from
the source system. You could then do except between previous datadump and the
new one to get these precious 500 new rows that has been added since last time and only import them.
for this solution to work you need plenty of ram since zetci does all in ram.

## Simple example
`zetci fileA.csv fileB.csv fileC.csv`
will to stdoutput show a union of all unique rows. So if two rows are the same
only one of them will be in the output. Union is the default subcommand.

`zetci intersect --files fileA.csv fileB.csv fileC.csv --output ~/temp/whats-the-same.csv`
only what rows that are in all of the files will be output to a textfile.

`zetci intersect --files fileA.csv key=1,2,3 ft=csv fileB.csv key=7,8,9  ft=csv fileC.csv key=1,2,3  ft=csv --output ~/temp/whats-the-same.csv`
only what rows that are all of the files and also we define what fields holds
the unique key for the row and how the fields are defined and separeted.

## More advanced examples
`zetci except --files fileA.csv --exceptfile fileB.csv --output ~/temp/whats-new.csv`
Will show what is in the except file fileb.csv bot not in any of the others
files which in this case is just fileA.csv

## Really advanced example
## What parameters does it understand
* files Sets the input file to use
* union Performs union operation on csv files
* intersect Performs intersection operation on csv files
* difference  Performs difference operation on csv files
# How is the program written
## Testdriven
## Hashmaps
## General walk through
## What libraries does it use
### Clapper - arguments handler
### Terminal colors

# Roadmap
## Goals of this program.
### What it should be good at.
Fast, simple
### What is not should be good at.
Compete with sql databases
## Features Request.
### Sort output file
### compress large files that has key
### Guess format of datafiles when you use keys
## Todos
checkout right hash algoritmh. The default one for hashmap is not great for
small och large date. 
# How to contribute
## These are the groundrules

# Dev notes
to test params cargo run -- --files ./testdata/fee.csv ./testdata/foo.csv
./target/debug/zetci   --files './testdata/fee.csv' './testdata/foo.csv' union

                // TODO: Add function that takes array of hashmaps with data from files and
                // performs the operation union on them. move function later to library.
                // TODO: The array of function needs to be a struct perhaps with meta data about
                // the data like which one is the biggest, cardinality and perhaps others so the
                // rudimentary queriy optimizer gets relevant info.
