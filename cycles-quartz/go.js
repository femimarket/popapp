  // const a = {
//     "base": {"account": "pop"},
//     "module": {
//         "quartz": {
//             "msg": {
//                 "config": {
//                     "mr_enclave": "",
//                     "epoch_duration": {"secs": 5, "nanos": 0},
//                     "light_client_opts": {
//                         "chain_id":"pion-1",
//                         "trusted_height":0,
//                         "trusted_hash":"",
//                         "trusting_period":0,
//                         "max_clock_drift":0,
//                         "max_block_lag":0,
//                         "trust_threshold":[0,0]
//                     }
//                 }
//             },
//             "attestation":{}
//         }
//     }
// }
//
`
quartz --mock-sgx contract deploy --contract-manifest "/Users/u/pop/contracts/pop/Cargo.toml" --init-msg '{"base":{"account":"pop"},"module":{"quartz":{"msg":{"config":{"mr_enclave":"","epoch_duration":{"secs":5,"nanos":0},"light_client_opts":{"chain_id":"pion-1","trusted_height":0,"trusted_hash":"","trusting_period":0,"max_clock_drift":0,"max_block_lag":0,"trust_threshold":[0,0]}}},"attestation":"7154ae5d304082a2b264d67660645323c6a4ef7af563db36f6ffd80bd10784830000000000000000000000000000000000000000000000000000000000000000"}}}'
`

let hex = Buffer.from( [113, 84, 174, 93, 48, 64, 130, 162, 178, 100, 214, 118, 96, 100, 83, 35, 198, 164, 239, 122, 245, 99, 219, 54, 246, 255, 216, 11, 209, 7, 132, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).toString("hex")
console.log(hex)
