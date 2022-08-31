
/* 
use seija_render::{resource::{Texture, TextureDescInfo}, material::{MaterialStorage, Material}};

pub fn _load_texture(textures:&mut Assets<Texture>,path:&str) -> Handle<Texture> {
    
    let texture = Texture::from_image_bytes(&std::fs::read(path).unwrap(),TextureDescInfo::default()).unwrap();
    textures.add(texture)
}

pub fn conv_pbr_material(gltf_material:&GltfMaterial,materials:&MaterialStorage) -> Option<Handle<Material>> {
    let material_name = if gltf_material.is_opaque() { "pbrStandard" } else { "pbrStandard[T]" };
    materials.create_material_with(material_name, |props| {
        if let Some(base_color) = gltf_material.base_color_texture.clone() {
            props.texture_props.set("baseColor", base_color);
        }
        if let Some(normal_texture) = gltf_material.normal_texture.clone() {
            
            props.texture_props.set("normal", normal_texture);
        }
        if let Some(metallic_roughness_texture) = gltf_material.metallic_roughness_texture.clone() {
          
            props.texture_props.set("metallicRoughness", metallic_roughness_texture);
        }
        if let Some(emissive_texture) = gltf_material.emissive_texture.clone() {
            props.texture_props.set("emissive", emissive_texture);
        }
        props.props.set_float3("emissiveFactor", gltf_material.emissive_factor, 0);
        props.props.set_float4("baseColorFactor", gltf_material.base_color_factor, 0);
        props.props.set_f32("metallicFactor",      gltf_material.metallic_factor, 0);
        props.props.set_f32("roughnessFactor",     gltf_material.roughness_factor, 0);
    })
}*/