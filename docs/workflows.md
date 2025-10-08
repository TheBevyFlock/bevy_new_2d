# Workflows

This template uses [GitHub workflows](https://docs.github.com/en/actions/using-workflows) for [CI / CD](https://www.redhat.com/en/topics/devops/what-is-ci-cd), defined in [`.github/workflows/`](../.github/workflows).

> [!WARNING]
> GitHub puts a limit on free CI usage for [private repositories](https://docs.github.com/en/actions/using-github-hosted-runners/using-github-hosted-runners/about-github-hosted-runners#standard-github-hosted-runners-for--private-repositories), so tune your workflows accordingly!

## CI (testing)

The [CI workflow](.github/workflows/ci.yaml) will trigger on every commit or PR to `main`, and it will:

- Check formatting.
- Check documentation.
- Run Clippy lints.
- Run [Bevy lints](https://thebevyflock.github.io/bevy_cli/bevy_lint/).
- Run tests.
- Check that the web build compiles.

> [!TIP]
> <details>
>   <summary>Consider setting up a <a href="https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets">GitHub ruleset</a> to require that all commits to <code>main</code> pass CI.</summary>
>
>   <img src="img/workflow-ruleset.png" alt="A screenshot showing a GitHub ruleset with status checks enabled" width="100%">
> </details>

## CD (releasing)

The [CD workflow](../.github/workflows/release.yaml) will trigger on manual workflow dispatch, and it will:

- (Optional) Create a release build for Windows, macOS, Linux, and web.
- (Optional) Upload to [GitHub releases](https://docs.github.com/en/repositories/releasing-projects-on-github).
- (Optional) Deploy to [itch.io](https://itch.io).
- (Optional) Deploy to [GitHub Pages](https://docs.github.com/en/pages).

<details>
  <summary><ins>Triggering a release</ins></summary>

  In your GitHub repository, navigate to `Actions > Release > Run workflow`:

  ![A screenshot showing a manually triggered workflow on GitHub Actions](./img/workflow-dispatch-release.png)

  Enter a version number in the format `v1.2.3`, then hit the green `Run workflow` button.
</details>

> [!IMPORTANT]
> Using this workflow requires some setup. We'll go through this now.

### Configure environment variables

The release workflow can be configured by tweaking the environment variables in [`.github/workflows/release.yaml`](../.github/workflows/release.yaml).

<details>
  <summary><ins>List of environment variables and how they're used</ins></summary>

  ```yaml
  # The base filename of the binary produced by `cargo build`.
  cargo_build_binary_name: bevy_new_2d

  # The path to the assets directory.
  assets_path: assets

  # The itch.io project to deploy to in the format `user-name/project-name`.
  # There will be no deployment to itch.io if this is commented out.
  itch_page: the-bevy-flock/bevy-new-2d

  # The ID of the app produced by this workflow.
  # Applies to macOS releases.
  # Must contain only A-Z, a-z, 0-9, hyphen, and period: <https://developer.apple.com/documentation/bundleresources/information_property_list/cfbundleidentifier>.
  app_id: the-bevy-flock.bevy-new-2d

  # The base filename of the binary in the package produced by this workflow.
  # Applies to Windows, macOS, and Linux releases.
  # Defaults to `cargo_build_binary_name` if commented out.
  app_binary_name: bevy_new_2d

  # The name of the `.zip` or `.dmg` file produced by this workflow.
  # Defaults to `app_binary_name` if commented out.
  app_package_name: bevy-new-2d

  # The display name of the app produced by this workflow.
  # Applies to macOS releases.
  # Defaults to `app_package_name` if commented out.
  app_display_name: Bevy New 2D

  # The short display name of the app produced by this workflow.
  # Applies to macOS releases.
  # Must be 15 or fewer characters: <https://developer.apple.com/documentation/bundleresources/information_property_list/cfbundlename>.
  # Defaults to `app_display_name` if commented out.
  app_short_name: Bevy New 2D

  # Before enabling LFS, please take a look at GitHub's documentation for costs and quota limits:
  # <https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-storage-and-bandwidth-usage>
  git_lfs: false

  # Enabling this only helps with consecutive releases to the same version (and takes up cache storage space).
  # See: <https://github.com/orgs/community/discussions/27059>.
  use_github_cache: false
  ```
</details>

The initial values are set automatically by `bevy new`, but you can edit them yourself and push a commit.

### Set up itch.io deployment

#### Add butler credentials

<details>
  <summary>In your GitHub repository, navigate to <code>Settings > Secrets and variables > Actions</code>.</summary>

  ![A screenshot showing where to add secrets in the GitHub Actions settings](./img/workflow-secrets.png)
</details>

Hit `New repository secret` and enter the following values, then hit `Add secret`:

- **Name:** `BUTLER_CREDENTIALS`
- **Secret:** Your [itch.io API key](https://itch.io/user/settings/api-keys) (create a new one if necessary)

#### Create itch.io project

Create a new itch.io project with the same user and project name as in the `itch_page` variable in [`.github/workflows/release.yaml`](../.github/workflows/release.yaml).
Hit `Save & view page` at the bottom of the page.

Trigger the [release workflow](#cd-releasing) for the first time. Once it's done, go back to itch.io and hit `Edit game` in the top left.

Set `Kind of project` to `HTML`, then find the newly uploaded `web` build and tick the box that says `This file will be played in the browser`.

![A screenshot showing a web build selected in the itch.io uploads](./img/workflow-itch-release.png)

### Set up GitHub Pages deployment

#### Enable deployment via GitHub Actions

<details>
  <summary>In your GitHub repository, navigate to <code>Settings > Pages > Build and Deployment</code>.</summary>

  ![A screenshot showing where to enable Pages deployment via GitHub Actions](./img/enable_github_pages.png)
</details>

Hit the drop-down under `Source` and choose `Github Actions` as the source of your Pages deployment.

#### Add your branch to the `github-pages` environment

If you want to deploy to GitHub Pages from a branch other than `main`, you'll have to add that branch to the `github-pages` environment.

<details>
  <summary>In your GitHub repository, navigate to <code>Settings > Environments > github-pages > Deployment branches</code>.</summary>

  ![A screenshot showing where to add a branch to the github-pages environment](./img/add_branch_to_github_pages_environment.png)
</details>

Hit `Add deployment branch rule`, type in the name of the branch you want to deploy to Pages, and hit `Add rule`.
