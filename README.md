<h1 align="center">mktex</h1>

## Description

A simple command line utility to make working with personal common templates/classes easier.

## Quick Start

```bash
$ ./build.sh
$ ./mktex -h
```

## History

For [a while](https://github.com/jakewilliami/tex-macros/commit/1a1885bd67dc529fa5babd993fd8dfa6933fee83), I've had a [`mktex`](https://github.com/jakewilliami/tex-macros/blob/bc47621e1009a7c8e65c2051ade1ba6100c18a1a/tools/mktex) script.  However, it is written in Bash, so it was very big and not fast, reliable, nor very portable.

I figured, now that I am back at uni and using LaTeX, it would be a good time to start to port some of the functionality (and perhaps some more complex functionality) of `mktex` to a more reliable-in-production language.

The main thing we want to extend is the class option, which generates an empty LaTeX document with a given class.  However, we no longer want this class to be local to each project, so we should put it into the local texmf folder.  Along with this, version controlling becomes more important, so we may want to implement features that allow us to evaluate the class in a single file at a given time.  Finally, it would also be nice to have this pull from the git remote, rather than assuming local files exist.

This port to Rust started still [within the `tex-macros` repo](https://github.com/jakewilliami/tex-macros/tree/bc47621e1009a7c8e65c2051ade1ba6100c18a1a/tools/mktex.rs), however I quickly realised the project outgrew being but a child in this repository.  Hence, I migrated it to its own repository here.
