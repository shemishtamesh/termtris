# termtris
A tui tetris game.

![gameplay_demo](./examples/demo.gif)

## requirements
on linux, some keybaord events wouldn't get handled unless you use one of these terminals:
  * [kitty terminal](https://sw.kovidgoyal.net/kitty/)
  * [alacritty terminal](https://alacritty.org/)
  * [foot terminal](https://codeberg.org/dnkl/foot)
  * [WezTerm terminal](https://wezfurlong.org/wezterm/index.html)
  * [rio terminal](https://raphamorim.io/rio/) (after enabaling use-kitty-keyboard-protocol)

because support for [kitty's keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) is needed.

## running
```bash
$ git clone https://github.com/shemishtamesh/termtris
$ cd termtris
$ cargo run --release
```

