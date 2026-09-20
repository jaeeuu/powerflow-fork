#[test]
fn export_bindings() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.ts");
    powerflow_lib::create_specta()
        .export(specta_typescript::Typescript::default(), &path)
        .expect("Failed to export TypeScript bindings");
    let generated = std::fs::read_to_string(&path).expect("Failed to read bindings");
    let body = generated
        .lines()
        .skip(1)
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(
        path,
        format!("// Automatically generated. Do not edit.\n{body}\n"),
    )
    .expect("Failed to format bindings");
}
