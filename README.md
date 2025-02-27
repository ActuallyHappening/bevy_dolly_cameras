<h1>bevy_dolly</h1>
<div align="center">
<table>
  <tr>
    <th>Static</th>
    <th>Pinned</th>
  </tr>
  <tr>
    <td><a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_dolly_1.svg" alt="bevy dolly static"></a></td>
    <td><a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_dolly_dev_0.svg" alt="bevy dolly pinned"></a></td>
  </tr>
</table>
  <div align="center">
<a href="https://crates.io/crates/bevy_dolly"><img src="https://img.shields.io/crates/v/bevy_dolly" alt="link to crates.io"></a>
<a href="https://docs.rs/bevy_dolly"><img src="https://docs.rs/bevy_dolly/badge.svg" alt="link to docs.rs"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly/blob/main/LICENSE-MIT"><img src="https://img.shields.io/crates/l/bevy_dolly" alt="link to license"></a>
<a href="https://crates.io/crates/bevy_dolly"><img src="https://img.shields.io/crates/d/bevy_dolly" alt="downloads/link to crates.io"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly"><img src="https://img.shields.io/github/stars/BlackPhlox/bevy_dolly" alt="stars/github repo"></a>
<a href="https://github.com/BlackPhlox/bevy_dolly/actions/workflows/main.yml"><img src="https://github.com/BlackPhlox/bevy_dolly/actions/workflows/main.yml/badge.svg" alt="github actions"></a>
<a href="https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking"><img src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue" alt="tracking bevy release branch"></a>
</div>
</div>
</br>

`bevy_dolly_camera` is a prototype plugin using [h3r2tic](https://github.com/h3r2tic)'s powerful crate: [dolly](https://github.com/h3r2tic/dolly), implemented for bevy.

⚠ **Feedback** - _Bevy_dolly's API is still being revised, so feedback on ergonomics and DX is highly appreciated_ ⚠

It is important to note that dolly is a way to control the movement of the camera and thus, not the camera component itself. </br>

Dolly requires two steps to function:

1. Creating a `Rig` we are able to define drivers on which the dolly can enact, these drivers can both be constraints and functionality.
2. A marker component that is registered on both the Camera and the Rig.

## What are drivers?

Explain what drivers are

To read more about the different drivers.

```rs
#[derive(Component)]
struct MainCamera;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    //..
    .add_system(Dolly::<MainCamera>::update_active)
    //..
    .run();
}
```

```rs
// In your setup system
fn setup(
  mut commands: Commands,
) {
  commands.spawn((
    MainCamera, // The rig tag
    Rig::builder()
      .with(Position::new(Vec3::ZERO))
      .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
      .with(Smooth::new_position(0.3))
      .with(Smooth::new_rotation(0.3))
      .with(Arm::new(Vec3::Z * 4.0))
      .build(),
    Camera3dBundle::default(),
  ));
}
```

Link to [examples readme](/examples/README.md)

## Helpers

This plugin currently also provides some helper plugins, which.
They are enabled by default but are not needed and can be removed when setting up bevy_dolly's dependency:

```toml
[dependencies]
bevy_dolly = { version = "0.0.1", default-features = false }
```

Note this will also remove the bevy_dolly's extended drivers and add `features = ["drivers"],` to include the drivers back.

## Examples

To see how works in Bevy in practice, please look at this repository's [examples](/examples/).

### How to run

`cargo run --release --example orbit`

## Support

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_dolly|
|---|---|
|0.12| |0.0.2|
|0.11| |0.0.1|

## Alternatives

There is a bunch of other bevy camera controllers that are worth checking out, especially if you are just starting out learning bevy:

- [bevy_fps_controller](https://github.com/qhdwight/bevy_fps_controller) - A Fps controller with crouching, sprinting, flymode and more
- [smooth-bevy-cameras](https://github.com/bonsairobo/smooth-bevy-cameras) - 3 Smooth Camera controllers: Fps, Orbit or Unreal
- [bevy_spectator](https://github.com/JonahPlusPlus/bevy_spectator) - A spectator camera controller
- [bevy_flycam](https://github.com/sburris0/bevy_flycam) - A simple fly camera
- [bevy_fly_camera](https://github.com/mcpar-land/bevy_fly_camera)  - A advanced fly camera
- [bevy_pancam](https://github.com/johanhelsing/bevy_pancam) - 2D Click and Drag - Style camera movement
- [bevy_config_cam](https://github.com/BlackPhlox/bevy_config_cam) - Plugin that enables to use collection of different camera controllers at runtime, uses bevy_dolly as the backend

## Licensing

The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

## Contributing

Yes this project is still a WIP, so PRs are very welcome.
