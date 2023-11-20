fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("../../proto/movie.proto")?;
  tonic_build::compile_protos("../../proto/user.proto")?;
  Ok(())
}