# ![L5R-Name-Generator](https://fontmeme.com/permalink/210209/1bb0abe026dcbe92c0921cac6d631be8.png)

Randomly generates a number of Japanese names based on Legend of the Five Rings roleplaying game.

## Installation

This code is built using [cargo](https://doc.rust-lang.org/cargo/). Run by typing ``cargo run`` on the command line.

## Usage

The program displays on the command line. Upon running, you will be given the option to generate names or add a name.

### Generate Name

Press `g` on the command line. You will then be asked to choose a clan. Follow the prompt on the command line to select the appropriate letter for the clan. You can then select a surname for the given clan. Next you will be prompted to select how many names to generate. Finally, you will select a sex.Then the name(s) will be displayed on the command line.

#### Examples

```
Input [g] to generate name or [a] to add name. Press any 
        other key to exit.
g
```
```
Choose a clan:
    [c]rab
    cra[n]e
    [d]ragon
    [l]ion
    [p]hoenix
    [s]corpion
    [u]nicorn
    [r]andom
c
```
```
Choose a clan name : 
	1 -- Hida
	2 -- Hiruma
	3 -- Kaiyū
	4 -- Kuni
	5 -- Yasuki
	r -- random name
r
	Kanji: 昼間
	Kunyomi: Hiruma
	Onyomi: Hiruma
	English: Dawn Room
```
```
How many names do you want to generate? :
2

[m]ale or [f]emale names: 
m
```
```
Name 1 ----------
	Kanji: 昼間 五郎
	Kunyomi: Hiruma Satsuira
	Onyomi: Hiruma Gorō
	English: Dawn Room Five Son
Name 2 ----------
	Kanji: 昼間 厡太
	Kunyomi: Hiruma Okataka
	Onyomi: Hiruma Genta
	English: Dawn Room Original Great
```

### Add a Name

Press `a` on the command line. You will then be asked whether you are adding a clan name or personal name. If you select a clan name, you will need to specify which L5R clan to add the name to. For any name, you will need to supply a certain number amount of information, and be prepared to input kanji and transliteration information.

### Exit

Press any key other than `g` or `a` to exit the program.

### License

This project is covered

