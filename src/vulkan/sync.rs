use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use crate::app::AppData;

// ── Constants ─────────────────────────────────────────────────────────────────

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

// ── Synchronisation ─────────────────────────────────────────────────────────────────

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.in_flight_fences
            .push(device.create_fence(&fence_info, None)?);
    }

    Ok(())
}

pub unsafe fn create_swapchain_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();

    data.render_finished_semaphores.clear();
    for _ in 0..data.swapchain_images.len() {
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
    }

    data.images_in_flight = vec![vk::Fence::null(); data.swapchain_images.len()];

    Ok(())
}
