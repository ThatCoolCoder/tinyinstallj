extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        // Require admin priveliges to run the main program
        let mut res = winres::WindowsResource::new();
        res.set_manifest(r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="true" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
            </assembly>
        "#);
        res.compile().unwrap();
    }
}