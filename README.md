# Git Stratum

<p style="text-align:center;">
    <b>Stratum</b>: A single layer of something - <a 
        href="https://dictionary.cambridge.org/dictionary/english/stratum">
        Cambridge Dictionary
    </a> 
</p>

A library that lets you anlayse your repository one **strata** at a time, leveraging a higher level API than [git2-rs](https://github.com/rust-lang/git2-rs) analysing your repository is simple!

```bash
cargo add git-stratum
```

## Usage

*This is currently unstable and any and all versions of the API up untiil version 1 should be considered unstable. The below usage is the current vision for the project.*

```rust
use stratum::Repository;

let repo = Repository::from_str("path/to/repo");
for commit in repo.iter_commits() {
    ...
}
```