#  ⌨ key way
`keyway` displays keystrokes on your desktop screen. It is designed to work on both Wayland and X11 environments and has the following features:

* Works on both Wayland and X11
* Implemented in Rust for high safety and no memory leaks
* Provides both CLI and GUI tools
* Minimal dependencies, avoiding the latest libraries as much as possible
* Customizable look and feel
* Hidden mode (easily toggle visibility for password input, etc.)

While there are other keystroke display programs like screenkey and showmethekey, they have limitations such as only working on X11 or requiring latest libraries(so User must use ArchLinux, Ubuntu 24.04 later and so on).

`keyway` was created to address the complexity and difficulty of existing keystroke display programs, which are often used by casual users. We believe that such tools should be easy to install and use, without requiring complex dependencies.


# Installation
## Ubuntu
sorry, I will prepare as soon as possible.
Support Ubuntu21.04 or later I will.

## Debian
sorry, I will prepare as soon as possible.


## ArchLinux
sorry, I will prepare as soon as possible.


## cargo
Support Ubuntu20.04 or later.

```sh
$ cargo install keyway
```

# Tasklist
## Appearance
- [ ] change fontsize
- [ ] change fontfamily
- [ ] change textcolor(RGBA)
- [ ] change background-color(RGBA)
- [ ] toggle background transparent
> Note
> Not working transparent by iced 0.12.1 on Wayland and Windows11, but cofirm to work it iced 0.13(dev).
> I will implement 'toggle background transparent' after iced 0.13 is stabled.
- [ ] select dynamic width or fixed width by keystrokes length
- [ ] select symbol or svg

## Misc Functions
- [ ] change interval time of continuous keystrokes
- [ ] toggle visible mouse click
- [ ] toggle visible modifier
