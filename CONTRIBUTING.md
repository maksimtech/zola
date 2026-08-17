# Contributing
**As the documentation site is automatically built on commits to master, all development happens on
the `next` branch, unless it is fixing the current documentation.**

However, if you notice an error or typo in the documentation, feel free to directly submit a PR without opening an issue.

## Feature requests
If you want a feature added or modified, please open a thread on the [forum](https://zola.discourse.group/) to discuss it before doing a PR.

Requested features will not be all added: an ever-increasing features set makes for a hard to use and explain softwares.
Having something simple and easy to use for 90% of the use cases is more interesting than covering 100% use cases after sacrificing simplicity.

## Issues tagging

As the development happens on the `next` branch, issues are kept open until a release containing the fix is out.
During that time, issues already resolved will have a `done` tag.

If you want to work on an issue, please mention it in a comment to avoid potential duplication of work. If you have
any questions on how to approach it do not hesitate to ping me (@keats).
Easy issues are tagged with `help wanted` and/or `good first issue`

## Benchmarks

The benchmarks live in `components/*/benches` and use [divan](https://github.com/nvzqz/divan)
through the CodSpeed compatibility layer. They run on every pull request and the results are
reported by [CodSpeed](https://app.codspeed.io/maksimtech/zola).

They can be run locally with:

```bash
cargo codspeed build --workspace
cargo codspeed run --workspace
```

`cargo-codspeed` can be installed with `cargo install cargo-codspeed --locked`.

`components/site/benches/gen.py` generates bigger sites if you need to profile Zola on
something larger than the `test_site` fixture used by the benchmarks.

## Adding syntax highlighting languages, themes or aliases

Open an issue on the [Giallo repository](https://github.com/getzola/giallo).
