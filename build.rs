use protobuf_codegen::Codegen;
fn main() {
    // 创建新的目录来存放proto生成的rs文件
    //std::fs::create_dir_all("./bin/proto_demo/protos").unwrap();
    Codegen::new()
        .out_dir("./bin/proto_demo")
        .inputs(&["./bin/proto_demo/proto/tutorial.proto"])
        .include("./bin/proto_demo/proto")
        .run()
        .expect("Codegen failed.");
}
