fn main() {
    let output = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/ipc/bindings.ts");
    nibble_desktop::command_builder()
        .export(specta_typescript::Typescript::default(), output)
        .expect("binding export failed");
}
