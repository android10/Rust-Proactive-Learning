## Rust Proactive Introduction

The code is a bit messy but it serves just as a personal Rust Playground to play around and understand the language. You can use to explore language feature and try things out. This `README.md` file will help you setup the IDE too. For managing the [Rust Programming Language](https://www.rust-lang.org/), I use [asdf](https://asdf-vm.com/).

## Run the project

 - `cargo run`

## Messy code structure

```rust
#[path="basic.rs"] mod basic;
#[path="intermediate.rs"] mod intermediate;
#[path="advanced.rs"] mod advanced;

fn main() {
    basic::basic_rust();
    intermediate::intermediate_rust();
    advanced::advanced_rust();
}
```

So the files to check and play around are:

 - `basic.rs`
 - `intermediate.rs`
 - `advanced.rs`

## Setup IDE (VSCode)

### Extensions

 - **rust-analyzer:** Supports code completion, go to definition, go to implementation and go to type definiton, syntax highlighting. You can find more about rust-analyzer at [the official website](https://rust-analyzer.github.io/manual.html).\
 - **crates:** To manage the dependencies with `Cargo.toml` file.
 - **CodeLLDB:** Supports native and remote debugging, disassembly, reverse debugging, core dump inspection. [Here more about codelldb](https://github.com/vadimcn/vscode-lldb).
 - **Better TOML:** Syntax hightlighting and validation for `.toml` files and `markdown` files.
 - **Error Lens::** Tool that helps you highlight errors, warnings and show other kinds of language diagnostics as soon as you make a change in the code.

### Themes

 - [One Dark Pro](https://marketplace.visualstudio.com/items?itemName=zhuangtongfa.Material-theme)
 - [Dracula](https://draculatheme.com/visual-studio-code)

### Font

 - [Source Code Pro](https://adobe-fonts.github.io/source-code-pro/)

### More IDE customization

 - [Setting up Rust in VSCode for Linux](https://nayabsd.com/setting-up-rust-in-vs-code-for-linux)
 - [Developing in Rust using Visual Studio Code](https://dev.to/thiagomg/developing-in-rust-using-visual-studio-code-4kkl)
 - [Rust recommended VSCode Extensions](https://www.becomebetterprogrammer.com/rust-recommended-vscode-extensions/)
 - [Best fonts for programming with VSCode](https://toastofcode.com/best-fonts-for-programming-with-vscode/)
 - [The best VSCode themes](https://medium.com/quick-code/the-best-vs-code-themes-2022-9e9b648c4596)

## Rust Programming Language Knowledge Base

 - [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
 - [How not to learn Rust](https://dystroy.org/blog/how-not-to-learn-rust/)
 - [Rust cheatsheet](https://cheats.rs/)
 - [Rust The Book](https://doc.rust-lang.org/stable/book/)
 - [Rust gentle intro](https://stevedonovan.github.io/rust-gentle-intro/)

## License

    Copyright 2022 Fernando Cejas

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.


![https://fernandocejas.com](https://github.com/android10/Sample-Data/blob/master/android10/android10_logo_big.png)

<a href="https://www.buymeacoffee.com/android10" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>
