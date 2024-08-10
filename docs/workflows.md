# Workflows

This template uses [GitHub workflows](https://docs.github.com/en/actions/using-workflows) for [CI / CD](https://www.redhat.com/en/topics/devops/what-is-ci-cd).
They are defined in [`.github/workflows/`](../.github/workflows).

## CI (testing)

The [CI workflow](.github/workflows/ci.yaml) will trigger on every commit or PR to `main`, and do the following:

- Run tests.
- Run Clippy lints.
- Check formatting.
- Check documentation.

> [!Tip]
> <details>
>   <summary>You may want to set up a <a href="https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets">GitHub ruleset</a> to require that all commits to <code>main</code> pass CI.</summary>
>
>   <img src="img/workflow-ruleset.png" alt="A screenshot showing a GitHub ruleset with status checks enabled" width="100%">
> </details>

## CD (releasing)

The [CD workflow](../.github/workflows/release.yaml) will trigger on every pushed tag in the format `v1.2.3`, and do the following:

- Create a release build for Windows, macOS, Linux, and web.
- (Optional) Upload to [GitHub releases](https://docs.github.com/en/repositories/releasing-projects-on-github).
- (Optional) Upload to [itch.io](https://itch.io).

<details>
  <summary>This workflow can also be triggered manually.</summary>

  In your GitHub repository, navigate to `Actions > Release > Run workflow`:

  ![A screenshot showing a manually triggered workflow on GitHub Actions](./img/workflow-dispatch-release.png)

  Enter a version number in the format `v1.2.3`, then hit the green `Run workflow` button.
</details>

> [!Important]
> Using this workflow requires some setup. We will go through this now.

### Configure environment variables

<details>
  <summary><a href="../.github/workflows/release.yaml"><code>.github/workflows/release.yaml</code></a> contains a list of environment variables.</summary>

  ```yaml
  env:
    # The base filename of the binary produced by `cargo build`.
    BINARY: bevy_template
    # The name to use for the packaged application produced by this workflow.
    PACKAGE_NAME: bevy-template
    # The itch.io page to upload to, in the format: `user-name/project-name`.
    # Comment this out to disable.
    ITCH_TARGET: the-bevy-flock/bevy-template
    # The organization or author that owns the rights to the game.
    OWNER: the-bevy-flock
    # The path to the assets directory.
    ASSETS_DIR: assets
    # Whether packages produced by this workflow should be uploaded to the Github release.
    UPLOAD_PACKAGES_TO_GITHUB_RELEASE: true
    # Before enabling LFS, please take a look at GitHub's documentation for costs and quota limits:
    # https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-storage-and-bandwidth-usage
    USE_GIT_LFS: false
  ```
</details>

The values are set automatically by `cargo generate`, or you can edit them yourself and push a commit.

### Set up itch.io upload

#### Add butler credentials

<details>
  <summary>In your GitHub repository, navigate to <code>Settings > Secrets and variables > Actions</code>.</summary>

  ![A screenshot showing where to add secrets in the GitHub Actions settings](./img/workflow-secrets.png)
</details>

Hit `New repository secret` and enter the following values, then hit `Add secret`:

- **Name:** `BUTLER_CREDENTIALS`
- **Secret:** Your [itch.io API key](https://itch.io/user/settings/api-keys) (create a new one if necessary)

#### Create itch.io project

Create a new itch.io project with the same user and project name as in the `ITCH_TARGET` variable in [release.yaml](../.github/workflows/release.yaml).
Hit `Save & view page` at the bottom of the page.

[Run the release workflow](#cd-releasing) for the first time. Once it's done, go back to itch.io and hit `Edit game` in the top left.

Set `Kind of project` to `HTML`, then find the newly uploaded `web` build and tick the box that says "This file will be played in the browser".

![A screenshot showing a web build selected in the itch.io uploads](img/workflow-itch-release.png)
