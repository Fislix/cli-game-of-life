Command line implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) that I wrote to learn 🚀Rust🚀

You can either generate a random starting state or read a text file which contains a starting state
described by a grid of 0s (dead cells) and 1s (living cells). The file needs to end with a new line.

The input_examples folder contains some text files with common patterns/machines.

## Usage

Running this program with `--help` will show you how to use this tool:

```
Usage: cli-game-of-life.exe [OPTIONS]

Options:
  -W, --width <WIDTH>                  World width in number of characters. Required if <FILE> is not used
  -H, --height <HEIGHT>                World height in number of lines. Required if <FILE> is not used
  -p, --portion-alive <PORTION_ALIVE>  What portion of the cells should be alive in the randomly generated world [default: 0.3]
      --seconds <SECONDS>              How many seconds the game should run for (cannot be used with --steps)
      --steps <STEPS>                  How many simulation steps the game should run for (cannot be used with --seconds)
  -s, --speed <STEPS_PER_SECOND>       How many steps per second should (tried to) be computed [default: 2]
      --char-alive <CHAR_ALIVE>        The character to print for a living cell [default: ■]
      --char-dead <CHAR_DEAD>          The character to print for a dead cell [default: " "]
  -f, --file <FILE>                    Path to a text file, containing a starting state described by an n x m grid of 0s (dead cells) and 1s (living cells). Required if <WIDTH> and <HEIGHT> are not used 
  -h, --help                           Print help
```
