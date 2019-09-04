stor-age
========

[![Build Status](https://travis-ci.com/idiv-biodiversity/stor-age.svg?token=r7FuYXpRu1vR1bHxqWSV&branch=master)](https://travis-ci.com/idiv-biodiversity/stor-age)

Show aging of directories.


Table of Contents
-----------------

<!-- toc -->

- [Motivation](#motivation)
- [Usage](#usage)
- [Features](#features)
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

### Interactive

Show how much of your directories have been lying around unused:

```console
$ stor-age 90 365 -- ~/media/pics ~/projects

Directory             Total     Age Accessed  Percent Modified  Percent
/home/user/media/pics 222.9 MiB  90 715.0 kiB   0.31% 382.5 kiB   0.17%
                                365   7.4 MiB    3.3%   1.1 MiB    0.5%
/home/user/projects     4.0 GiB  90   3.4 GiB  84.78%   2.5 GiB  62.79%
                                365   3.7 GiB  92.64%   2.7 GiB  66.94%

```

**Note:** The two dashes `--` are *required* because you can supply both
*multiple ages* and *mulitple directories* and the command-line argument parser
needs a way to distinguish these two lists.

### Metrics for Monitoring Systems

Iterate over a set of directories with `find`-like tools:

```bash
find /data/ -mindepth 1 -maxdepth 1 -type d |
  stor-age --format prometheus 90 365
```

The output is in valid [Prometheus][] metric exposition format:

```
# HELP stor_age_bytes_total Total size.
# TYPE stor_age_bytes_total gauge
stor_age_bytes_total{dir="/data/foo"} 6564000342
stor_age_bytes_total{dir="/data/bar"} 4453853339
stor_age_bytes_total{dir="/data/baz"} 1229901478

# HELP stor_age_bytes_accessed Accessed size.
# TYPE stor_age_bytes_accessed gauge
stor_age_bytes_accessed{dir="/data/foo",age="90"} 5887683522
stor_age_bytes_accessed{dir="/data/foo",age="365"} 6496950716
stor_age_bytes_accessed{dir="/data/bar",age="90"} 3728333714
stor_age_bytes_accessed{dir="/data/bar",age="365"} 4269130140
stor_age_bytes_accessed{dir="/data/baz",age="90"} 1229849194
stor_age_bytes_accessed{dir="/data/baz",age="365"} 1229896439

# HELP stor_age_bytes_modified Modified size.
# TYPE stor_age_bytes_modified gauge
stor_age_bytes_modified{dir="/data/foo",age="90"} 5607932802
stor_age_bytes_modified{dir="/data/foo",age="365"} 6432800333
stor_age_bytes_modified{dir="/data/bar",age="90"} 1506205504
stor_age_bytes_modified{dir="/data/bar",age="365"} 4228829760
stor_age_bytes_modified{dir="/data/baz",age="90"} 976617687
stor_age_bytes_modified{dir="/data/baz",age="365"} 1162918704
```


Features
--------

The following list contains the **crate** features to be enabled via e.g.
`cargo build --feature <FEATURES>`. The list contains whether or not the
feature is enabled by default and what the rationale of this feature is.

-   **table** (default)

    Adds an output format, i.e. `--format table`, that pretty-prints the report
    as a table. This is intended for interactive command-line usage. It is the
    default output format if this feature is enabled.

    If you only need the output formats that are useful to be included as
    metrics in monitoring systems (e.g. `--format prometheus`), you can disable
    this feature to minimize dependencies.

-   **spectrum-scale**

    Adds an optional file system iteration mode specific to IBM Spectrum Scale
    file systems. This iteration mode uses the `mmapplypolicy` command instead
    of universal directory traversal ([std::fs::read_dir][read_dir]).

    This can be considerably faster, especially for large directories, because
    it uses file system internals and can use extensive parallelism. See the
    respective command-line options in `--help` output for more information.

    **Note:** The policies used with `mmapplypolicy` write temporary lists of
    files to plain text files. These can get quite large, scaling with the
    amount of files. Depending on where these lists are kept, you will either
    need large amounts of memory (**tmpfs**) or disk space, approximately 150
    MiB per million files.


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
[Prometheus]: https://prometheus.io/ "Prometheus home page"
[read_dir]: https://doc.rust-lang.org/std/fs/fn.read_dir.html "rust std::fs::read_dir function"
