use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;

use glib::object::ObjectExt;
use glib::translate::{FromGlib, FromGlibPtrFull, IntoGlib};
use libfprint_rs::{FpContext, FpDevice};

fn send_command(socket: &Path, command: &str) {
    UnixStream::connect(socket)
        .expect("connect to the virtual device")
        .write_all(command.as_bytes())
        .expect("send a virtual device command");
}

fn device_error_code(nick: &str) -> i32 {
    let type_ =
        unsafe { glib::Type::from_glib(libfprint_sys::fp_device_error_get_type() as usize) };
    glib::EnumClass::with_type(type_)
        .unwrap()
        .value_by_nick(nick)
        .unwrap()
        .value()
}

fn assert_device_error(error: libfprint_rs::GError, nick: &str) {
    assert_eq!(error.domain().into_glib(), unsafe {
        libfprint_sys::fp_device_error_quark()
    });
    assert_eq!(error.code(), device_error_code(nick));
}

#[test]
fn list_prints_storage_contract() {
    if std::env::var_os("LIBFPRINT_RS_TEST_CHILD").is_none() {
        let directory =
            std::env::temp_dir().join(format!("libfprint-rs-list-{}", std::process::id()));
        std::fs::create_dir(&directory).expect("create a test directory");
        let status = Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", "list_prints_storage_contract", "--nocapture"])
            .env("LIBFPRINT_RS_TEST_CHILD", "1")
            .env(
                "FP_DRIVERS_ALLOWLIST",
                "virtual_device_storage:virtual_device",
            )
            .env(
                "FP_VIRTUAL_DEVICE_STORAGE",
                directory.join("storage.socket"),
            )
            .env("FP_VIRTUAL_DEVICE", directory.join("device.socket"))
            .status()
            .expect("run the virtual device test");
        std::fs::remove_dir_all(directory).expect("remove the test directory");
        assert!(status.success());
        return;
    }

    let context = unsafe { FpContext::from_glib_full(libfprint_sys::fp_context_new()) };
    let device: FpDevice = context
        .devices()
        .into_iter()
        .find(|device| device.driver() == "virtual_device_storage")
        .expect("build libfprint with the virtual_device_storage driver");

    assert_device_error(device.list_prints_sync(None).unwrap_err(), "not-open");

    device.open_sync(None).unwrap();
    assert!(device.list_prints_sync(None).unwrap().is_empty());
    let cancellable = gio::Cancellable::new();
    assert!(
        device
            .list_prints_sync(Some(&cancellable))
            .unwrap()
            .is_empty()
    );
    let socket = std::env::var_os("FP_VIRTUAL_DEVICE_STORAGE").unwrap();
    let socket = Path::new(&socket);

    send_command(socket, "INSERT first-print");
    assert_eq!(device.list_prints_sync(None).unwrap().len(), 1);
    send_command(socket, "INSERT second-print");
    let prints = device.list_prints_sync(None).unwrap();
    assert_eq!(prints.len(), 2);
    assert_ne!(prints[0], prints[1]);

    let weak_prints: Vec<_> = prints.iter().map(ObjectExt::downgrade).collect();
    let retained = prints[0].clone();
    let retained_weak = retained.downgrade();
    drop(prints);
    assert_eq!(
        weak_prints
            .iter()
            .filter(|print| print.upgrade().is_some())
            .count(),
        1
    );

    send_command(socket, &format!("ERROR {}", device_error_code("data-full")));
    assert_device_error(device.list_prints_sync(None).unwrap_err(), "data-full");
    assert_eq!(device.list_prints_sync(None).unwrap().len(), 2);
    device.close_sync(None).unwrap();
    drop(device);
    drop(context);
    assert_eq!(retained.driver(), "virtual_device_storage");
    assert!(!retained.serialize().unwrap().is_empty());
    drop(retained);
    assert!(retained_weak.upgrade().is_none());
    assert!(weak_prints.iter().all(|print| print.upgrade().is_none()));

    let context = unsafe { FpContext::from_glib_full(libfprint_sys::fp_context_new()) };
    let device = context
        .devices()
        .into_iter()
        .find(|device| device.driver() == "virtual_device")
        .expect("build libfprint with the virtual_device driver");
    device.open_sync(None).unwrap();
    assert_device_error(device.list_prints_sync(None).unwrap_err(), "not-supported");
    device.close_sync(None).unwrap();
}

#[test]
#[ignore = "requires a connected UPEK TouchStrip reader"]
fn list_prints_upekts_reports_unsupported() {
    let context = unsafe { FpContext::from_glib_full(libfprint_sys::fp_context_new()) };
    let device = context
        .devices()
        .into_iter()
        .find(|device| device.driver() == "upekts")
        .expect("connect a UPEK TouchStrip reader");
    device.open_sync(None).unwrap();
    let result = device.list_prints_sync(None);
    device.close_sync(None).unwrap();
    assert_device_error(result.unwrap_err(), "not-supported");
}
