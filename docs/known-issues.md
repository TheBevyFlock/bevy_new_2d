# Known Issues

## My audio is stuttering on web

There are a number of issues with audio on web, so this is not an exhaustive list. The short version is that you can try the following:

- If you're using materials, you should force your render pipelines to [load at the start of the game](https://github.com/rparrett/bevy_pipelines_ready/blob/main/src/lib.rs).
- Optimize your game as much as you can to keep its FPS high.
- Apply the suggestions from the blog post [Workaround for the Choppy Music in Bevy Web Builds](https://necrashter.github.io/bevy-choppy-music-workaround).
- Advise your users to try a Chromium-based browser if there are still issues.

## My game window flashes white for a split second when I start the game on Windows

The game window is created before the GPU is ready to render everything.
This means that it'll start with a white screen for a few frames.
The workaround is to [spawn the Window hidden](https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L29-L32)
and only [make it visible a few frames later](https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L56-L64).

## My character or camera movement is choppy

Choppy character movement is often caused by movement updates being tied to the frame rate.
See the [`physics_in_fixed_timestep`](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs) example
for how to fix this.

Choppy camera movement is almost always caused by the camera being tied too tightly to a moving target position.
You can use [`smooth_nudge`](https://github.com/bevyengine/bevy/blob/main/examples/movement/smooth_follow.rs#L127-L142) to make your camera
smoothly approach its target position instead.
