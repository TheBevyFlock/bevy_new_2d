# Design philosophy

The high-level goal of this template is to feel like the official template that is currently missing from Bevy.
The exists an [official CI template](https://github.com/bevyengine/bevy_github_ci_template), but that is currently more
of an extension to the [Bevy examples](https://bevyengine.org/examples/) than an actual template.

## Principles

So, how would an official template that is built for real-world game development look like?
The Bevy Jam working group has agreed on the following guiding design principles:

- Show how to do things in pure Bevy. This means using no 3rd-party dependencies.
- Have some basic game code written out already.
- Have everything outside of code already set up.
  - Nice IDE support
  - `cargo-generate` support
  - Workflows that provide CI and CD with an auto-publish to itch.io.
  - Builds configured for perfomance by default
- Answer questions that will quickly come up when creating an actual game
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
use bevy::prelude::*;

mod player;
mod enemy;
mod powerup;

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
