// build.rs
use std::env;

fn main() {
    // 设置环境变量 TEST_FOO 为当前时间戳（用于 tests7）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 设置 pass 特性标志（用于 tests8）
    println!("cargo:rustc-cfg=feature=\"pass\"");
    
    // 如果你想要在编译时重新运行 build.rs 当某些文件变化时
    println!("cargo:rerun-if-changed=build.rs");
}