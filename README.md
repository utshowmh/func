# Func

Func (this name ain't final) is a [interpreted](https://en.wikipedia.org/wiki/Interpreter_(computing)) toy [programing language](https://en.wikipedia.org/wiki/Programming_language) of mine.

---

## Grammer

The [Grammer](https://en.wikipedia.org/wiki/Context-free_grammar) for func is defined [here](https://github.com/utshowmh/func/blob/main/GRAMMER.md).

---

## Building Func

In order to build this project, you need to have [git](https://git-scm.com/downloads) and [rust](https://www.rust-lang.org/tools/install) installed on your system. Then you'll be able to [clone this repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository). After cloning, you'll have to [cd](https://en.wikipedia.org/wiki/Cd_(command)) to 'func' and [build it with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html). If you want some strait forward commands, you can run these:

```
sudo apt install git -y
sudo apt install rust -y
git clone https://github.com/utshowmh/func.git
cd func
cargo build --release
```

After building the project, you'll find a [binary](https://en.wikipedia.org/wiki/Executable) named 'func' in './target/release'.
