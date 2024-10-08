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

## Comparison others

| Features     | [KeyCastr](https://github.com/keycastr/keycastr) | [Keyviz](https://github.com/mulaRahul/keyviz)    | [showmethekey](https://github.com/AlynxZhou/showmethekey) | Keyway                  |
|:------------:|:------------------------------------------------:|:------------------------------------------------:|:---------------------------------------------------------:|:-----------------------:|
| Platform     | MacOS                                            | MacOS, Windows, Linux(X11,Wayland)               | Linux(X11,Wayland)                                        | MacOS, Windows, Linux   |
| DisplayStyle | WhileInputting / Always                          | WhileInputting                                   | Always                                                    | WhileInputting / Always |
| Layout       | Horinzontal / Vertical                           | Vertical                                         | Horizontal                                                | Horizontal / Vertical   |
| Position     | Draggable                                        | Fixed(Right/Left/Top/Bottom and so on in screen) | Draggable                                                 | Fixed / Draggable       |
| Symbol       | Text / UnicodeSymbol(eg '⌥ ', '⇪' )              | Text / Flat / Elevated / Mechanical              | Text                                                      | ummm...                 |


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

# Device permission
If you use linux and wayland, we would recommend to create udev rules for keyway.
If you belong to `your_group` group, put following udev rules file on `/etc/udev/rules.d/`.

```udev: 99-keyway.rules
KERNEL=="event[0-9]*", SUBSYSTEM=="input", GROUP="your_group", MODE="0660", TAG+="uaccess"
```

After created it, you need to reboot pc to apply udev-rules.
If udev load valid 99-keyway.rules, you can execute `keyway` without sudo.
