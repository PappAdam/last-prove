use std::sync::Arc;

use bytemuck::Pod;
use vulkano::{
    buffer::{BufferContents, DeviceLocalBuffer, CpuAccessibleBuffer, BufferUsage},
    command_buffer::{
        pool::standard::StandardCommandPoolAlloc, AutoCommandBufferBuilder,
        CommandBufferExecFuture, PrimaryAutoCommandBuffer, CopyBufferInfo,
    },
    device::{Device, Queue},
    sync::{FenceSignalFuture, NowFuture, self, GpuFuture},
};

use super::VulkanApp;

impl VulkanApp {
    pub fn copy_into_building_buffer(&mut self) {
        let gpu_stored_building_vector = self.map.get_building_instance_coordinates();
        self.building_instance_count = gpu_stored_building_vector.len() as u16;
        if self.building_instance_count == 0 {
            return;
        }
        (self.device_local_building_instance_buffer, _) = Self::create_device_local_buffer(
            self.device.clone(),
            self.graphics_queue.clone(),
            gpu_stored_building_vector,
        );
    }
    pub fn copy_into_troop_buffer(&mut self) {
        let gpu_stored_troop_vector = self.map.get_troop_instance_coordinates();
        self.troop_instance_count = gpu_stored_troop_vector.len() as u16;
        if self.troop_instance_count == 0 {
            return;
        }
        (self.device_local_troop_instance_buffer, _) = Self::create_device_local_buffer(
            self.device.clone(),
            self.graphics_queue.clone(),
            gpu_stored_troop_vector,
        );
    }
    pub fn copy_into_hud_buffer(&mut self) {
        let gpu_stored_hud_vector = self.camera.get_hud_instance_coordinates();
        self.hud_instance_count = gpu_stored_hud_vector.len() as u8;
        if self.hud_instance_count == 0 {
            return;
        }
        (self.device_local_hud_instance_buffer, _) = Self::create_device_local_buffer(
            self.device.clone(),
            self.graphics_queue.clone(),
            gpu_stored_hud_vector,
        );
    }

    pub fn create_cmd_buffer_builder(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.graphics_queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap()
    }

    pub fn create_device_local_buffer<T>(
        device: Arc<Device>,
        queue: Arc<Queue>,
        data: Vec<T>,
    ) -> (
        Arc<DeviceLocalBuffer<[T]>>,
        FenceSignalFuture<
            CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer<StandardCommandPoolAlloc>>,
        >,
    )
    where
        T: Pod + BufferContents,
    {
        let data_len = data.len() as u64;
        let cpu_accessible_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage {
                vertex_buffer: true,
                transfer_src: true,
                ..Default::default()
            },
            false,
            data,
        )
        .unwrap();

        let device_local_buffer = DeviceLocalBuffer::array(
            device.clone(),
            data_len,
            BufferUsage {
                vertex_buffer: true,
                transfer_dst: true,
                ..Default::default()
            },
            [queue.queue_family_index()],
        )
        .unwrap();

        let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
            device.clone(),
            queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        cmd_buffer_builder
            .copy_buffer(CopyBufferInfo::buffers(
                cpu_accessible_buffer,
                device_local_buffer.clone(),
            ))
            .unwrap();

        let cmd_buffer = cmd_buffer_builder.build().unwrap();

        let copy_future = sync::now(device)
            .then_execute(queue.clone(), cmd_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        (device_local_buffer, copy_future)
    }
}
