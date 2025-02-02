###### *<div align="right"><sub>// design by Fifi</sub></div>*

---

**Viper** is a blazing-fast, customizable terminal emulator built in Rust. It’s designed to offer modern performance, visual elegance, and seamless configuration.

## 🚀 Features

- **Fast & Lightweight**: Built with Rust for maximum performance.
- **Customizable Themes**: Personalize your terminal with sleek, modern designs.
- **Advanced Keybindings**: Tailor shortcuts to your workflow.
- **Cross-Platform**: Runs on Linux, macOS, and Windows.

---

  <a href="#-installation"><kbd> <br> Installation <br> </kbd></a>&ensp;&ensp;
  <a href="#-keybindings"><kbd> <br> Keybindings <br> </kbd></a>&ensp;&ensp;
  <a href="#-contributing"><kbd> <br> Contributing <br> </kbd></a>
  
---

[![][image-preview]][viper-link]

## ⚙️ Installation

> [!IMPORTANT]
> Please note that it is necessary to install the `libgtk-4-dev` package for successful compilation.
> For more information about GTK-4, visit its documentation. [here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/).

To install, execute the following commands:

```shell
sudo apt install build-essential libgtk-4-dev
git clone --depth 1 https://github.com/Fifixex/viper
cd ~/viper
cargo run
```

## 💕 Contributing

Contributions to Viper are greatly appreciated! If you're interested in contributing, please read the [Contributing Guide](https://github.com/Fifixex/viper/blob/master/.github/CONTRIBUTING.md) **before submitting a pull request**.

## ⌨ Keybindings

| Keys | Action |
| :--- | :--- |
| <kbd>Super</kbd> + <kbd>Q</kbd><br><kbd>Alt</kbd> + <kbd>F4</kbd> | Close focused window|
| <kbd>Super</kbd> + <kbd>W</kbd> | Toggle the window between focus and float |
| <kbd>Super</kbd> + <kbd>G</kbd> | Toggle the window between focus and group |
| <kbd>Super</kbd> + <kbd>Shift</kbd> + <kbd>←</kbd><kbd>→</kbd><kbd>↑</kbd><kbd>↓</kbd> | Resize windows |
| <kbd>Super</kbd> + <kbd>LeftClick</kbd><br><kbd>Super</kbd> + <kbd>Z</kbd> | Move focused window |
| <kbd>Super</kbd> + <kbd>RightClick</kbd><br><kbd>Super</kbd> + <kbd>X</kbd> | Resize focused window |
| <kbd>Super</kbd> + <kbd>S</kbd> | Toggle to special workspace |
| <kbd>Super</kbd> + <kbd>J</kbd> | Toggle focused window split |
| <kbd>Super</kbd> + <kbd>Ctrl</kbd> + <kbd>H</kbd> | Move between grouped windows backward |
| <kbd>Super</kbd> + <kbd>Ctrl</kbd> + <kbd>L</kbd> | Move between grouped windows forward |

[image-preview]: https://github.com/user-attachments/assets/b7597228-cf68-4cf3-96ea-1fd8135024bc
[viper-link]: https://github.com/Fifixex/viper
