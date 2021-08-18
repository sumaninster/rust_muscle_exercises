fn main() {
    tonic_build::compile_protos("proto/grpc_json/muscle_exercises_json.proto").unwrap();
    tonic_build::compile_protos("proto/grpc_object/muscle_exercises_object.proto").unwrap();
}