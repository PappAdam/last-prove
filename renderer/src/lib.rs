mod base;
mod data;
mod draw_setup;
pub mod engine;
pub mod resources;
mod setup;
pub mod utils;

use ash::vk;
use resources::buffer::Buffer;
use utils::{buffer_data::BufferObject, MAX_WORLD_OBJECTS};
use winit::window::Window;

use crate::{base::RenderBase, data::RenderData, utils::MAX_FRAME_DRAWS};

pub struct Renderer {
    pub data: RenderData,
    pub base: RenderBase,

    meshes_buffers: Vec<[Buffer; 2]>,

    pub current_frame_index: usize,
    pub current_pipeline_index: usize,
    pub rebuild_swapchain: bool,

    pub image_index: usize,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, String> {
        let mut base = RenderBase::new(window)?;
        let data = RenderData::new(&mut base)?;

        Ok(Self {
            base,
            data,
            current_frame_index: 0,
            current_pipeline_index: 0,
            rebuild_swapchain: true,
            image_index: 0,
            meshes_buffers: Vec::with_capacity(MAX_WORLD_OBJECTS),
        })
    }

    pub fn load_mesh(&mut self, mesh: [Buffer; 2]) {
        self.meshes_buffers.push(mesh)
    }

    #[inline]
    pub fn stage_mesh(&self, mesh: (vk::Buffer, vk::Buffer, u32, usize)) {
        let current_command_buffer = self.data.command_buffers[self.current_frame_index];
        unsafe {
            self.base.device.cmd_bind_descriptor_sets(
                current_command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.data.pipeline_layout,
                0,
                &[self.data.descriptor_sets[self.current_frame_index]],
                &[mesh.3 as u32 * self.data.dynamic_uniform_buffer.alignment as u32],
            );

            self.base
                .device
                .cmd_bind_vertex_buffers(current_command_buffer, 0, &[mesh.0], &[0]);

            self.base.device.cmd_bind_index_buffer(
                current_command_buffer,
                mesh.1,
                0,
                vk::IndexType::UINT32,
            );

            self.base
                .device
                .cmd_draw_indexed(current_command_buffer, mesh.2, 1, 0, 0, 0);
        }
    }

    #[inline]
    pub fn prepare_renderer(&mut self) -> Result<(), String> {
        self.image_index = match self.get_img_index()? {
            Some(index) => index as usize,
            None => {
                self.rebuild_swapchain = true;
                return Ok(());
            }
        };

        self.wait_resource_available()?;

        let current_command_buffer = self.data.command_buffers[self.current_frame_index];
        self.data.uniform_buffer.update(
            &self.base.device,
            self.data.world_view.as_void_ptr(),
            &[self.data.descriptor_sets[self.current_frame_index]],
        );

        unsafe {
            self.base
                .device
                .reset_command_buffer(
                    current_command_buffer,
                    vk::CommandBufferResetFlags::default(),
                )
                .unwrap();
        }

        self.begin_command_buffer();
        self.begin_render_pass();
        self.start_record();
        Ok(())
    }

    #[inline]
    pub fn flush(&mut self) -> Result<(), String> {
        self.end_record();
        self.submit()?;

        if !self.present()? {
            self.rebuild_swapchain = true;
            return Ok(());
        }

        self.current_frame_index = (self.current_frame_index + 1) % MAX_FRAME_DRAWS;

        Ok(())
    }

    #[inline]
    pub fn resize(&mut self, window: &Window) -> Result<(), String> {
        unsafe {
            let _ = self.base.device.device_wait_idle();
        }

        self.set_scissor();
        self.set_viewport();
        self.base.resize(window)?;
        self.data.resize(&self.base)?;

        self.data.push_const.wh_ratio =
            self.base.surface_extent.height as f32 / self.base.surface_extent.width as f32;

        Ok(())
    }

    // #[inline]
    // pub fn free(&self) {
    //     self.data.clean_up(&self.base.device);
    //     self.base.clean_up();
    // }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.base.device.device_wait_idle().unwrap();
        }

        if self.meshes_buffers.len() != 0 {
            self.meshes_buffers.iter().for_each(|b| {
                b[0].free(&self.base.device);
                b[1].free(&self.base.device)
            });
        }

        self.data.clean_up(&self.base.device);
        self.base.clean_up();
    }
}
