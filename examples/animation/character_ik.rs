//! Applies IK on skinned glTF character.

use std::f32::consts::*;
use std::time::Duration;

use bevy::{prelude::*, render::mesh::skinning::SkinnedMesh};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(ClearColor(Color::rgb(0.105, 0.10, 0.11)))
        .add_startup_system(setup)
        .add_system(setup_scene_once_loaded)
        .add_system(joint_animation)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/CharacterIK.glb#Animation0")
    ]));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 1.0, 5.0)
            .looking_at(Vec3::new(-2.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
        material: materials.add(Color::rgb(0.105, 0.10, 0.11).into()),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            illuminance: 70000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Character
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/animated/CharacterIK.glb#Scene0"),
        ..default()
    });
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn joint_animation(
    time: Res<Time>,
    parent_query: Query<&Parent, With<SkinnedMesh>>,
    children_query: Query<&Children>,
    mut transform_query: Query<&mut Transform>,
) {
    // Iter skinned mesh entity
    for skinned_mesh_parent in &parent_query {
        // Mesh node is the parent of the skinned mesh entity.
        let mesh_node_entity = skinned_mesh_parent.get();
        // Get `Children` in the mesh node.
        let mesh_node_children = children_query.get(mesh_node_entity).unwrap();

        // // First joint is the second child of the mesh node.
        // let first_joint_entity = mesh_node_children[1];
        // // Get `Children` in the first joint.
        // let first_joint_children = children_query.get(first_joint_entity).unwrap();

        // // Second joint is the first child of the first joint.
        // let second_joint_entity = first_joint_children[0];
        // // Get `Transform` in the second joint.
        // let mut second_joint_transform = transform_query.get_mut(second_joint_entity).unwrap();

        // second_joint_transform.rotation =
        //     Quat::from_rotation_z(FRAC_PI_2 * time.elapsed_seconds().sin());
    }
}
