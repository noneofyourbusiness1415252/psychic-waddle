#![deny(clippy::all)]
use rc_zip_sync::ReadZip;
use std::{
    error, fs, io,
    os::unix::{fs::PermissionsExt, process::CommandExt},
    process::Command,
};
fn main() -> Result<(), Box<dyn error::Error>> {
    io::copy(&mut minreq::get(format!("https://maurobalbi.gallery.vsassets.io/_apis/public/gallery/publisher/maurobalbi/extension/glas-vscode/{}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?targetPlatform=linux-x64", String::from_utf8_lossy(&minreq::post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery").with_header("Accept", "application/json;api-version=3.0-preview.1").with_header("Content-Type", "application/json").with_body(r#"{"filters":[{"criteria":[{"filterType":4,"value":"758d1193-7041-409c-a0ca-dbcc5879139c"}]}],"flags":1}"#).send_lazy()?.take(570).skip(565).map(|x| x.unwrap().0).collect::<Vec<_>>()))).send()?.into_bytes().into_iter().collect::<Vec<u8>>().read_zip()?.by_name("extension/glas").unwrap().reader(),&mut fs::File::options().write(true).create(true).truncate(true).open("/tmp/glas")?)?;
    let mut perm = fs::metadata("/tmp/glas")?.permissions();
    perm.set_mode(0o777);
    fs::set_permissions("/tmp/glas", perm)?;
    Err(Box::new(Command::new("/tmp/glas").arg("--stdio").exec()))
}
