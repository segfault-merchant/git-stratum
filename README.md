# Git Stratum

**Stratum**: A single layer of something - [Cambridge Dictionary]("https://dictionary.cambridge.org/dictionary/english/stratum").

A library that lets you anlayse your repository one **strata** at a time, leveraging a higher level API than [git2-rs](https://github.com/rust-lang/git2-rs) analysing your repository is simple!

## Quick Start

First add `stratum` to your dependencies.

```bash
cargo add git-stratum
```

Then inside of your main function:

```rust
use stratum::open_repository;

let repo = open_repository("path/to/repo").unwrap();
for commit in repo.traverse_commits().unwrap() {
    let commit = commit.unwrap();
    ...
}
```

*Note that the API is liable to change up until version 1.0.0.*

For more detail on the API, see the [docs]().

## Testing

### Unit Testing

In `/src/lib.rs` a test specific module called `common` is defined, in here a `git2` reposiotry is lazilly made for testing purposes. This method for unit testing was chosen for several reasons:

- Mocking `git2` objects would be very challenging as they do not expose any traits.
- Lazilly constructing this repository ensures it is only made once per unit testing module.
- This allows for the direct testing of private functions with minimal overhead/difficulty.

#### Example

```rust
#[cfg(test)]
mod test {
    use crate::common::{init_repo};

    #[test]
    fn test_something() {
        let repo = init_repo();
        ...
    }
}
```

### Integration Testing

The integration tests, will effectively function the same as the unit tests, however the data that the integration tests have access to (in `test-repos.zip`) is far more elaborate and rich, the primary goal of this design is to catch the edge cases that the unit tests miss.

To write new tests:
- Manually unzip the `test-repos.zip` choose a repository to use in testing.
    - If one is not appropriate, create a new one, **be sure to upload a new zip file containing your new repository**.
- To use your chosen repository, use a code block similar to the following:

```rust
mod common;
use common::test_data_dir;

fn small_repo() -> PathBuf {
    test_data_dir().join("small_repo")
}
```

- The `test-repo` should only be unzipped once per test module, so if your testing fits within an existing test module, it will save some time to put the test in an existing module.

**credit:** The `/test-repos.zip` file was originally created by the maintainer of [PyDriller](https://github.com/ishepard/pydriller/tree/master), which is the core inspiration for this project. At the time of writing Pydriller is under the Apache2.0 license.