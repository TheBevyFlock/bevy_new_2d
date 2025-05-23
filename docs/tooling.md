# Recommended 3rd-party tools

None of these are required, but they can save you a lot of time and effort.

Check out the [Bevy Assets](https://bevyengine.org/assets/) page for more great options.

## Libraries

A few libraries that the authors of this template have vetted and think you might find useful:

| Name                                                                                   | Category       | Description                           |
| -------------------------------------------------------------------------------------- | -------------- | ------------------------------------- |
| [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager) | Input          | Input -> Action mapping               |
| [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui)        | Debugging      | Live entity inspector                 |
| [`bevy_mod_debugdump`](https://github.com/jakobhellermann/bevy_mod_debugdump)          | Debugging      | Schedule inspector                    |
| [`inline_tweak`](https://github.com/Uriopass/inline_tweak/)                            | Debugging      | Live tweaking of literal values       |
| [`avian`](https://github.com/Jondolf/avian)                                            | Physics        | Physics engine                        |
| [`bevy_rapier`](https://github.com/dimforge/bevy_rapier)                               | Physics        | Physics engine (not ECS-driven)       |
| [`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets)                 | Asset loading  | Asset loaders for common file formats |
| [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader)                   | Asset loading  | Asset management tools                |
| [`iyes_progress`](https://github.com/IyesGames/iyes_progress)                          | Asset loading  | Progress tracking                     |
| [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio)                       | Audio          | Advanced audio features               |
| [`bevy_cobweb_ui`](https://github.com/UkoeHB/bevy_cobweb_ui)                           | UI             | UI framework                          |
| [`bevy_egui`](https://github.com/mvlabat/bevy_egui)                                    | UI / Debugging | UI framework (great for debug UI)     |
| [`tiny_bail`](https://github.com/benfrankel/tiny_bail)                                 | Error handling | Error handling convenience macros     |

In particular:

- `leafwing-input-manager` is very likely to be upstreamed into Bevy in the near future.
- `bevy-inspector-egui`, `bevy_mod_debugdump`, and `inline_tweak` help fill the gap until Bevy has its own editor.
- `avian` or `bevy_rapier` helps fill the gap until Bevy has its own physics engine. `avian` is easier to use, while `bevy_rapier` is more performant.
- `bevy_cobweb_ui` is well-aligned with `bevy_ui` and helps fill the gap until Bevy has a full collection of UI widgets and features.

## CLI tools

A few command-line tools that you may find useful:

| Name                                                              | Description                                                   |
|-------------------------------------------------------------------|---------------------------------------------------------------|
| [`bevy_lint`](https://thebevyflock.github.io/bevy_cli/bevy_lint/) | Checks for good practices and footguns specific to Bevy       |
| [`oxipng`](https://github.com/shssoichiro/oxipng)                 | Lossless PNG compression (may help reduce file sizes for web) |
| [`gifski`](https://github.com/ImageOptim/gifski)                  | High-quality GIF encoder (good for animated itch.io content)  |

> [!NOTE]
>
> `bevy_lint` already runs in CI by default (see [workflows](./workflows.md)).

## Other templates

There are many other Bevy templates out there.
You can find some of them in the [templates category](https://bevyengine.org/assets/#templates) on Bevy Assets.

> [!TIP]
> Even if you don't end up using them directly, they can be very helpful as learning material!

# IDE integration

## VS Code extensions

If you're using [VS Code](https://code.visualstudio.com/), the following extensions are highly recommended:

| Name                                                                                                      | Description                       |
|-----------------------------------------------------------------------------------------------------------|-----------------------------------|
| [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)              | Rust support                      |
| [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)          | TOML support                      |
| [vscode-ron](https://marketplace.visualstudio.com/items?itemName=a5huynh.vscode-ron)                      | RON support                       |
| [Dependi](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi)                          | `crates.io` dependency resolution |
| [EditorConfig for VS Code](https://marketplace.visualstudio.com/items?itemName=EditorConfig.EditorConfig) | `.editorconfig` support           |

> [!NOTE]
> <details>
>   <summary><ins>About the included <code>rust-analyzer</code> settings</ins></summary>
>
>   This template sets [`rust-analyzer.cargo.targetDir`](https://rust-analyzer.github.io/generated_config.html#rust-analyzer.cargo.targetDir)
>   to `true` in [`.vscode/settings.json`](../.vscode/settings.json).
>
>   This makes `rust-analyzer` use a different `target` directory than `cargo`,
>   which means that you can run commands like `cargo run` even while `rust-analyzer` is still indexing.
>   As a trade-off, this will use more disk space.
>
>   If that is an issue for you, you can set it to `false` or remove the setting entirely.
> </details>

## RustRover Live Templates

If you're using [RustRover](https://www.jetbrains.com/rust/), you may want to set up [Live Templates](https://www.jetbrains.com/help/rust/using-live-templates.html) to provide autocomplete for common boilerplate code.

Unfortunately, it is not really possible at this time to share Live Templates on a per-project basis, as they are global, however you can still set them up yourself.

Here's a quick guide for porting this template's [VS Code snippets](../.vscode/bevy.code-snippets) to Live Templates:

- Replace any instances of `$0` in the template with `$END$`
- Replace any instances of `$1` in the template with `$NAME$` or something similar.
- For the `plugin` template, you might want to set the applicability to rust modules.
- For the other templates, you might want to set the applicability to rust modules, statements, and expressions.

To make it easier to enable or disable these live templates for different projects, you can put them in a template group called `Bevy`.

## Debugging with RustRover  

This template comes with a Cargo Run Configuration that disables dynamic linking (and dev tools) so that the debugger will work out of the box. If you'd like to enable those features in the debugger, it'll require some setup:

1. Run `rustc --print target-libdir` and copy the output. You can specify a channel here with e.g. `rustc +nightly --print target-libdir`
2. Edit the Cargo Run Configuration named "Run Native Debug" (it should be the one without a terminal icon).
3. Add the following Environment Variable:
  a. Linux or Mac: `LD_LIBRARY_PATH` = `./target/debug/deps:<LIBDIR_PATH>` where `<LIBDIR_PATH>` is the output from step 1.
  b. Windows: `PATH` = `.\target\debug\deps:<LIBDIR_PATH>`, where `<LIBDIR_PATH>` is the output from step 1.
3. Remove `--no-default-features` from the command in the Run Configuration.
4. Click Apply and then Debug, and if everything is correct it should launch the game.

If you want to use multiple different channels for the same project, you will need to add in a `LIBDIR_PATH` for every channel you intend on using.

If you're still having issues, please ensure that the channels in the path and the Run Configuration match, and that there are no extra spaces (especially at the beginning or end).

> [!NOTE]
> <details>
>   <summary><ins>Attaching the debugger to a running game</ins></summary>
>
>   If you started your game with a Shell Script Run Configuration, you can attach the debugger to it while it's running by using `Run > Attach to Process` and selecting the process with the same name as your game (not the one named `bevy`).
>
>   This does not work for web builds.
> </details>
