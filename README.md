stor-age
========

[![Build Status](https://travis-ci.com/idiv-biodiversity/stor-age.svg?token=r7FuYXpRu1vR1bHxqWSV&branch=master)](https://travis-ci.com/idiv-biodiversity/stor-age)

Show aging of directories.


Table of Contents
-----------------

<!-- toc -->

- [Motivation](#motivation)
- [Usage](#usage)
- [Installation](#installation)
  * [Arch Linux](#arch-linux)
  * [cargo install](#cargo-install)
  * [from source](#from-source)

<!-- tocstop -->


Motivation
----------

The main motivation of this tool is to find out, for a given set of data
directories, how much of the *storage capacity* is still in active use. This
information can be used on data processing systems, like High-Performance
Computing (HPC) clusters, to motivate users to migrate their unused data to
long-term storage facilities.

The file system metadata available for this analysis are the *access time* and
the *modification time* of the files in the data directory. Using this metadata
and given a set of *ages*, the percentage of accessed and modified capacity of
the total can be calculated for each of these ages and directories. Doing this
over time, a usage profile of the data directories can be created. Users can be
shown, when their data on the system gets *stale* and should be migrated to a
storage facility suited for long-term archival.


Usage
-----

Show how much of your directories have been lying around unused:

```console
$ stor-age 90 -- ~/media
/home/umcdev/media: 63.1 GiB
unaccessed for 90 days: 90% (56.6 GiB)
unmodified for 90 days: 99% (62.2 GiB)
```

Iterate over set of directories with find-like tools.

```bash
find /data/ -mindepth 1 -maxdepth 1 -type d | stor-age 90 365
fd  --max-depth 1 --type d . /data | stor-age 90 365
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
