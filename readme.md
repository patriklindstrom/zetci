**tl;dr This cmd program can compare text files and find what rows are the
same or differ**
```

                                                     ...............................................
                   &%%@(          /@%%%.        ..............%%%@/..........%@%%&..................
              %%@                        ,%%.............%%.........................@%%.............
           %@                         ......./%/.....,%/................................&%..........
        &%                       ...............@%.%&......................................%%.......
      &%                   ......................%@%.........................................&%.....
     %              ...........................(%...%#........................................,%....
   /%        .................................&&.....@%.........................................%&..
  .% ........................................@&.......@&.........................................%*.
..%..........................................%.........%.........................................,%.
.%@.........................................&/.........&%.........................................@&
.&&.........................................%...........%.........................................#%
.&@.........................................%...........%.........................................%%
./%.........................................@@.........@&.........................................&%
  %.                                   ......%.........%.........................................(%.
   %                                         (%       %#                                         %
    %                                         /%     %#                                         %
     %&                                         %  .%                                         %%
       %                                         &%@                                        *%,
         %&                                     %# /%                                     &%.
           &%#                               %%       &%                               &%&
               %%&                      %%%.              %%*                      %%%
                    #%%%&&@/,**/@&%%%&                         &%%%&@(**,/@@&%%%%
          /$$$$$  /$$$$$$  /$$$$$$ /$$   /$$       /$$$$$$$$ /$$$$$$$$ /$$$$$$$$  /$$$$$$  /$$$$$$
         |__  $$ /$$__  $$|_  $$_/| $$$ | $$      |_____ $$ | $$_____/|__  $$__/ /$$__  $$|_  $$_/
            | $$| $$  \ $$  | $$  | $$$$| $$           /$$/ | $$         | $$   | $$  \__/  | $$
            | $$| $$  | $$  | $$  | $$ $$ $$          /$$/  | $$$$$      | $$   | $$        | $$
       /$$  | $$| $$  | $$  | $$  | $$  $$$$         /$$/   | $$__/      | $$   | $$        | $$
      | $$  | $$| $$  | $$  | $$  | $$\  $$$        /$$/    | $$         | $$   | $$    $$  | $$
      |  $$$$$$/|  $$$$$$/ /$$$$$$| $$ \  $$       /$$$$$$$$| $$$$$$$$   | $$   |  $$$$$$/ /$$$$$$
       \______/  \______/ |______/|__/  \__/      |________/|________/   |__/    \______/ |______/

```
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
* **files** Sets the input file to use
* **union** Performs union operation on csv files
* **intersect** Performs intersection operation on csv files
* **difference**  Performs difference operation on csv files

# About sets and its operations
We want eg the following set operations:
not A and B => DiffFile  see [Explanation of expression](http://www.wolframalpha.com/input/?i=not+A+and+B "link to Wolframealpha") or see more easily [Venn diagram](http://www.wolframalpha.com/share/clip?f=d41d8cd98f00b204e9800998ecf8427e41kvo33uui "link to graph on Wolframealpha")
A and B => IntersectionFile see [Explanation of expression](http://www.wolframalpha.com/input/?i=A+and+B "link to Wolframealpha") or see more easily [Venn diagram]( http://www.wolframalpha.com/share/clip?f=d41d8cd98f00b204e9800998ecf8427e7e2qko5194 "link to graph on Wolframealpha")
NotaBene combined => (not A and B) or (A and B) see [Explanation of expression](http://www.wolframalpha.com/input/?i=%28not+A+and+B%29+or+%28A+and+B%29 "link to Wolframealpha") or see more easily [Venn diagram](http://www.wolframalpha.com/share/clip?f=d41d8cd98f00b204e9800998ecf8427eguh00j5eik "link to graph on Wolframealpha")
DiffFile+IntersctionFile => FileB
## Example
### Simple intersection
Make an intersection between file a and b the key are in column 4,6,7 seperator in the csv files a and b are semicolon (;) make it verbose.
> zetci -v -i -a"s:\Darkcompare\A_TestFile.cs"  -b"s:\Darkcompare\B_TestFile.cs" -k4 6 7 -s;
Shown in Venn Diagram it would be:
![Link to Venn Diagagram showing A and B](http://i.imgur.com/lNnPvV2.png)
In pseudo SQL it would be something like:
> SELECT a.* from A_TestFile as a INNER JOIN B_TestFile as b on a.k4=b.k4 and a.k6=b.k6 and a.k7=b.k7

### Many Setoperation on two sets
On the two sets A_TestFile and B_TestFile defined by the key on column 1 and 2 where the column separator is semicolon (;)
make sets that are DiffB and DiffA and the intersection of the two. Describe all in a verbose style.
>zetci -v  -a".\A_TestFile.csv"  -b".\B_TestFile.csv" -k1 2 -s; -r -d -i

In Venn Diagram
DiffB  ![Venn Diagram Showing DiffB operation](http://i.imgur.com/Ig0o6mf.png), DiffA ![Venn diagram DiffA](http://i.imgur.com/9DK6QlX.png), Intersection ![Link to Venn Diagagram showing A and B](http://i.imgur.com/lNnPvV2.png)

In pseudo SQL it would be something like:
> SELECT b.* from B_TestFile as b  WHERE b.key_1_2 not in (Select a.key_1_2 from A_TestFile as a)

DiffA
> SELECT a.* from A_TestFile as a  WHERE a.key_1_2 not in (Select a.key_1_2 from B_TestFile as b)

Intersection
> SELECT a.* from A_TestFile as a  INNER JOIN B_TestFile as b ON (a.key_1_2 = b.key_1_2 )

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
### Define multiple columns as key
### Compress large files data
### Guess format of datafiles when you use keys
### Parallelize the workload, eg read files in parallel, hashmaps in parallel.
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
