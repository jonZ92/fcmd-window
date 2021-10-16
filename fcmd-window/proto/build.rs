
extern crate protoc_rust;

use protoc_rust::Customize;


// 编译 rust protobuf
fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["src/fileBodys.proto"],
        includes: &["src"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
