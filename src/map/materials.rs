use bevy::{
    pbr::{deferred::DEFAULT_PBR_DEFERRED_LIGHTING_PASS_ID, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::Face,
};

pub const GRASS_MATERIAL: StandardMaterial = StandardMaterial {
    base_color: Color::rgb(148. / 255., 186. / 255., 101. / 255.),
    reflectance: 0.,
    ..DEFAULT_MATERIAL
};

pub const WATER_MATERIAL: StandardMaterial = StandardMaterial {
    base_color: Color::rgb(39. / 255., 144. / 255., 176. / 255.),
    reflectance: 0.2,
    ..DEFAULT_MATERIAL
};

pub const DEFAULT_MATERIAL: StandardMaterial = StandardMaterial {
    // White because it gets multiplied with texture values if someone uses
    // a texture.
    base_color: Color::rgb(1.0, 1.0, 1.0),
    base_color_texture: None,
    emissive: Color::BLACK,
    emissive_texture: None,
    // Matches Blender's default roughness.
    perceptual_roughness: 0.5,
    // Metallic should generally be set to 0.0 or 1.0.
    metallic: 0.0,
    metallic_roughness_texture: None,
    // Minimum real-world reflectance is 2%, most materials between 2-5%
    // Expressed in a linear scale and equivalent to 4% reflectance see
    // <https://google.github.io/filament/Material%20Properties.pdf>
    reflectance: 0.5,
    diffuse_transmission: 0.0,
    #[cfg(feature = "pbr_transmission_textures")]
    diffuse_transmission_texture: None,
    specular_transmission: 0.0,
    #[cfg(feature = "pbr_transmission_textures")]
    specular_transmission_texture: None,
    thickness: 0.0,
    #[cfg(feature = "pbr_transmission_textures")]
    thickness_texture: None,
    ior: 1.5,
    attenuation_color: Color::WHITE,
    attenuation_distance: f32::INFINITY,
    occlusion_texture: None,
    normal_map_texture: None,
    flip_normal_map_y: false,
    double_sided: false,
    cull_mode: Some(Face::Back),
    unlit: false,
    fog_enabled: true,
    alpha_mode: AlphaMode::Opaque,
    depth_bias: 0.0,
    depth_map: None,
    parallax_depth_scale: 0.1,
    max_parallax_layer_count: 16.0,
    parallax_mapping_method: ParallaxMappingMethod::Occlusion,
    opaque_render_method: OpaqueRendererMethod::Auto,
    deferred_lighting_pass_id: DEFAULT_PBR_DEFERRED_LIGHTING_PASS_ID,
};
