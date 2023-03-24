//! This binary will emit a .d.ts file that defines interfaces
//! for the types that this crate defines.
//! 

use schemars::schema_for;
use scrab_public_types::*;
use std::io::Write;
use std::process::{Command, Stdio};
fn main() {
    let npx_name = if cfg!(windows) { "npx.cmd" } else { "npx" };

    let output_dir = std::env::current_dir().unwrap();

    //
    // This lists the types that should be emited
    //
    let types_to_export = [schema_for!(PublicRoom), schema_for!(PublicWorld)];

    if let Ok(mut out_file) = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_dir.join("scrab_frontend_types.d.ts"))
    {
        {
            let mut proc = Command::new(npx_name)
                .args(["json-schema-to-typescript"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to run `npx json-schema-to-typescript`");
            proc.stdin.take().unwrap().write_all(b"{}").unwrap();
            let output = &proc.wait_with_output().unwrap().stdout;
            out_file
                .write_all(output)
                .expect("failed to write to types.d.ts");
        }

        for schema in types_to_export {
            let mut proc = Command::new(npx_name)
                .args(["json-schema-to-typescript", "--no-bannerComment"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Ts types failed to generate");

            let mut stdin = proc.stdin.take().unwrap();
            std::thread::spawn(move || {
                stdin
                    .write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())
                    .unwrap();
            });

            let output = &proc
                .wait_with_output()
                .expect("failed to read output")
                .stdout;

            out_file
                .write_all(output)
                .expect("failed to write to types.d.ts");
        }
    }
}
