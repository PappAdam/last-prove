use ash::vk;

use crate::parse_error;

use super::buffer::find_memory_type_index;

pub struct Image {
    pub img: vk::Image,
    pub mem: vk::DeviceMemory,
    pub view: vk::ImageView,
}

impl Image {
    pub fn new(
        device: &ash::Device,
        extent: vk::Extent3D,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        aspect_mask: vk::ImageAspectFlags,
        properties: vk::MemoryPropertyFlags,
        memory_props: vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, String> {
        let img = Self::create_image(device, extent, format, tiling, usage)?;
        let mem = Self::create_mem(device, properties, memory_props, img)?;
        let view = Self::create_image_view(device, img, format, aspect_mask)?;

        Ok(Self { img, mem, view })
    }

    #[inline]
    pub fn create_mem(
        device: &ash::Device,
        properties: vk::MemoryPropertyFlags,
        memory_props: vk::PhysicalDeviceMemoryProperties,
        img: vk::Image,
    ) -> Result<vk::DeviceMemory, String> {
        let mem_reqs = unsafe { device.get_image_memory_requirements(img) };

        let mem_type_index =
            find_memory_type_index(memory_props, mem_reqs.memory_type_bits, properties)?;

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_reqs.size)
            .memory_type_index(mem_type_index)
            .build();

        let mem = unsafe {
            let mem = device
                .allocate_memory(&alloc_info, None)
                .map_err(|err| parse_error!(err))?;

            device
                .bind_image_memory(img, mem, 0)
                .map_err(|err| parse_error!(err))?;

            mem
        };

        Ok(mem)
    }

    #[inline]
    pub fn create_image(
        device: &ash::Device,
        extent: vk::Extent3D,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
    ) -> Result<vk::Image, String> {
        let create_info = vk::ImageCreateInfo::builder()
            .extent(extent)
            .image_type(vk::ImageType::TYPE_2D)
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .usage(usage)
            .samples(vk::SampleCountFlags::TYPE_1)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();

        let img = unsafe {
            device
                .create_image(&create_info, None)
                .map_err(|err| parse_error!(err))?
        };

        Ok(img)
    }

    #[inline]
    pub fn create_image_view(
        device: &ash::Device,
        image: vk::Image,
        format: vk::Format,
        aspect_mask: vk::ImageAspectFlags,
    ) -> Result<vk::ImageView, String> {
        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            })
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .build();

        let img_view = unsafe {
            device
                .create_image_view(&create_info, None)
                .map_err(|err| parse_error!(err))?
        };

        Ok(img_view)
    }

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_image(self.img, None);
            device.destroy_image_view(self.view, None);
            device.free_memory(self.mem, None);
        }
    }
}
