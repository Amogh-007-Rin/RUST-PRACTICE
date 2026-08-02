use module_081_exercises::{ExportError, LinearMemory, MemoryError, WasmModule};

#[test]
fn new_memory_has_requested_size() {
    let mem = LinearMemory::new(16);
    assert_eq!(mem.size(), 16);
    assert_eq!(mem.max_size(), None);
}

#[test]
fn memory_initialized_to_zero() {
    let mem = LinearMemory::new(4);
    assert_eq!(mem.as_slice(), &[0, 0, 0, 0]);
}

#[test]
fn store_then_load_roundtrips() {
    let mut mem = LinearMemory::new(8);
    mem.store_u8(3, 42).unwrap();
    assert_eq!(mem.load_u8(3).unwrap(), 42);
    assert_eq!(mem.as_slice(), &[0, 0, 0, 42, 0, 0, 0, 0]);
}

#[test]
fn load_at_end_of_memory_errors() {
    let mem = LinearMemory::new(4);
    assert_eq!(
        mem.load_u8(4),
        Err(MemoryError::OutOfBounds {
            offset: 4,
            length: 1,
            size: 4,
        })
    );
}

#[test]
fn store_out_of_bounds_errors() {
    let mut mem = LinearMemory::new(4);
    assert!(mem.store_u8(4, 1).is_err());
    assert_eq!(mem.size(), 4);
}

#[test]
fn u32_roundtrip_is_little_endian() {
    let mut mem = LinearMemory::new(8);
    mem.store_u32(0, 0xDEAD_BEEF).unwrap();
    assert_eq!(mem.as_slice(), &[0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0]);
    assert_eq!(mem.load_u32(0).unwrap(), 0xDEAD_BEEF);
}

#[test]
fn u32_spanning_past_the_end_errors() {
    let mut mem = LinearMemory::new(5);
    assert_eq!(
        mem.store_u32(2, 1),
        Err(MemoryError::OutOfBounds {
            offset: 2,
            length: 4,
            size: 5,
        })
    );
    assert!(mem.load_u32(3).is_err());
}

#[test]
fn grow_returns_previous_size_and_zero_fills() {
    let mut mem = LinearMemory::new(4);
    mem.store_u8(0, 7).unwrap();
    assert_eq!(mem.grow(10).unwrap(), 4);
    assert_eq!(mem.size(), 14);
    assert_eq!(mem.load_u8(4).unwrap(), 0);
    assert_eq!(mem.load_u8(0).unwrap(), 7);
}

#[test]
fn grow_without_maximum_always_succeeds() {
    let mut mem = LinearMemory::new(1);
    assert!(mem.grow(4096).is_ok());
    assert_eq!(mem.size(), 4097);
}

#[test]
fn grow_beyond_configured_maximum_errors() {
    let mut mem = LinearMemory::with_max(4, Some(16));
    assert_eq!(
        mem.grow(32),
        Err(MemoryError::GrowOverflow {
            current: 4,
            max: 16
        })
    );
    assert_eq!(mem.size(), 4);
}

#[test]
fn module_starts_empty_and_remembers_imports() {
    let module = WasmModule::new("greeter", 16);
    assert_eq!(module.name(), "greeter");
    assert_eq!(module.import_count(), 0);
    assert_eq!(module.export_count(), 0);
    assert!(module.export_names().is_empty());
}

#[test]
fn add_import_records_pair() {
    let mut module = WasmModule::new("m", 4);
    assert!(module.add_import("env", "log_i32"));
    assert_eq!(module.import_count(), 1);
    assert!(!module.add_import("env", "log_i32"));
    assert_eq!(module.import_count(), 1);
    assert!(module.add_import("env", "log_f64"));
    assert_eq!(module.import_count(), 2);
}

#[test]
fn add_export_then_resolve() {
    let mut module = WasmModule::new("m", 4);
    module.add_export("add", "f0").unwrap();
    assert_eq!(module.export("add").unwrap(), "f0");
    assert_eq!(module.export_names(), vec!["add".to_string()]);
}

#[test]
fn duplicate_export_name_is_rejected() {
    let mut module = WasmModule::new("m", 4);
    module.add_export("add", "f0").unwrap();
    assert_eq!(
        module.add_export("add", "f1"),
        Err(ExportError::AlreadyExported("add".to_string()))
    );
    assert_eq!(module.export("add").unwrap(), "f0");
}

#[test]
fn unknown_export_name_errors() {
    let module = WasmModule::new("m", 4);
    assert_eq!(
        module.export("nope"),
        Err(ExportError::NotFound("nope".to_string()))
    );
}

#[test]
fn exports_sorted_lexicographically() {
    let mut module = WasmModule::new("m", 4);
    module.add_export("zebra", "f0").unwrap();
    module.add_export("apple", "f1").unwrap();
    assert_eq!(
        module.export_names(),
        vec!["apple".to_string(), "zebra".to_string()]
    );
}

#[test]
fn module_memory_is_shared_and_writable() {
    let mut module = WasmModule::new("m", 8);
    module
        .memory_mut()
        .store_u32(0, 1_234_567)
        .expect("in bounds");
    assert_eq!(module.memory().load_u32(0).unwrap(), 1_234_567);
}

#[test]
fn module_simulates_js_side_writing_to_linear_memory() {
    let mut module = WasmModule::new("m", 64);
    {
        let slice = module.memory_mut().as_mut_slice();
        slice[0] = b'h';
        slice[1] = b'i';
    }
    assert_eq!(&module.memory().as_slice()[0..2], b"hi");
}
