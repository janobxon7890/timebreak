use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorGeometry {
    pub id: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
    pub is_primary: bool,
}

pub trait OverlayBackend {
    fn initialize_overlay(&mut self) -> Result<(), String>;
    fn set_transparent_and_floating(&mut self, window_label: &str) -> Result<(), String>;
    fn list_monitors(&self) -> Vec<MonitorGeometry>;
    fn set_input_passthrough(
        &mut self,
        window_label: &str,
        passthrough: bool,
    ) -> Result<(), String>;
}

pub struct DummyOverlayBackend;

impl OverlayBackend for DummyOverlayBackend {
    fn initialize_overlay(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn set_transparent_and_floating(&mut self, _window_label: &str) -> Result<(), String> {
        Ok(())
    }
    fn list_monitors(&self) -> Vec<MonitorGeometry> {
        vec![MonitorGeometry {
            id: "main".to_string(),
            name: "Primary Display".to_string(),
            x: 0.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
            scale_factor: 2.0,
            is_primary: true,
        }]
    }
    fn set_input_passthrough(
        &mut self,
        _window_label: &str,
        _passthrough: bool,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
#[allow(deprecated, unexpected_cfgs)]
pub mod macos {
    use cocoa::appkit::{NSColor, NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::{id, nil, NO};
    use objc::{msg_send, sel, sel_impl};

    /// Configure a raw NSWindow pointer to be completely transparent, floating always-on-top,
    /// and accessible across all Spaces / Fullscreen applications.
    ///
    /// # Safety
    /// Caller must guarantee `ns_window` is a valid pointer to a Cocoa `NSWindow` instance.
    pub unsafe fn configure_macos_transparent_overlay(ns_window: id) {
        if ns_window == nil {
            return;
        }

        // kCGStatusWindowLevelKey = 25 or NSFloatingWindowLevel = 3
        let floating_level: i64 = 25;
        let _: () = msg_send![ns_window, setLevel: floating_level];

        // Transparent background
        let clear_color = NSColor::clearColor(nil);
        ns_window.setBackgroundColor_(clear_color);
        ns_window.setOpaque_(NO);
        ns_window.setHasShadow_(NO);

        let content_view: id = msg_send![ns_window, contentView];
        if content_view != nil {
            let _: () = msg_send![content_view, setWantsLayer: cocoa::base::YES];
        }

        // Join all spaces and show over full screen apps
        let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
        ns_window.setCollectionBehavior_(behavior);
    }
}
