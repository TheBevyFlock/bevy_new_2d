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
- [Interaction Callbacks](#interaction-callbacks)
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

## Asset Preloading

### Pattern

Define your assets with a resource that maps asset paths to `Handle`s.
If you're defining the assets in code, add their paths as constants.
Otherwise, load them dynamically from e.g. a file.

```rust
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ImageHandles(HashMap<String, Handle<Image>>);

impl ImageHandles {
    pub const KEY_PLAYER: &'static str = "images/player.png";
    pub const KEY_ENEMY: &'static str = "images/enemy.png";
    pub const KEY_POWERUP: &'static str = "images/powerup.png";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let files = [
            ImageHandles::KEY_PLAYER,
            ImageHandles::KEY_ENEMY,
            ImageHandles::KEY_POWERUP,
        ];
        let map = files
            .into_iter()
            .map(|file| (file.to_string(), asset_server.load(file)))
            .collect();

        Self(map)
    }
}
```

Then start preloading in the `assets::plugin`:

```rust
pub(super) fn plugin(app: &mut App) {
    app.register_type::<ImageHandles>();
    app.init_resource::<ImageHandles>();
}
```

And finally add a loading check to the `screens::loading::plugin`:

```rust
fn all_assets_loaded(
    image_handles: Res<ImageHandles>,
) -> bool {
    image_handles.all_loaded(&asset_server)
}
```

### Reasoning

This pattern is inspired by [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader).
By preloading your assets, you can avoid hitches during gameplay.
We start loading as soon as the app starts and wait for all assets to be loaded in the loading screen.

By using strings as keys, you can dynamically load assets based on input data such as a level file.
If you prefer a purely static approach, you can also use an `enum YourAssetHandleKey` and `impl AsRef<str> for YourAssetHandleKey`.
You can also mix the dynamic and static approach according to your needs.

## Spawn Commands

### Pattern

Spawn a game object by using a custom command. Inside the command,
run the spawning code with `world.run_system_once` or  `world.run_system_once_with`:

```rust
// monster.rs

#[derive(Debug)]
pub struct SpawnMonster {
    pub health: u32,
    pub transform: Transform,
}

impl Command for SpawnMonster {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_monster);
    }
}

fn spawn_monster(
    spawn_monster: In<SpawnMonster>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Monster"),
        Health::new(spawn_monster.health),
        SpatialBundle::from_transform(spawn_monster.transform),
        // other components
    ));
}
```

And then to use a spawn command, add it to `Commands`:

```rust
// dangerous_forest.rs

fn spawn_monster(mut commands: Commands) {
    commands.add(SpawnMonster {
        health: 100,
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
    });
}
```

### Reasoning

By encapsulating the spawning of a game object in a custom command,
you save on boilerplate code and can easily change the behavior of spawning.
We use `world.run_system_once_with` to run the spawning code with the same syntax as a regular system.
That wasy, you can easily add system parameters to access things like assets and resources while spawning the entity.

A limitation of this approach is that calling code cannot extend the spawn call with additional components or children,
as custom commands don't return `Entity` or `EntityCommands`. This kind of usage will be possible in future Bevy versions.

## Interaction Callbacks

### Pattern

When spawning an entity that can be interacted with, such as a button that can be pressed,
register a [one-shot system](https://bevyengine.org/news/bevy-0-12/#one-shot-systems) to handle the interaction.
Add the resulting `SystemId` through a newtype component to your entity:

```rust
#[derive(Component, Debug, Reflect, Deref, DerefMut)]
#[reflect(Component, from_reflect = false)]
pub struct OnPress(#[reflect(ignore)] pub SystemId);

fn spawn_button(mut commands: Commands) {
    let pay_money = commands.register_one_shot_system(pay_money);
    commands.button("Pay up!", pay_money);
}

// See the `Widgets` pattern for context
impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>, on_press: SystemId) -> EntityCommands {
        self.spawn((
            Name::new("Button"),
            ButtonBundle {
                // ...
                default()
            },
            OnPress(on_press),
        ));
    }
}
```

The reflect attributes are currently needed due to
[`SystemId` not implementing `Reflect`](https://github.com/bevyengine/bevy/issues/14496)

Also despawn the one-shot system when the interactive entity is despawned to prevent memory leaks:

```rust
pub(super) fn plugin(app: &mut App) {
    app.observe(despawn_one_shot_system);
}

/// Remove the one-shot system entity when the [`OnPress`] component is removed.
/// This is necessary as otherwise, the system would still exist after the button
/// is removed, causing a memory leak.
fn despawn_one_shot_system(
    trigger: Trigger<OnRemove, OnPress>,
    mut commands: Commands,
    on_press_query: Query<&OnPress>,
) {
    let on_press = on_press_query.get(trigger.entity()).unwrap();
    let one_shot_system_entity = on_press.entity();
    commands.entity(one_shot_system_entity).despawn_recursive();
}
```

### Reasoning

This pattern is inspired by [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking).
By adding the system handling the interaction to the entity as a component,
the code running on interactions can be scoped to the exact context of the interaction.

For example, the code for what happens when you press a *specific* button is directly attached to that exact button.

This also keeps the interaction logic close to the entity that is interacted with,
allowing for better code organization. If you want multiple buttons to do the same thing, consider triggering an event in their callbacks.

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
    app.add_systems(OnEnter(Screen::GameOver), show_game_over_screen);
    app.add_systems(OnExit(Screen::GameOver), reset_highscore);
}

fn show_game_over_screen(mut commands: Commands) {
    commands.
        .ui_root()
        .insert(StateScoped(Screen::GameOver))
        .with_children(|parent| {
            // Add UI elements
        });
}

fn reset_highscore(mut highscore: ResMut<Highscore>) {
    *highscore = default();
}
```

### Reasoning

"Screen" is not meant as a physical screen, but as "what kind of screen is the game showing right now", e.g. the title screen, the loading screen, the playing screen, the game over screen, etc.
These screens usually correspond to different logical states of your game that have different systems running.

By using dedicated `State`s for each screen, you can easily manage systems and entities that are only relevant for a certain screen.
This allows you to flexibly transition between screens whenever your game logic requires it.
