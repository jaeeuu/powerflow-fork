use std::process;

use objc2_app_kit::NSWindowCollectionBehavior;
use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, Manager, Runtime,
};
use tauri_plugin_nspopover::{AppExt, ToPopoverOptions, WindowExt as _};
use tauri_specta::Event;

use crate::{event::PowerUpdatedEvent, ext::WebviewWindowExt};

/// `NSPopUpMenuWindowLevel`. Above a fullscreen app's own windows, which sit
/// at the normal level, so the panel is not buried when one is active.
const NS_POPUP_MENU_WINDOW_LEVEL: i64 = 101;
/// `NSWindowCollectionBehaviorCanJoinAllSpaces | ...FullScreenAuxiliary`,
/// which lets the panel appear over the current space, including the one a
/// fullscreen app occupies, instead of forcing a space switch.

/// Raise the popover above fullscreen windows.
///
/// The popover is an `NSPopover`, so the `alwaysOnTop` flag on the Tauri
/// window does not apply: `to_popover()` moves that window's contentView into
/// an `NSViewController`, and what is displayed is the popover's own window.
/// That window only exists once the popover is shown, so this runs after
/// `show_popover()`.
fn raise_popover_window<R: Runtime>(app: &impl Manager<R>) {
    let popover = app.app_handle().ns_popover();
    unsafe {
        let Some(controller) = popover.contentViewController() else {
            return;
        };
        let Some(view) = controller.view().window() else {
            return;
        };
        view.setLevel(NS_POPUP_MENU_WINDOW_LEVEL as _);
        view.setCollectionBehavior(
            NSWindowCollectionBehavior::CanJoinAllSpaces
                | NSWindowCollectionBehavior::FullScreenAuxiliary,
        );
    }
}

pub fn setup_tray_icon<R: Runtime>(app: &impl Manager<R>) -> tauri::Result<()> {
    let show = MenuItemBuilder::new("Show Window").build(app)?;
    let quit = MenuItemBuilder::new("Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show)
        .separator()
        .item(&quit)
        .build()?;

    let tray_icon = TrayIconBuilder::with_id("main").title("0 w").build(app)?;

    // NOTE: the menu is deliberately not attached here. `NSStatusItem.setMenu`
    // makes AppKit handle the click itself and swallow `mouseDown:` before it
    // reaches tray-icon's TrayTarget subview, so no Click event is ever
    // emitted and the popover can never open. `menu_on_left_click(false)` only
    // flips tray-icon's own ivar; the AppKit-level interception stays. The
    // menu is attached on demand when the right button goes down.
    let menu_for_right_click = menu.clone();

    tray_icon.on_menu_event(move |tray_handle, event| match event.id() {
        val if val == show.id() => {
            let Ok((window, _)) = tray_handle.app_handle().get_or_create_window("main") else {
                log::error!("Main window is unavailable");
                return;
            };

            if let Err(error) = window.show().and_then(|_| window.set_focus()) {
                log::error!("Failed to show main window: {error}");
            }
            tray_handle.app_handle().hide_popover();
            if let Err(error) = tray_handle
                .app_handle()
                .set_activation_policy(ActivationPolicy::Regular)
            {
                log::error!("Failed to restore activation policy: {error}");
            }
        }
        val if val == quit.id() => {
            tray_handle.app_handle().cleanup_before_exit();
            process::exit(0);
        }
        _ => {}
    });

    tray_icon.on_tray_icon_event(move |tray_handle, event| {
        match event {
            // Left button toggles the popover.
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let handle = tray_handle.app_handle();
                if handle.is_popover_shown() {
                    handle.hide_popover();
                } else {
                    handle.show_popover();
                    // The popover's window only exists once it is shown, so
                    // the level has to be raised here rather than at setup.
                    raise_popover_window(handle);
                }
            }
            // Right button gets the context menu. The menu cannot stay
            // attached: `NSStatusItem.setMenu` makes AppKit swallow
            // `mouseDown:` so left clicks would stop emitting Click events.
            // Attach it for this click, ask the button to perform its click so
            // AppKit actually pops the menu (by the time this handler runs
            // AppKit has already decided how to treat the press), then detach.
            // `performClick` blocks until the menu closes, so the matching
            // mouse-up may never arrive and cannot be relied on for cleanup.
            TrayIconEvent::Click {
                button: MouseButton::Right,
                button_state: MouseButtonState::Down,
                ..
            } => {
                let handle = tray_handle.app_handle();
                if let Err(error) = tray_handle.set_menu(Some(menu_for_right_click.clone())) {
                    log::error!("failed to attach tray menu: {error}");
                    return;
                }
                let button = handle.ns_statusbar_button();
                unsafe { button.performClick(None) };
                if let Err(error) = tray_handle.set_menu(None::<tauri::menu::Menu<R>>) {
                    log::error!("failed to detach tray menu: {error}");
                }
            }
            _ => {}
        }
    });

    let last_title = Mutex::new(String::new());
    PowerUpdatedEvent::listen(app.app_handle(), move |event| {
        let Ok(mut previous) = last_title.lock() else {
            return;
        };
        if *previous == event.payload.0 {
            return;
        }
        match tray_icon.set_title(Some(&event.payload.0)) {
            Ok(()) => *previous = event.payload.0,
            Err(error) => log::warn!("Failed to update tray title: {error}"),
        }
    });

    match app.popover_window() {
        Some(window) => window.to_popover(ToPopoverOptions {
            is_fullsize_content: false,
        }),
        None => log::error!("popover window not found; left click will do nothing"),
    }

    Ok(())
}
