use std::env;
use std::process::Command;

fn lookup_tool(name: &str, target: &str) -> Option<String> {
    let target_underscored = target.replace('-', "_");
    [
        format!("{name}_{target}"),
        format!("{name}_{target_underscored}"),
        format!("TARGET_{name}"),
        name.to_string(),
    ]
    .into_iter()
    .find_map(|key| env::var(&key).ok().filter(|value| !value.is_empty()))
}

fn main() {
    let target = env::var("TARGET").expect("TARGET must be set");
    let mut command = Command::new("make");
    command.arg("static").arg("-j8").current_dir("impl");

    for tool in ["CC", "AR", "STRIP"] {
        if let Some(value) = lookup_tool(tool, &target) {
            command.arg(format!("{tool}={value}"));
        }
    }

    command.status().expect("Failed to build impl");

    println!("cargo:rustc-link-search=native=impl/bin");
    println!("cargo:rustc-link-lib=static=hev-socks5-tunnel");
    println!("cargo:rustc-link-search=native=impl/third-part/hev-task-system/bin");
    println!("cargo:rustc-link-lib=static=hev-task-system");
    println!("cargo:rustc-link-search=native=impl/third-part/lwip/bin");
    println!("cargo:rustc-link-lib=static=lwip");
    println!("cargo:rustc-link-search=native=impl/third-part/yaml/bin");
    println!("cargo:rustc-link-lib=static=yaml");
}
