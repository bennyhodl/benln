fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::path::PathBuf::from("../proto");

    let protos = ["benln.proto"];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let path = dir.join(proto);
            path
        })
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .type_attribute(
            "GetNodeInfoResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "NewAddressResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "SignMessageResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "GetTotalOnchainBalanceResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "ListPeersResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute("Peer", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&proto_paths, &[dir])?;

    Ok(())
}
