use std::io::Read;
use std::process::Command;
use reqwest::blocking::Client;
use zip::read::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut v = String::new();
    Client::new().post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
        .header("Accept", "application/json;api-version=3.0-preview.1")
        .header("Content-Type", "application/json")
        .body(r#"{"filters":[{"criteria":[{"filterType":4,"value":"758d1193-7041-409c-a0ca-dbcc5879139c"}]}],"flags":1}"#)
        .send()?.take(570).read_to_string(&mut v)?;
    let v = &v[565..570];
    std::io::copy(&mut ZipArchive::new(std::io::Cursor::new(client.get(&format!("https://marketplace.visualstudio.com/_apis/public/gallery/publishers/maurobalbi/vsextensions/glas-vscode/{v}/vspackage?targetPlatform=linux-x64")).send()?.bytes()?))?.archive.by_name("extension/glas")?, "/dev/shm/glas")?;
    println!("{}", std::str::from_utf8(&Command::new("/dev/shm/glas")
        .output()?.stdout)?);

    Ok(())
}
