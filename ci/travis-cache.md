# Clearing the cache of Travis CI builds

Travis CI offers caching for Rust / Cargo builds with the intent to reduce the
build times by caching dependencies (like Cargo packages) and tools (like the
current Rust compiler binaries). Some cache directories are imported before the
build and - if changes occurred to them - exported after the build.

While this is a good idea at first glance, it can quickly fill with bloat, e. g.
from older compiler versions or from older versions of Cargo crates and related
build artifacts used by previous builds. Therefore it may be necessary to clear
the build cache once in a while, because the overhead of having a big cache can
lead to the inverse effect and actually __increase__ build times, compared to
having no cache at all.

To clear the build cache, modify the section `script` in `.travis.yml` from

```
script:
  - cargo build
  - cargo test
```

to

```
script:
  - rm -rf /home/travis/build/striezel/corona/target
  - rm -rf /home/travis/.cache
  - rm -rf /home/travis/.cargo
  - rm -rf /home/travis/.rustup
  - curl -sSf https://build.travis-ci.org/files/rustup-init.sh | sh -s -- --default-toolchain=$TRAVIS_RUST_VERSION --profile=minimal -y
  - cargo build
  - cargo test
```

Commit, push to GitHub, and let the build run once for all jobs.
After that the changes can be reverted. You may want to use a force push in that
case to avoid that the commit shows up in the commit log.
