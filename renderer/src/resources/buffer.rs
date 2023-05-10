use std::{ffi::c_void, mem::size_of, ptr::copy_nonoverlapping};

use ash::vk::{self};

use crate::parse_error;

// #[derive(Copy, Clone, Debug)]
pub struct Buffer {
    pub mem: vk::DeviceMemory,
    pub buf: vk::Buffer,
}

#[derive(Copy, Clone, Debug)]
pub struct UniformBuffer {
    pub mem: vk::DeviceMemory,
    pub buf: vk::Buffer,
    pub buffer_pointer: *mut c_void,
    pub size: u64,
    pub binding: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct DynamicUniformBuffer {
    pub mem: vk::DeviceMemory,
    pub buf: vk::Buffer,
    pub buffer_pointer: *mut c_void,
    pub size: u64,
    pub binding: u32,
    pub alignment: usize,
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
            copy_nonoverlapping(data, self.buffer_pointer, self.size as usize);
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

impl DynamicUniformBuffer {
    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buf, None);
            device.unmap_memory(self.mem);
            device.free_memory(self.mem, None);
        }
    }

    #[inline]
    pub fn update(&self, device: &ash::Device, descriptor_sets: &[vk::DescriptorSet]) {
        unsafe {
            let mem_range = vk::MappedMemoryRange::builder()
                .memory(self.mem)
                .offset(0)
                .size(self.size * self.alignment as u64)
                .build();

            let _ = device.flush_mapped_memory_ranges(&[mem_range]);

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
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                    p_buffer_info: &buffer_info,
                    ..Default::default()
                };

                device.update_descriptor_sets(&[descriptor_write], &[]);
            });
        }
    }

    #[inline]
    pub fn update_mesh(
        &self,
        device: &ash::Device,
        descriptor_sets: &[vk::DescriptorSet],
        mesh_index: usize,
    ) {
        unsafe {
            let mem_range = vk::MappedMemoryRange::builder()
                .memory(self.mem)
                .offset(mesh_index as u64 * self.alignment as u64)
                .size(self.alignment as u64)
                .build();

            let _ = device.flush_mapped_memory_ranges(&[mem_range]);

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
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
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
    pub fn uniform_buffer<T>(
        device: &ash::Device,
        data: *const c_void,
        memory_props: vk::PhysicalDeviceMemoryProperties,
        binding: u32,
    ) -> Result<UniformBuffer, String> {
        let uniform_buffer = Buffer::new(
            device,
            size_of::<T>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            memory_props,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let buffer_pointer = unsafe {
            device
                .map_memory(
                    uniform_buffer.mem,
                    0,
                    size_of::<T>() as u64,
                    vk::MemoryMapFlags::empty(),
                )
                .map_err(|err| format!("{err}"))?
        };

        unsafe {
            copy_nonoverlapping(data, buffer_pointer, size_of::<T>());
        }

        Ok(UniformBuffer {
            mem: uniform_buffer.mem,
            buf: uniform_buffer.buf,
            buffer_pointer,
            size: size_of::<T>() as u64,
            binding,
        })
    }

    pub fn dynamic_uniform_buffer<T>(
        device: &ash::Device,
        memory_props: vk::PhysicalDeviceMemoryProperties,
        device_props: vk::PhysicalDeviceProperties,
        size: usize,
        binding: u32,
    ) -> Result<DynamicUniformBuffer, String> {
        let min_buffer_alignment = device_props.limits.min_uniform_buffer_offset_alignment as usize;
        let mut dynamic_alignment = size_of::<T>();
        if min_buffer_alignment > 0 {
            dynamic_alignment =
                (dynamic_alignment + min_buffer_alignment - 1) & !(min_buffer_alignment - 1);
        }

        let uniform_buffer = Buffer::new(
            device,
            dynamic_alignment as u64 * size as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            memory_props,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        let buffer_pointer = unsafe {
            device
                .map_memory(
                    uniform_buffer.mem,
                    0,
                    dynamic_alignment as u64 * size as u64,
                    vk::MemoryMapFlags::empty(),
                )
                .map_err(|err| format!("{err}"))?
        };

        Ok(DynamicUniformBuffer {
            mem: uniform_buffer.mem,
            buf: uniform_buffer.buf,
            size: size as u64,
            buffer_pointer,
            binding,
            alignment: dynamic_alignment,
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
        None => Err(parse_error!("Failed to find suitable memory")),
    }
}
