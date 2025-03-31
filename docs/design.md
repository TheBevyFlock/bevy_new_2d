# Design philosophy

The high-level goal of this template is to feel like the official template that is currently missing from Bevy.
There exists an [official CI template](https://github.com/bevyengine/bevy_github_ci_template), but, in our opinion,
that one is currently more of an extension to the [Bevy examples](https://bevyengine.org/examples/) than an actual template.
We say this because it is extremely bare-bones and as such does not provide things that in practice are necessary for game development.

## Principles

So, how would an official template that is built for real-world game development look like?
The Bevy Jam working group has agreed on the following guiding design principles:

- Show how to do things in pure Bevy. This means using no 3rd-party dependencies.
- Have some basic game code written out already.
- Have everything outside of code already set up.
  - Nice IDE support.
  - `bevy new` template support.
  - Workflows that provide CI and CD with an auto-publish to itch.io.
  - Builds configured for performance by default.
- Answer questions that will quickly come up when creating an actual game.
  - How do I structure my code?
  - How do I preload assets?
  - What are best practices for creating UI?
  - etc.

The last point means that in order to make this template useful for real-life projects,
we have to make some decisions that are necessarily opinionated.

These opinions are based on the experience of the Bevy Jam working group and
what we have found to be useful in our own projects.
If you disagree with any of these, it should be easy to change them.

Bevy is still young, and many design patterns are still being discovered and refined.
Most do not even have an agreed name yet. For some prior work in this area that inspired us,
see [the Unofficial Bevy Cheatbook](https://bevy-cheatbook.github.io/) and [bevy_best_practices](https://github.com/tbillington/bevy_best_practices).

## Pattern Table of Contents

- [Plugin Organization](#plugin-organization)
- [Widgets](#widgets)
- [Asset Preloading](#asset-preloading)
- [Spawn Commands](#spawn-commands)
- [Dev Tools](#dev-tools)
- [Screen States](#screen-states)

When talking about these, use their name followed by "pattern",
e.g. "the widgets pattern", or "the plugin organization pattern".

## Plugin Organization

### Pattern

Structure your code into plugins like so:

```rust
// game.rs
mod player;
mod enemy;
mod powerup;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, enemy::plugin, powerup::plugin));
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

A good rule of thumb is to have one plugin per file,
but feel free to leave out a plugin if your file does not need to do anything with the `App`.

## Widgets

### Pattern

Spawn your UI elements by extending the [`Widgets` trait](../src/theme/widgets.rs):

```rust
pub trait Widgets {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
    fn text_input(&mut self, text: impl Into<String>) -> EntityCommands;
    fn image(&mut self, texture: Handle<Texture>) -> EntityCommands;
    fn progress_bar(&mut self, progress: f32) -> EntityCommands;
}
```

### Reasoning

`Widgets` is implemented for `Commands` and similar, so you can easily spawn UI elements in your systems.
By encapsulating a widget inside a function, you save on a lot of boilerplate code and can easily change the appearance of all widgets of a certain type.
By returning `EntityCommands`, you can easily chain multiple widgets together and insert children into a parent widget.

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

Then start preloading in `assets::plugin`:

```rust
pub(super) fn plugin(app: &mut App) {
    app.register_type::<ActorAssets>();
    app.load_resource::<ActorAssets>();
}
```

Note that `app.load_resource` comes from an extension trait defined in [src/asset_tracking.rs](../src/asset_tracking.rs)

### Reasoning

This pattern is inspired by [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader).
By preloading your assets, you can avoid hitches during gameplay.
Assets will begin loading immediately at startup, and the loading screen will wait until they're done.

## Spawn Commands

### Pattern

Spawn a game object by using a custom command. Inside the command,
run the spawning code with `world.run_system_cached` or  `world.run_system_cached_with`:

```rust
// monster.rs

#[derive(Debug)]
pub struct SpawnMonster {
    pub health: u32,
    pub transform: Transform,
}

impl Command for SpawnMonster {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_monster, self);
    }
}

fn spawn_monster(
    spawn_monster: In<SpawnMonster>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Monster"),
        Health::new(spawn_monster.health),
        spawn_monster.transform,
        Visibility::default(),
        // other components
    ));
}
```

And then to use a spawn command, add it to `Commands`:

```rust
// dangerous_forest.rs

fn spawn_forest_goblin(mut commands: Commands) {
    commands.queue(SpawnMonster {
        health: 100,
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
    });
}
```

### Reasoning

By encapsulating the spawning of a game object in a custom command,
you save on boilerplate code and can easily change the behavior of spawning.
We use `world.run_system_once_with` to run the spawning code with the same syntax as a regular system.
That way you can easily add system parameters to access things like assets and resources while spawning the entity.

A limitation of this approach is that calling code cannot extend the spawn call with additional components or children,
as custom commands don't return `Entity` or `EntityCommands`. This kind of usage will be possible in future Bevy versions.

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
By adding your dev tools here, you automatically guarantee that they are not included in release builds.

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
    Credits,
    Gameplay,
    Victory,
    Leaderboard,
    MultiplayerLobby,
    SecretMinigame,
}
```

Constrain entities that should only be present in a certain screen to that screen by adding a
[`StateScoped`](https://docs.rs/bevy/latest/bevy/prelude/struct.StateScoped.html) component to them.
Transition between screens by setting the [`NextState<Screen>`](https://docs.rs/bevy/latest/bevy/prelude/enum.NextState.html) resource.

For each screen, create a plugin that handles the setup and teardown of the screen with `OnEnter` and `OnExit`:

```rust
// game_over.rs
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Victory), show_victory_screen);
    app.add_systems(OnExit(Screen::Victory), reset_highscore);
}

fn show_victory_screen(mut commands: Commands) {
    commands.
        .ui_root()
        .insert((Name::new("Victory screen"), StateScoped(Screen::Victory)))
        .with_children(|parent| {
            // Spawn UI elements.
        });
}

fn reset_highscore(mut highscore: ResMut<Highscore>) {
    *highscore = default();
}
```

### Reasoning

"Screen" is not meant as a physical screen, but as "what kind of screen is the game showing right now", e.g. the title screen, the loading screen, the credits screen, the victory screen, etc.
These screens usually correspond to different logical states of your game that have different systems running.

By using dedicated `State`s for each screen, you can easily manage systems and entities that are only relevant for a certain screen.
This allows you to flexibly transition between screens whenever your game logic requires it.
