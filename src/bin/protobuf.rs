fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos") // 输出路径
        .inputs(&["src/protos/src/prototype.proto"]) // 输入文件
        .include("src/protos/src") // 添加一个包含的目录
        .run()
        .expect("Codegen failed.");
}