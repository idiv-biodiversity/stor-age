stor-age
========

[![Build Status](https://travis-ci.com/idiv-biodiversity/stor-age.svg?token=r7FuYXpRu1vR1bHxqWSV&branch=master)](https://travis-ci.com/idiv-biodiversity/stor-age)

Show aging of directories.


Table of Contents
-----------------

<!-- toc -->

- [Usage](#usage)
- [Installation](#installation)
  * [Arch Linux](#arch-linux)
  * [cargo install](#cargo-install)
  * [from source](#from-source)

<!-- tocstop -->


Usage
-----

Show how much of your directories have been lying around unused:

```console
$ stor-age 90 ~/media
/home/umcdev/media: 63.1 GiB
unaccessed for 90 days: 90% (56.6 GiB)
unmodified for 90 days: 99% (62.2 GiB)
```

See `--help` for more information on command-line usage.


Installation
------------

### Arch Linux

Install the [stor-age AUR package][aur-package]:

```bash
pacaur -S stor-age
```

### cargo install

```bash
cargo install stor-age
```

### from source

```bash
git clone https://github.com/idiv-biodiversity/stor-age.git
cd stor-age
cargo build --release
install -Dm755 target/release/stor-age ~/bin/stor-age
```


[aur-package]: https://aur.archlinux.org/packages/stor-age "stor-age AUR package"
