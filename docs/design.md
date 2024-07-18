# Design philosophy

The high-level goal of this template is to feel like the official template that is currently missing from Bevy.
The exists an [official CI template](https://github.com/bevyengine/bevy_github_ci_template), but, in our opinion,
that one is currently more of an extension to the [Bevy examples](https://bevyengine.org/examples/) than an actual template.
We say this because it is extremely bare-bones and as such does not provide things that in practice are necessary for game development.

## Principles

So, how would an official template that is built for real-world game development look like?
The Bevy Jam working group has agreed on the following guiding design principles:

- Show how to do things in pure Bevy. This means using no 3rd-party dependencies.
- Have some basic game code written out already.
- Have everything outside of code already set up.
  - Nice IDE support.
  - `cargo-generate` support.
  - Workflows that provide CI and CD with an auto-publish to itch.io.
  - Builds configured for perfomance by default.
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
see [bevy-design-patterns](https://github.com/tbillington/bevy_best_practices) and [bevy_best_practices](https://github.com/tbillington/bevy_best_practices).

## Code structure

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

## UI

### Pattern

Spawn your UI elements by extending the [`Widgets` trait](../src/ui/widgets.rs):

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

This pattern is inspired by [sickle_ui](https://github.com/UmbraLuminosa/sickle_ui).
`Widgets` is implemented for `Commands` and similar, so you can easily spawn UI elements in your systems.
By encapsulating a widget inside a function, you save on a lot of boilerplate code and can easily change the appearance of all widgets of a certain type.
By returning `EntityCommands`, you can easily chain multiple widgets together and insert children into a parent widget.

## Assets

### Pattern

Preload your assets by encapsulating them in a struct:

```rust
#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SomeAsset {
    Player,
    Enemy,
    Powerup,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SomeAssets(HashMap<SomeAsset, Handle<Something>>);

impl SomeAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        // load them from disk via the asset server
    }

    pub fn all_loaded(&self, assets: &Assets<Something>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}
```

Then add them to the [loading screen](../src/screen/loading.rs) functions `enter_loading` and `check_all_loaded`.

### Reasoning

This pattern is inspired by [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader).
In general, by preloading your assets, you can avoid hitches during gameplay.
By using an enum to represent your assets, you don't leak details like file paths into your game code and can easily change the asset that is loaded at a single point.

## Spawning

### Pattern

Spawn a game object by using an observer:

```rust
// monster.rs
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_monster);
}

#[derive(Event, Debug)]
pub struct SpawnMonster;

fn on_spawn_monster(
    _trigger: Trigger<SpawnMonster>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Monster"),
        // other components
    ));
}
```

And then, somewhere else in your code, trigger the observer:

```rust
fn spawn_monster(mut commands: Commands) {
    commands.trigger(SpawnMonster);
}
```

### Reasoning

By encapsulating the spawning of a game object in a function,
you save on boilerplate code and can easily change the behavior of spawning.
An observer is an elegant way to then trigger this function from anywhere in your code.
A limitation of this approach is that calling code cannot extend the spawn call with additional components or children.
If you know about a better pattern, please let us know!

## Dev tools

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

## Screens

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
    Playing,
    GameOver,
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
    app.add_systems(OnEnter(Screen::GameOver), enter_game_over);
    app.add_systems(OnExit(Screen::GameOver), exit_game_over);
}

fn enter_game_over(mut commands: Commands) {
    commands.
        .ui_root()
        .insert(StateScoped(Screen::GameOver))
        .with_children(|parent| {
            // Add UI elements
        });
}

fn exit_game_over(mut next_screen: ResMut<NextState<Screen>>) {
    // Go back to the title screen
    next_screen.set(Screen::Title);
}
```

### Reasoning

"Screen" is not meant as a physical screen, but as "what kind of screen is the game showing right now", e.g. the title screen, the loading screen, the playing screen, the game over screen, etc.
These screens usually correspond to different logical states of your game that have different systems running.

By using dedicated `State`s for each screen, you can easily manage systems and entities that are only relevant for a certain screen.
This allows you to flexibly transition between screens whenever your game logic requires it.
