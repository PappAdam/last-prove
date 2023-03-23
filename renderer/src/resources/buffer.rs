use std::{ffi::c_void, mem::size_of, ptr::copy_nonoverlapping};

use ash::vk::{self};

use crate::parse_error;

pub struct Buffer {
    pub mem: vk::DeviceMemory,
    pub buf: vk::Buffer,
}

pub struct UniformBuffer {
    pub mem: vk::DeviceMemory,
    pub buf: vk::Buffer,
    pub data: *mut c_void,
    pub size: u64,
    pub binding: u8,
}

impl UniformBuffer {
    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buf, None);
            device.unmap_memory(self.mem);
            device.free_memory(self.mem, None);
        }
    }

    #[inline]
    pub fn update(
        &self,
        device: &ash::Device,
        data: *const c_void,
        descriptor_sets: &[vk::DescriptorSet],
    ) {
        unsafe {
            copy_nonoverlapping(data, self.data, self.size as usize);
            let buffer_info = vk::DescriptorBufferInfo {
                buffer: self.buf,
                range: self.size,
                ..Default::default()
            };

            descriptor_sets.iter().for_each(|set| {
                let descriptor_write = vk::WriteDescriptorSet {
                    dst_set: *set,
                    dst_binding: self.binding as u32,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_buffer_info: &buffer_info,
                    ..Default::default()
                };

                device.update_descriptor_sets(&[descriptor_write], &[]);
            });
        }
    }
}

impl Buffer {
    #[inline]
    pub fn uniform_buffer(
        device: &ash::Device,
        buffer_size: u64,
        data: *const c_void,
        memory_props: vk::PhysicalDeviceMemoryProperties,
    ) -> Result<UniformBuffer, String> {
        let uniform_buffer = Buffer::new(
            device,
            size_of::<UniformBuffer>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            memory_props,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let buffer_ptr = unsafe {
            device
                .map_memory(
                    uniform_buffer.mem,
                    0,
                    size_of::<UniformBuffer>() as u64,
                    vk::MemoryMapFlags::empty(),
                )
                .map_err(|err| format!("{err}"))?
        };

        unsafe {
            copy_nonoverlapping(data, buffer_ptr, buffer_size as usize);
        }

        Ok(UniformBuffer {
            mem: uniform_buffer.mem,
            buf: uniform_buffer.buf,
            data: buffer_ptr,
            size: buffer_size,
            binding: 0,
        })
    }

    #[inline]
    pub fn device_local(
        device: &ash::Device,
        data: *const c_void,
        buffer_size: u64,
        memory_props: vk::PhysicalDeviceMemoryProperties,
        usage: vk::BufferUsageFlags,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
    ) -> Result<Self, String> {
        let staging_buffer = Buffer::new(
            device,
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            memory_props,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        unsafe {
            let mapped_data = device
                .map_memory(
                    staging_buffer.mem,
                    0,
                    buffer_size,
                    vk::MemoryMapFlags::empty(),
                )
                .map_err(|err| format!("{err}"))?;
            copy_nonoverlapping(data, mapped_data, buffer_size as usize);
            device.unmap_memory(staging_buffer.mem);
        };

        let device_local_buffer = Buffer::new(
            device,
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | usage,
            memory_props,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        staging_buffer.copy(
            device,
            &device_local_buffer,
            buffer_size,
            queue,
            command_pool,
        )?;

        staging_buffer.free(device);

        Ok(device_local_buffer)
    }

    #[inline]
    pub fn new(
        device: &ash::Device,
        buffer_size: u64,
        buffer_usage: vk::BufferUsageFlags,
        memory_props: vk::PhysicalDeviceMemoryProperties,
        buffer_props: vk::MemoryPropertyFlags,
    ) -> Result<Self, String> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(buffer_size)
            .usage(buffer_usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();

        let buffer = unsafe {
            device
                .create_buffer(&buffer_info, None)
                .map_err(|msg| format!("{}", msg))?
        };

        let mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };

        let mem_type_index =
            find_memory_type_index(memory_props, mem_req.memory_type_bits, buffer_props)?;

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_req.size)
            .memory_type_index(mem_type_index);

        let memory = unsafe {
            device
                .allocate_memory(&alloc_info, None)
                .map_err(|msg| format!("{}", msg))?
        };

        unsafe {
            device
                .bind_buffer_memory(buffer, memory, 0)
                .map_err(|msg| format!("{}", msg))?
        };

        Ok(Self {
            buf: buffer,
            mem: memory,
        })
    }

    #[inline]
    pub fn copy(
        &self,
        device: &ash::Device,
        dst_buffer: &Buffer,
        buffer_size: u64,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
    ) -> Result<(), String> {
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(vk::CommandBufferLevel::PRIMARY)
            .build();

        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&allocate_info)
                .map_err(|msg| format!("{}", msg))?[0]
        };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        let buffer_copy_region = vk::BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size(buffer_size)
            .build();

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer])
            .build();

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .map_err(|msg| format!("{}", msg))?;

            device.cmd_copy_buffer(
                command_buffer,
                self.buf,
                dst_buffer.buf,
                &[buffer_copy_region],
            );

            device
                .end_command_buffer(command_buffer)
                .map_err(|msg| format!("{}", msg))?;

            device
                .queue_submit(queue, &[submit_info], vk::Fence::null())
                .map_err(|msg| format!("{}", msg))?;
            device
                .queue_wait_idle(queue)
                .map_err(|msg| format!("{}", msg))?;
            device.free_command_buffers(command_pool, &[command_buffer]);
        }

        Ok(())
    }

    #[inline]
    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buf, None);
            device.free_memory(self.mem, None);
        }
    }
}

pub fn find_memory_type_index(
    memory_props: vk::PhysicalDeviceMemoryProperties,
    allowed_types: u32,
    props: vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    match memory_props
        .memory_types
        .iter()
        .enumerate()
        .find(|(i, mem_type)| {
            allowed_types & (1 << i) != 0 && mem_type.property_flags & props == props
        })
        .map(|(i, _)| i as u32)
    {
        Some(ind) => Ok(ind),
        None => Err(parse_error!("Failed to suitable memory")),
    }
}
