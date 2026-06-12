#[cfg(windows)]
fn main() {
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon("assets/app-icon.ico");
    resource.set("FileDescription", "Thesis Architect");
    resource.set("ProductName", "Thesis Architect");
    resource.set("CompanyName", "Thesis Architect");
    resource.set("LegalCopyright", "MIT License");

    if let Err(error) = resource.compile() {
        eprintln!("failed to embed Windows application icon: {error}");
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {}
