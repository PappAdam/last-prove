mod base;
mod data;
mod draw_setup;
mod engine;
mod resources;
mod utils;

use std::time::Duration;

use ash::vk;
use winit::window::Window;

use crate::{base::RenderBase, data::RenderData, utils::MAX_FRAME_DRAWS};

pub struct Renderer {
    pub data: RenderData,
    pub base: RenderBase,

    pub current_frame_index: usize,
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
            rebuild_swapchain: true,
            image_index: 0,
        })
    }

    #[allow(unused)]
    #[inline]
    pub fn draw(&mut self, delta_time: &Duration) -> Result<(), String> {
        self.image_index = match self.get_img_index()? {
            Some(index) => index as usize,
            None => {
                self.rebuild_swapchain = true;
                return Ok(());
            }
        };

        self.wait_resource_available()?;
        let current_command_buffer = self.data.command_buffers[self.current_frame_index];

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

        unsafe {
            self.base.device.cmd_bind_pipeline(
                current_command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.data.pipeline,
            );

            self.base
                .device
                .cmd_set_viewport(current_command_buffer, 0, &[self.data.viewport]);

            self.base
                .device
                .cmd_set_scissor(current_command_buffer, 0, &[self.data.scissor]);

            self.base.device.cmd_bind_vertex_buffers(
                current_command_buffer,
                0,
                &[self.data.vertex_buffer.buf],
                &[0],
            );

            self.base.device.cmd_bind_index_buffer(
                current_command_buffer,
                self.data.index_buffer.buf,
                0,
                vk::IndexType::UINT16,
            );

            self.base.device.cmd_draw_indexed(
                current_command_buffer,
                self.data.index_count,
                1000,
                0,
                0,
                0,
            );

            self.base.device.cmd_end_render_pass(current_command_buffer);

            self.base
                .device
                .end_command_buffer(current_command_buffer)
                .map_err(|_| String::from("failed to end command buffer"))?
        }
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

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            let _ = self.base.device.device_wait_idle();
            self.data.clean_up(&self.base.device);
            self.base.clean_up();
        }
    }
}
