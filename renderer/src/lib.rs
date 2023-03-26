mod base;
mod data;
mod draw_setup;
pub mod engine;
mod resources;
mod setup;
mod utils;

use ash::vk;
use utils::buffer_data::BufferObject;
use winit::window::Window;

use crate::{base::RenderBase, data::RenderData, utils::MAX_FRAME_DRAWS};

pub struct Renderer {
    pub data: RenderData,
    pub base: RenderBase,

    pub current_frame_index: usize,
    pub rebuild_swapchain: bool,
    pub image_index: usize,

    pub rotation: f32,
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
            rotation: 0.,
        })
    }

    #[inline]
    pub fn draw(&mut self) -> Result<(), String> {
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
            self.data.transform.as_void_ptr(),
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
        self.record_commands();

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
