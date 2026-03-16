fn main() {
    // 跳过图标检查，允许没有图标文件也能构建
    println!("cargo:rustc-env=TAURI_SKIP_ICON_CHECK=1");
    tauri_build::build()
}
