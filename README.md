# Bevy GitHub CI Template

This repo shows how to set up CI on a GitHub project for Bevy.

It creates two workflows:

* [CI](#ci)
* [Release](#release)

## Running the project

Native:

```sh
cargo run
```

Web:

```sh
trunk serve
```

## CI

Definition: [.github/workflows/ci.yaml](./.github/workflows/ci.yaml)

This workflow runs on every commit to `main` branch, and on every PR targeting the `main` branch.

It will use rust stable on linux, with cache between different executions, those commands:

* `cargo test`
* `cargo clippy -- -D warnings`
* `cargo fmt --all -- --check`

If you are using anything OS specific or rust nightly, you should update the file [ci.yaml](./.github/workflows/ci.yaml) to use those.

## Release

Definition: [.github/workflows/release.yaml](./.github/workflows/release.yaml)

This workflow runs on every tag.

It will build:

* For Linux and Windows, a .zip archive containing the executable and the `assets`.
* For macOS, a dmg image with a .app containing the `assets`.
* For wasm, a .zip archive with the wasm binary, the js bindings, an html file loading it, and the `assets`.

If you don't want to target some of those platforms, you can remove the corresponding job from the file [release.yaml](./.github/workflows/release.yaml).

If you don't want to attach the builds to the GitHub release, set `env.add_binaries_to_github_release` to `false`.

If you are using Git LFS, set `env.use_git_lfs` to `true` so your assets are properly checked out.

> [!Warning]
> GitHub's LFS storage has a quota. Please take a look at GitHub's documentation [here](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-storage-and-bandwidth-usage) to understand the quota and costs before enabling this option.

### Git Tag from GitHub UI

You can follow [Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)

### Git Tag from the CLI

Execute the following commands:

```sh
git tag -a "my-game-1.0" -m "First official release"
git push --tags
```

### Result

A new release will be available in GitHub, with the archives per platform available as downloadable assets.

The `git` commands above produced this release: [my-game-1.0](
https://github.com/bevyengine/bevy_github_ci_template/releases/tag/my-game-1.0).

## Using the workflows in your own project

If you would like to use the GitHub workflows included here for your own project, there are a few things you might have to adapt:

1. The release workflow relies on the files under `/web` for web builds
2. Make sure that the env variable `BINARY` in [release.yaml](.github/workflows/release.yaml#L20) matches the name of your binary
3. Adapt the used toolchain if you are using nightly
4. In your GitHub repo's settings, under `Actions -> General` make sure "Read and Write permissions" is selected under "Workflow permissions" near the bottom. This fixes the error `Error: Resource not accessible by integration`.

### Publish on itch.io

The release flow can be configured to push the releases to itch.io:

1. Create an API key in <https://itch.io/user/settings/api-keys>
2. Go to the repository's Settings tab in GitHub, click on Secrets->Actions in the sidebar,and add a repository secret named `BUTLER_CREDENTIALS` set to the API key.
3. Set `ITCH_TARGET` in [`release.yaml`](.github/workflows/release.yaml#L25) to your itch.io username and the name of the game on itch.io, separated by a slash (`/`)

Once that is done, any tag pushed to GitHub will trigger an itch.io release and use the tag as the [user version](https://itch.io/docs/butler/pushing.html#specifying-your-own-version-number).

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE-2.0](LICENSE-Apache-2.0) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
* CC0-1.0 License
   ([LICENSE-CC0-1.0](LICENSE-CC0-1.0) or <https://creativecommons.org/publicdomain/zero/1.0/legalcode>)

at your option.

The Ducky sprite is CC-0 licensed by [Caz Creates Games](https://caz-creates-games.itch.io/ducky-2).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.
