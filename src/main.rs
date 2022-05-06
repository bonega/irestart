use std::fmt::format;

use color_eyre::eyre::{eyre, Context};
use dll_syringe::{process::OwnedProcess, Syringe};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let process_name = "isaac-ng";
    let target_process = OwnedProcess::find_first_by_name(process_name)
        .ok_or_else(|| eyre!("Couldn't find process named: {process_name}"))?;
    let syringe = Syringe::for_process(target_process);

    let payload_path = "./target/i686-pc-windows-msvc/debug/irestart.dll";
    let _injected_payload = syringe
        .inject(payload_path)
        .wrap_err(format!("Couldn't inject payload into process {process_name}"))?;

    Ok(())
}
