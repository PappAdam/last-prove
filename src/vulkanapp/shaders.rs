pub mod tile_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/tile_vertex_shader.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

pub mod hud_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/hud_vertex_shader.vert"
    }
}

pub mod general_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/general_fragment_shader.frag"
    }
}