# Sudoku Solver

### **Program to solve sudokus given in JSON Format, written in Rust**

<br>

### Sources
> https://www.youtube.com/watch?v=VPVtlODPdPY

> https://www.surfpoeten.de/apps/sudoku/generator/

<br>

Solving this

| | | | | | | | | |
|-|-|-|-|-|-|-|-|-|
|0|0|0|0|9|0|0|0|0|
|0|3|2|0|0|7|0|0|0|
|0|0|0|0|0|5|0|3|0|
|0|0|0|0|6|0|9|0|7|
|6|0|0|0|0|4|3|0|0|
|0|0|0|7|2|0|0|0|4|
|0|0|0|0|0|0|4|0|3|
|1|0|0|8|0|0|0|0|0|
|4|6|0|0|0|2|0|5|0|

to this

| | | | | | | | | |
|-|-|-|-|-|-|-|-|-|
|7|8|6|2|9|3|1|4|5|
|5|3|2|4|1|7|6|9|8|
|9|1|4|6|8|5|7|3|2|
|2|4|5|3|6|1|9|8|7|
|6|7|8|9|5|4|3|2|1|
|3|9|1|7|2|8|5|6|4|
|8|2|9|5|7|6|4|1|3|
|1|5|3|8|4|9|2|7|6|
|4|6|7|1|3|2|8|5|9|

- took  2.0449473s without logging

- took 68.9601746s with logging