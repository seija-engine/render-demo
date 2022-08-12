use std::sync::Arc;
mod camera;
mod tools;
mod demo;
use camera::add_pbr_camera;
use demo::{DemoGame, GameState};
use seija_app::ecs::system::{ResMut, Res};
use seija_app::{App};
use seija_asset::{AssetModule, AssetServer, Assets};
use seija_core::math::{Vec3, Quat};
use seija_core::window::AppWindow;
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_core::bevy_ecs::{prelude::{Commands}};
use seija_core::math::{EulerRot};
use seija_gltf::asset::GltfAsset;
use seija_gltf::{GLTFModule};
use seija_input::InputModule;
use seija_pbr::lights::PBRLight;
use seija_render::material::MaterialStorage;
use seija_render::{RenderModule, RenderConfig, GraphSetting};
use seija_transform::{TransformModule, Transform};
use seija_winit::WinitModule;
use seija_pbr::{create_pbr_plugin};


#[derive(Default)]
pub struct GameData {
   
}


fn main() {
    env_logger::Builder::new().filter_level(log::LevelFilter::Info).try_init().unwrap();
    let mut app = App::new();
    init_modules(&mut app);
    app.add_system2(CoreStage::Startup ,StartupStage::Startup, on_start);
    app.add_system(CoreStage::PostUpdate,camera::update_camera_trans_system);
    app.init_resource::<GameData>();
    app.run();
}

fn init_modules(app:&mut App) {
    app.add_module(CoreModule);
    app.add_module(TransformModule);
    app.add_module(InputModule);
    app.add_module(AssetModule);
    app.add_module(GLTFModule);
    app.add_module(WinitModule::default());
    let render_config = RenderConfig {
        config_path:".shader".into(),
        script_path:"script/render.clj".into(),
        setting:Arc::new(GraphSetting::default() ),
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec!["script".into() ],
    };
    app.add_module(RenderModule(Arc::new(render_config)));
    app.init_resource::<DemoGame>();
    app.add_system(CoreStage::Update, on_update);
    app.start();
}

pub fn on_start(mut commands:Commands,win:Res<AppWindow>,materials:Res<MaterialStorage>) {
    materials.load_material_def(std::fs::read_to_string("materials/pbrColor.mat.clj").unwrap().as_str());
    materials.load_material_def(std::fs::read_to_string("materials/pbrStandard.mat.clj").unwrap().as_str());

    add_pbr_camera(&mut commands,Vec3::new(0f32, 0f32, 2f32),Quat::IDENTITY,&win); 
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    }  
}

fn on_update(mut game:ResMut<DemoGame>,mut commands:Commands,asset:Res<AssetServer>,gltfs:Res<Assets<GltfAsset>>,mats:Res<MaterialStorage>) {
    match game.state {
        GameState::None => { game.state = GameState::LoadAssets }
        GameState::LoadAssets => {
            game.load_assets(&asset); 
            game.state = GameState::WaitAssets 
        },
        GameState::WaitAssets => {
            if game.loadings.iter().all(|v| v.1.is_finish()) {
                let loadings:Vec<_> = game.loadings.drain(..).collect();
                for (name,track) in loadings {
                    game.asset_cache.insert(name, track.handle().clone());
                }
                game.on_asset_ready(&gltfs,&mut commands,&mats);
                game.state = GameState::OnStart;
            }
        },
        _ => {},
    }
}

 