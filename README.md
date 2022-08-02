# ellipse

Command that truncates and ellipses strings in a human-friendly way.

## How to use

```
$ echo very long sentence |ellipse -n 12
very long se…
```

## How to install

Installation instructions are provided for ArchLinux only. When using a different
operation system, adoption to their package manager is left as an exercise to the
reader.

### Install from AUR (recommended)

This example uses `yay`, but you can use your favourite AUR helper instead.

```bash
yay -S ellipse
```

### Install via cargo

First, install rust/cargo:

```bash
sudo pacman -S rust
```

Then, install with cargo:

```bash
cargo install ellipse
```

This will only work if you have cargo's binary install directory added to your `$PATH`
variable.
