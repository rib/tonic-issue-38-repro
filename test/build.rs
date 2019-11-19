use std::env;
use ini::Ini;
use std::path::Path;

fn compile_client_protos(protos: &[&Path], includes: &[&Path]) -> Result<(), Box<dyn std::error::Error>>
{
    for proto in protos.iter() {
        // See: https://github.com/rust-lang/cargo/issues/4587
        println!("cargo:rerun-if-changed={}", proto.to_str().unwrap());
    }
    tonic_build::configure()
        .build_server(false)
        .compile(protos, includes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir_env = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR evvironment variable unset");
    let top_dir = Path::new(&manifest_dir_env).join("../");

    let conf_path = Path::new(&top_dir).join("environment.ini");
    let conf = Ini::load_from_file(&conf_path).expect("Failed to load config from environment.ini");
    let paths_section = conf.section(Some("paths".to_owned())).expect("Didn't find [paths] section in environment.ini");

    let openmatch_config = paths_section.get("openmatch").expect("[paths] openmatch= not set in environment.ini");
    let openmatch_dir = Path::new(&top_dir).join(openmatch_config);
    let openmatch_proto_include_dir = Path::new(&openmatch_dir).join("third_party");
    let openmatch_proto_dir = Path::new(&openmatch_dir).join("api");
    let openmatch_backend_proto_path = Path::new(&openmatch_proto_dir).join("backend.proto");
    let openmatch_frontend_proto_path = Path::new(&openmatch_proto_dir).join("frontend.proto");
    compile_client_protos(&[&openmatch_backend_proto_path,
                            &openmatch_frontend_proto_path
                           ],
                          &[&openmatch_dir,
                            &openmatch_proto_include_dir])?;

    Ok(())
}
