mod base;
mod data;
mod draw_setup;
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
        unsafe {
            self.base
                .device
                .reset_command_buffer(
                    self.data.command_buffers[self.current_frame_index],
                    vk::CommandBufferResetFlags::default(),
                )
                .unwrap();
        }

        self.begin_command_buffer();
        self.begin_render_pass();
        self.set_viewport();
        self.set_scissor();

        unsafe {
            self.base.device.cmd_set_viewport(
                self.data.command_buffers[self.current_frame_index],
                0,
                &[self.data.viewport],
            );

            self.base.device.cmd_set_scissor(
                self.data.command_buffers[self.current_frame_index],
                0,
                &[self.data.scissor],
            );

            self.base
                .device
                .cmd_end_render_pass(self.data.command_buffers[self.current_frame_index]);

            self.base
                .device
                .end_command_buffer(self.data.command_buffers[self.current_frame_index])
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
