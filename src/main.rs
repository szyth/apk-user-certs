use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;

fn main() {
    // Get the APK file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: apk-user-certs <APK file>");
        return;
    }
    let apk_file = &args[1];

    // Get the base name of the APK file (without the .apk extension)
    let base_name = apk_file.split(".apk").next().unwrap();

    // Run the apktool command to decompile the APK
    let output = Command::new("apktool")
        .arg("d")
        .arg(&apk_file)
        .arg("-o")
        .arg(&format!("{}_decompiled", base_name))
        .output()
        .expect("Failed to run apktool");

    // Check if the command was successful
    if output.status.success() {
        println!("APK decompiled successfully");

        // Create the network_security_config.xml file
        let mut file = File::create(&format!(
            "{}_decompiled/res/xml/network_security_config.xml",
            base_name
        ))
        .expect("Failed to create network_security_config.xml file");

        // Write the XML content to the file
        let xml_content = "<network-security-config>
                                <base-config>
                                    <trust-anchors>
                                        <!-- Trust preinstalled CAs -->
                                        <certificates src=\"system\" />
                                        <!-- Additionally trust user added CAs -->
                                        <certificates src=\"user\" />
                                    </trust-anchors>
                                </base-config>
                            </network-security-config>";
        file.write_all(xml_content.as_bytes())
            .expect("Failed to write XML content to file");

        println!("network_security_config.xml file created successfully");

        // Update the AndroidManifest.xml file
        let mut manifest_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&format!("{}_decompiled/AndroidManifest.xml", base_name))
            .expect("Failed to open AndroidManifest.xml file");

        let mut manifest_content = String::new();
        manifest_file
            .read_to_string(&mut manifest_content)
            .expect("Failed to read AndroidManifest.xml file");

        // Replace the android:allowBackup attribute with android:allowBackup="true" and add android:networkSecurityConfig attribute
        let updated_manifest_content = manifest_content.replace(r#"android:allowBackup"#, r#"android:allowBackup="true" android:networkSecurityConfig="@xml/network_security_config""#);
        manifest_file
            .set_len(0)
            .expect("Failed to truncate AndroidManifest.xml file");
        manifest_file
            .write_all(updated_manifest_content.as_bytes())
            .expect("Failed to write updated AndroidManifest.xml content to file");

        println!("AndroidManifest.xml file updated successfully");

        // Run the apktool command to reassemble the APK
        let output = Command::new("apktool")
            .arg("b")
            .arg(&format!("{}_decompiled", base_name))
            .arg("-o")
            .arg(&format!("{}_recompiled.apk", base_name))
            .output()
            .expect("Failed to run apktool");

        // Check if the command was successful
        if output.status.success() {
            println!("APK reassembled successfully");
        } else {
            println!("Failed to reassemble APK");
        }
    } else {
        println!("Failed to decompile APK");
    }
}
