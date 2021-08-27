use std::env;

const LINUX_LLVM_PATHS: [&str; 6] = [
    "/usr/lib/llvm-6.0/lib/libclang.so",
    "/usr/lib/llvm-9/lib/libclang.so",
    "/usr/lib/llvm-10/lib/libclang.so",
    "/usr/lib/llvm-11/lib/libclang.so",
    "/usr/lib/libclang.so",
    "/usr/lib64/libclang.so",
];

const MACOS_LLVM_PATHS: [&str; 1] = ["/usr/local/opt/llvm/lib/"];
const WINDOWS_LLVM_PATHS: [&str; 1] = [r#"C:\Program Files\LLVM\bin\"#];

pub struct HostProps {
    // See https://github.com/dart-lang/ffigen/blob/6e10689c0e1a510f47d2e81540678771bf560250/lib/src/strings.dart#L158-L170
    pub llvm_paths: Vec<&'static str>,
}

impl HostProps {
    pub fn new() -> Self {
        // NOTE: add more here once we find out about them
        match env::consts::OS {
            "linux" => Self {
                llvm_paths: LINUX_LLVM_PATHS.to_vec(),
            },
            "macos" => Self {
                llvm_paths: MACOS_LLVM_PATHS.to_vec(),
            },
            "windows" => Self {
                llvm_paths: WINDOWS_LLVM_PATHS.to_vec(),
            },
            "android" | "ios" | "freebsd" | "dragonfly" | "netbsd"
            | "openbsd" | "solaris" | _ => {
                panic!("rid cli cannot run on {}", env::consts::OS)
            }
        }
    }
}
