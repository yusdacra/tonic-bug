use tonic_build::compile_protos;

fn main() {
    compile_protos("proto/phone.proto").expect("couldnt compile protos");
}
