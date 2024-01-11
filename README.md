# My Rust competitive programming setup

## Requirements

most of these can be replaced by similar things but I based my personal setup on these

- [Just](https://github.com/casey/just) for making building and testing easier
- [Fish](https://fishshell.com/) for writing shell functions and abbreviations / aliases 
- [Kitty](https://sw.kovidgoyal.net/kitty/) as my terminal emulator
- [Delta](https://github.com/dandavison/delta) a better diff highlighter

## Setup

- at first you need to install the requirements above
- next you need to source the config.fish file inside of your main fish config (by default `~/.config/fish/config.fish`)
  for this you need to add the line `source <this directory>/config.fish`
- add the following lines to your `~/.gitconfig` file to configure delta correctly:
  ```toml
  [core]
  	pager = delta
  [delta]
  	side-by-side = true
  	line-numbers = false
  	file-style = omit
  	hunk-header-style = omit
  ```
