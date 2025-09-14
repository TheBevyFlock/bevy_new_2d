# Design philosophy

The high-level goal of this template is to feel like the official template that Bevy is currently missing.
There's an [official CI template](https://github.com/bevyengine/bevy_github_ci_template), but its goal is to
demonstrate how to accomplish a particular task with Bevy (setting up CI), which makes it better aligned to
serve as an [example](https://bevyengine.org/examples/) than as a template.

## Principles

So, what would an official template built for real-world game development look like?
The Bevy Jam working group has agreed on the following guiding design principles:

- Have no 3rd-party dependencies besides Bevy to make it easy for users to pull in whatever they prefer.
- Include some basic game code to give users something to work with.
- Answer common questions like how you should structure your codebase, load assets, or write reusable UI code.
- Configure development tools to work well out-of-the-box, for example: 
  - Support creating a new project with [`bevy new`](https://github.com/TheBevyFlock/bevy_cli/) or [`cargo generate`](https://github.com/cargo-generate/cargo-generate).
  - Integrate with the most common IDEs (e.g. VS Code and RustRover).
  - Provide robust GitHub CI and CD workflows (including an option to automatically publish to [itch.io](https://itch.io)).
  - Configure dev builds for fast compile times and release builds for performance.

As a result, this template has to make some decisions that are necessarily opinionated.

These opinions are based on the experience of the Bevy Jam working group from working on our own projects.
If you disagree with any of the decisions made, please feel free to change them to your liking.

Bevy is still young, and many design patterns are still being discovered and refined.
For some prior work in this area that inspired us, see [the Unofficial Bevy Cheatbook](https://bevy-cheatbook.github.io/)
and [bevy_best_practices](https://github.com/tbillington/bevy_best_practices).

## Pattern Table of Contents

- [Plugin Organization](#plugin-organization)
- [Screen States](#screen-states)
- [Bundle Functions](#bundle-functions)
- [Asset Preloading](#asset-preloading)
- [Dev Tools](#dev-tools)

When referring to one of these patterns, you can use their name followed by "pattern",
like "the plugin organization pattern", or "the screen states pattern".

## Plugin Organization

### Pattern

Structure your code into plugins like so:

```rust
// game.rs
mod enemy;
mod player;
mod powerup;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((enemy::plugin, player::plugin, powerup::plugin));
}
```

```rust
// player.rs / enemy.rs / powerup.rs
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (your, systems, here));
}
```

### Reasoning

Bevy is great at organizing code into plugins. The most lightweight way to do this is by using simple functions as plugins.
By splitting your code like this, you can easily keep all your systems and resources locally grouped. Everything that belongs to the `player` is only in `player.rs`, and so on.

A good rule of thumb is to always have one plugin per file, but feel free to omit plugins that would be empty.

## Screen States

### Pattern

Use the [`Screen`](../src/screen/mod.rs) enum to represent your game's screens as states:

```rust
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Gameplay,
    Victory,
    Leaderboard,
    MultiplayerLobby,
}
```

For each screen, create a plugin that handles the setup and teardown in the
[`OnEnter`](https://docs.rs/bevy/latest/bevy/prelude/struct.OnEnter.html) and
[`OnExit`](https://docs.rs/bevy/latest/bevy/prelude/struct.OnExit.html) schedules.
You should mark the screen's entities to despawn on exit by giving them the
[`DespawnOnExit`](https://docs.rs/bevy/latest/bevy/prelude/struct.DespawnOnExit.html) component.

```rust
// victory.rs
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Victory), spawn_victory_screen);
    app.add_systems(OnExit(Screen::Victory), reset_highscore);
}

fn spawn_victory_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Victory Screen"),
        DespawnOnExit(Screen::Victory),
        children![
            // UI elements.
        ],
    ));
}

fn reset_highscore(mut highscore: ResMut<Highscore>) {
    *highscore = default();
}
```

Transition between screens by setting the [`NextState<Screen>`](https://docs.rs/bevy/latest/bevy/prelude/enum.NextState.html) resource:

```rust
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, enter_title_screen.run_if(input_just_pressed(KeyCode::Escape)));
}

fn enter_title_screen(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Title);
}
```

### Reasoning

"Screen" is not meant as the physical screen, but as "what kind of screen is the game showing right now", e.g. the title screen, the loading screen, the victory screen, etc.
These screens usually correspond to different logical states of your game that have different systems running.

By using a dedicated `State` type for your screens, you can easily manage systems and entities that are only relevant for a specific screen and flexibly transition between
them whenever your game logic requires it.

## Bundle Functions

### Pattern

Write functions that return `impl Bundle` to define simple entity templates.

```rust
pub fn monster(health: u32, transform: Transform) -> impl Bundle {
    (
        Name::new("Monster"),
        Health::new(health),
        transform,
        // other components
    )
}
```

You can extend a bundle function with additional components that are not present in the original bundle:

```rust
pub fn boss_monster(transform: Transform) -> impl Bundle {
    (
        monster(1000, transform),
        Better,
        Faster,
        Stronger,
    )
}
```

You can compose bundle functions to define simple entity hierarchies:

```rust
pub fn dangerous_forest() -> impl Bundle {
    (
        Name::new("Dangerous Forest"),
        Transform::default(),
        children![
            monster(100, Transform::from_xyz(10.0, 0.0, 0.0)),
            monster(200, Transform::from_xyz(20.0, 0.0, 0.0)),
            boss_monster(Transform::from_xyz(30.0, 0.0, 0.0)),
        ],
    )
}
```

And finally, you can spawn entities using your bundle functions:

```rust
fn spawn_dangerous_forest(mut commands: Commands) {
    commands.spawn(dangerous_forest());
}
```

### Reasoning

By encapsulating the definition of an entity in a bundle function, you can save on boilerplate
and make it easier to change its behavior, even if you spawn it in many different places in your code.

This approach comes with a few limitations, however:

- **No dependency injection:** If you want to use data from the world when creating a bundle, you have to pass it as an argument (e.g. `&AssetServer`)
  all the way down the entity hierarchy to the particular bundle function that needs it.
- **No replacing components:** If you want to extend a bundle function by _replacing_ one of its components (e.g. to modify its `Node::width`),
  you have to add an argument to the function to explicitly allow for it, or remove the component from the original bundle, or use `Commands` to access `insert` (like
  `commands.spawn(foo()).insert(Replacement)`), which is not compatible with `children![]`-style composition.

These limitations are expected to be [lifted in future Bevy versions](https://github.com/bevyengine/bevy/discussions/9538).

## Asset Preloading

### Pattern

Define an asset collection resource to load and store your asset `Handle`s:

```rust
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct ActorAssets {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    player: Handle<Image>,
    #[dependency]
    enemies: Vec<Handle<Image>>,
}

impl FromWorld for ActorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("images/player.png"),
            enemies: vec![
                assets.load("images/enemy1.png"),
                assets.load("images/enemy2.png"),
                assets.load("images/enemy3.png"),
            ],
        }
    }
}
```

Then start preloading in `actor::plugin`:

```rust
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<ActorAssets>();
}
```

Note that `app.load_resource` comes from an extension trait defined in [src/asset_tracking.rs](../src/asset_tracking.rs).

### Reasoning

This pattern is inspired by [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader).
By preloading your assets, you can avoid hitches during gameplay.
Assets will begin loading immediately at startup, and the loading screen will wait until they're done.

## Dev Tools

### Pattern

Add all systems that are only relevant while developing the game to the [`dev_tools` plugin](../src/dev_tools.rs):

```rust
// dev_tools.rs
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (draw_debug_lines, show_debug_console, show_fps_counter));
}
```

### Reasoning

The `dev_tools` plugin is only included in dev builds.
By adding your dev tools here, you guarantee that they won't be included in release builds.
