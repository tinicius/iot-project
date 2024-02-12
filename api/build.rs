fn main() {
    tonic_build::compile_protos("protos/server.proto").unwrap();
}
