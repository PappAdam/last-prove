use vulkano::pipeline::Pipeline;
use vulkano::sync::{self, GpuFuture};
use vulkano::{command_buffer::RenderPassBeginInfo, swapchain::PresentInfo};

use crate::map::tile::TileFlag;

use super::{shaders, VulkanApp};
impl VulkanApp {
    pub fn render(&mut self) {
        if self.recreate_swapchain {
            self.recreate_swapchain();
        }
    
        let draw_image_future = match self.acquire_swapchain_image() {
            Some(future) => future,
            None => return,
        };
    
        let push_constants = shaders::tile_vertex_shader::ty::Camera {
            coordinates: self.camera.coordinates.into(),
            tile_size: (2.0 / self.camera.tiles_fit).into(),
            size: self.camera.camera_size.into(),
        };
    
        let mut cmd_buffer_builder = self.create_cmd_buffer_builder();
        cmd_buffer_builder
            //General setup
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: self.clear_values.clone(),
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[self.draw_image_index].clone(),
                    )
                },
                vulkano::command_buffer::SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()])
            .push_constants(self.gameobject_pipeline.layout().clone(), 0, push_constants)
            //Tile rendering
            .bind_pipeline_graphics(self.gameobject_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                self.gameobject_pipeline.layout().clone(),
                0,
                self.tile_texture_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.device_local_tile_instance_buffer.clone())
            .draw(4, self.map.num_of_vulkan_instances, 0, 0)
            .unwrap();
        //Building rendering, pipeline is the same.
        if self.building_instance_count > 0 {
            cmd_buffer_builder
                .bind_vertex_buffers(0, self.device_local_building_instance_buffer.clone())
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    self.gameobject_pipeline.layout().clone(),
                    0,
                    self.building_texture_descriptor_set.clone(),
                )
                .draw(4, self.building_instance_count as u32, 0, 0)
                .unwrap();
        }
    
        //HUD rendering
        cmd_buffer_builder
            .bind_pipeline_graphics(self.hud_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                self.hud_pipeline.layout().clone(),
                0,
                self.hud_texture_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.device_local_hud_instance_buffer.clone())
            .draw(4, self.hud_instance_count as u32, 0, 0)
            .unwrap();
    
        cmd_buffer_builder.end_render_pass().unwrap();
    
        let cmd_buffer = cmd_buffer_builder.build().unwrap();
    
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
    
        let render_future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(draw_image_future)
            .then_execute(self.graphics_queue.clone(), cmd_buffer)
            .unwrap()
            .then_swapchain_present(
                self.graphics_queue.clone(),
                PresentInfo {
                    index: self.draw_image_index,
                    ..PresentInfo::swapchain(self.swapchain.clone())
                },
            )
            .then_signal_fence_and_flush();
    
        match render_future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(vulkano::sync::FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
    
    pub fn refresh_game(&mut self, delta_time: f32) {
        self.process_input_commands();
    
        self.camera.refresh_camera(&self.input, delta_time);
        self.input.refresh_input(delta_time);
    }
    
    fn process_input_commands(&mut self) {
        if self
            .input
            .get_mousebutton_pressed(winit::event::MouseButton::Left)
        {
            let mouse_position = self.input.get_mouse_position();
            let mouse_coordinates = self
                .camera
                .screen_position_to_tile_coordinates(mouse_position);
    
            //Clicked a hud object
            if let Some(_hud_object) = self
                .camera
                .get_hud_object_at_screen_position(mouse_position)
            {
                //do stuff with hud
                return;
            }
            //Clicked a tile
            if let Some(clicked_tile) = self.map.get_shown_tile_at_coordinates(mouse_coordinates) {
                //No building on top
                if clicked_tile.flags & TileFlag::BuildingOnTop as u8
                    != TileFlag::BuildingOnTop as u8
                {
                    self.map.build_building(clicked_tile.coordinates.into(), 0);
                    self.copy_into_building_buffer();
                }
                //Has building on top
                else {
                    self.camera.hud_objects[1].toggle_visibility()
                }
            }
        }
        for key_pressed in &self.input.keys_pressed_this_frame {
            self.camera.refresh_hud_on_key_press(*key_pressed);
        }
        self.copy_into_hud_buffer();
    }
}
