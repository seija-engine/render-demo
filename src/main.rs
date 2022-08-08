use std::sync::Arc;
mod camera;
mod tools;
use camera::add_pbr_camera;
use seija_app::ecs::system::{ResMut, Res};
use seija_app::{App};
use seija_asset::{AssetModule, Assets};
use seija_core::math::{Vec3, Quat};
use seija_core::window::AppWindow;
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_core::bevy_ecs::{prelude::{Commands}};
use seija_core::math::{EulerRot};
use seija_input::InputModule;
use seija_pbr::lights::PBRLight;
use seija_render::material::MaterialStorage;
use seija_render::resource::{Mesh, Texture, TextureDescInfo};
use seija_render::resource::shape::Sphere;
use seija_render::wgpu::{TextureFormat, AddressMode, FilterMode};
use seija_render::{RenderModule, RenderConfig, GraphSetting};
use seija_transform::{TransformModule, Transform};
use seija_winit::WinitModule;
use seija_pbr::{create_pbr_plugin};
use tools::load_texture;


fn main() {
    env_logger::init();
    let mut app = App::new();
    init_modules(&mut app);
    app.add_system2(CoreStage::Startup ,StartupStage::Startup, on_start);
    app.add_system(CoreStage::PostUpdate,camera::update_camera_trans_system);
    
    app.run();
}

fn init_modules(app:&mut App) {
    app.add_module(CoreModule);
    app.add_module(TransformModule);
    app.add_module(InputModule);
    app.add_module(AssetModule);
    app.add_module(WinitModule::default());
    let render_config = RenderConfig {
        config_path:".shader".into(),
        script_path:"script/render.clj".into(),
        setting:Arc::new(GraphSetting::default() ),
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec!["script".into() ],
    };
    app.add_module(RenderModule(Arc::new(render_config)));
    app.start();
}

pub fn on_start(mut commands:Commands,
                mut textures:ResMut<Assets<Texture>>,
                win:Res<AppWindow>,
                mut meshs:ResMut<Assets<Mesh>>,
                materials:Res<MaterialStorage>) {
    materials.load_material_def(std::fs::read_to_string("materials/pbrColor.mat.clj").unwrap().as_str());
    materials.load_material_def(std::fs::read_to_string("materials/pbrStandard.mat.clj").unwrap().as_str());

    let h_base_color = load_texture(&mut textures, "res/texture/red_brick/castle_brick_02_red_diff_1k.jpg");
    //let h_normal = load_texture(&mut textures, "res/texture/red_brick/castle_brick_02_red_nor_gl_1k.jpg");
    let h_rough = load_texture(&mut textures, "res/texture/red_brick/castle_brick_02_red_rough_1k.jpg");

    let mut desc = TextureDescInfo::default();
    desc.desc.format = TextureFormat::R32Float;
    desc.sampler_desc.min_filter = FilterMode::Linear;
    desc.sampler_desc.mag_filter = FilterMode::Linear;
    desc.sampler_desc.mipmap_filter = FilterMode::Linear;
    let texture = Texture::from_image_bytes(&std::fs::read("res/texture/red_brick/castle_brick_02_red_nor_gl_1k.jpg").unwrap(),
    desc).unwrap();
    let h_normal = textures.add(texture);
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
    //sphere
    {
        let mesh =  Sphere::new(1f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("pbrStandard", |mat| {
           mat.texture_props.set("baseColor", h_base_color.clone());
           mat.texture_props.set("metallicRoughness", h_rough.clone());
           mat.texture_props.set("normal", h_normal.clone());
        }).unwrap();

        let mut t = Transform::default();
        t.local.position = Vec3::new(-1f32, 0f32, -1f32);
       
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };

    //sphere2
    {
        let mesh =  Sphere::new(1f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("pbrColor", |_mat| {
           
        }).unwrap();

        let mut t = Transform::default();
        t.local.position = Vec3::new(1.5f32, 0f32, -1f32);
       
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };
}