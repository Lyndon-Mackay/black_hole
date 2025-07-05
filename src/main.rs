use bevy::{
    core_pipeline::bloom::Bloom,
    gltf::GltfMaterialName,
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F7)))
        .add_plugins(MaterialPlugin::<BlackHoleRimMaterial>::default())
        .insert_state(ShaderState::Waiting)
        .add_systems(Startup, (setup, load_black_hole).chain())
        .add_systems(Update, shader_on_rim.run_if(in_state(ShaderState::Waiting)))
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
enum ShaderState {
    Waiting,
    Done,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlackHoleRimMaterial {}

impl Material for BlackHoleRimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/black_hole_rim.wgsl".into()
    }
}

fn setup(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(0.0, 13.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);

    let bloom = Bloom {
        intensity: 0.2,
        ..default()
    };

    commands.insert_resource(AmbientLight {
        brightness: 15.0,
        ..Default::default()
    });

    commands.insert_resource(ClearColor(Color::BLACK));

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..Default::default()
        },
        camera_transform,
        bloom,
    ));
}

fn load_black_hole(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let black_hole = asset_server.load(GltfAssetLabel::Scene(0).from_asset("black_hole.glb"));

    let transform = Transform::from_xyz(0.0, 0.0, -30.0).with_scale(Vec3::splat(0.30));
    commands.spawn((SceneRoot(black_hole), transform)).observe(
        |_trigger: Trigger<Pointer<Pressed>>| {
            info!("you clicked");
        },
    );
    let mut camera_transform = camera.single_mut().unwrap();

    *camera_transform =
        Transform::from_xyz(0.0, 70.0, 10.0).looking_at(transform.translation, Vec3::Z);
}

fn shader_on_rim(
    gltf_query: Query<(Entity, &GltfMaterialName)>,
    mut next_state: ResMut<NextState<ShaderState>>,
    mut materials: ResMut<Assets<BlackHoleRimMaterial>>,
    mut commands: Commands,
) {
    if gltf_query.iter().next().is_some() {
        next_state.set(ShaderState::Done);
    }
    for (entity, q) in gltf_query {
        let mut entity_commands = commands.get_entity(entity).unwrap();
        entity_commands.insert(MeshMaterial3d(materials.add(BlackHoleRimMaterial {})));
    }
}
