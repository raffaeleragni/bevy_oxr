mod vulkan;

use bevy::render::renderer::{RenderAdapter, RenderAdapterInfo, RenderDevice, RenderQueue};
use bevy::window::RawHandleWrapper;
use wgpu::Instance;

use crate::input::XrInput;
use crate::resources::{
    XrEnvironmentBlendMode, XrFormat, XrFrameState, XrFrameWaiter, XrInstance, XrResolution,
    XrSession, XrSessionRunning, XrSwapchain, XrViews,
};

use openxr as xr;

pub fn initialize_xr_graphics(
    window: Option<RawHandleWrapper>,
    // Horrible hack to get the Handtacking extension Loaded, Replace with good system to load
    // any extension at some point
    enable_hand_tracking: bool,
) -> anyhow::Result<(
    RenderDevice,
    RenderQueue,
    RenderAdapterInfo,
    RenderAdapter,
    Instance,
    XrInstance,
    XrSession,
    XrEnvironmentBlendMode,
    XrResolution,
    XrFormat,
    XrSessionRunning,
    XrFrameWaiter,
    XrSwapchain,
    XrInput,
    XrViews,
    XrFrameState,
    // Horrible hack to get the Handtacking extension Loaded, Replace with good system to load
    // any extension at some point
    bool,
)> {
    vulkan::initialize_xr_graphics(window,enable_hand_tracking)
}

pub fn xr_entry() -> xr::Entry {
    #[cfg(feature = "linked")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "linked"))]
    let entry = unsafe { xr::Entry::load().unwrap() };
    entry
}
