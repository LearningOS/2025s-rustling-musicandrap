use std::env;

fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置 TEST_FOO 环境变量 (关键修改点)
    println!("cargo:TEST_FOO={}", timestamp);
    
    // 当 build.rs 变化时重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs");
}