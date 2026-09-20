use std::{ffi::c_void, io, thread};

use core_foundation::{
    base::CFRelease,
    runloop::{
        kCFRunLoopDefaultMode, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun,
        CFRunLoopSourceRef,
    },
};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPSNotificationCreateRunLoopSource(
        callback: extern "C" fn(*mut c_void),
        context: *mut c_void,
    ) -> CFRunLoopSourceRef;
}

pub fn watch_power_source(
    notify: impl Fn() + Send + 'static,
) -> io::Result<thread::JoinHandle<()>> {
    thread::Builder::new()
        .name("battery-watcher".into())
        .spawn(move || {
            extern "C" fn callback(context: *mut c_void) {
                if !context.is_null() {
                    let notify = unsafe { &*context.cast::<Box<dyn Fn() + Send>>() };
                    notify();
                }
            }
            let notify: Box<dyn Fn() + Send> = Box::new(notify);
            unsafe {
                let source = IOPSNotificationCreateRunLoopSource(
                    callback,
                    (&notify as *const Box<dyn Fn() + Send>).cast_mut().cast(),
                );
                if source.is_null() {
                    log::warn!("Battery notifications unavailable; using periodic sampling");
                    return;
                }
                let run_loop = CFRunLoopGetCurrent();
                CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);
                CFRunLoopRun();
                core_foundation::runloop::CFRunLoopRemoveSource(
                    run_loop,
                    source,
                    kCFRunLoopDefaultMode,
                );
                CFRelease(source.cast());
            }
            drop(notify);
        })
}
