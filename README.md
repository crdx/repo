# repo

**repo** is a tool that recursively finds and manages git repositories.

## CLI

```
Usage:
    repo [options] ls
    repo [options] run [ -- ] <command>...

Find git repository paths starting from the current directory,
or from stdin if supplied.

Options:
    -d, --dirty          Include only dirty repos
    -u, --unpushed       Include only unpushed repos
    -x, --absolute       Use absolute paths
    -v, --verbose        Show a header for each repo when executing a command
    -i, --interactive    Pause between command executions
    --ignore-errors      Ignore errors when executing commands
    -h, --help           Show help
```

## Examples

Find repositories you've left uncommitted changes in.

```bash
repo -d ls
```

Find repositories you've left unpushed commits in.

```bash
repo -u ls
```

Push all repositories you've left unpushed commits in.

```bash
repo -vu run git push
```

Perform garbage collection and fsck all repositories.

```bash
repo -v run git gc
repo -v run git fsck
```

Check the status of all repositories.

```bash
repo -v run git status -sb
```

## Ignore patterns

If a config file containing [gitignore patterns](https://git-scm.com/docs/gitignore) exists at `$HOME/.config/repo/ignore` then absolute repository paths will be matched against this file using the high-performance [ignore](https://docs.rs/ignore) crate.

As absolute repository paths are used to match against the ignore rules the ignore pattern file should be written as though its root is `/`, the root of the filesystem. This means that if the pattern contains a `/` then it must be either absolute or prefixed with a `**/` to match.

### Example

```
.cargo/              # Match ".cargo" as any component of path.
**/.cargo/registry/  # Match just the "registry" subdirectory.
```

## Build

Ensure `rust` is installed.

```bash
cargo build --release
```

The binary can be found in `target/release`.

## Why not use libgit2 instead of spawning git processes?

When benchmarked the [git2](https://docs.rs/git2) crate performed worse than spawning git processes to do the same job. Yes, this disappointed me too.

## Bugs or contributions

Open an [issue](http://github.com/crdx/repo/issues) or send a [pull request](http://github.com/crdx/repo/pulls).

## Licence

[MIT](LICENCE.md).
