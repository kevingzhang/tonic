fn main() {
    tonic_build::compile_protos("proto/docker/docker.proto").unwrap();
    tonic_build::compile_protos("proto/routeguide/route_guide.proto").unwrap();
    tonic_build::compile_protos("proto/echo/echo.proto").unwrap();
}
