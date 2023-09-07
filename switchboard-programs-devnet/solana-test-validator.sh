solana-test-validator -r \
    --bpf-program \
        SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f \
            switchboard.so \
    --bpf-program \
        sbattyXrzedoNATfc4L31wC9Mhxsi1BmFhTiN8gDshx \
            attestation-program.so \
    --account \
        Fi8vncGpNKbq62gPo56G4toCehWNy77GgqGkTaAF5Lkk \
            devnet-idl.json \
    --account \
        CyZuD7RPDcrqCGbNvLCyqk6Py9cEZTKmNKujfPi3ynDd \
            sbState.json \
    --account \
        7hkp1xfPBcD2t1vZMoWWQPzipHVcXeLAAaiGXdPSfDie \
            tokenVaulr.json \
    --account \
        5ExuoQR69trmKQfB95fDsUGsUrrChbGq9PFgt8qouncz \
            attestation-idl.json \
    --account \
        5MFs7RGTjLi1wtKNBFRtuLipCkkjs4YQwRRU9sjnbQbS \
            attestation-state.json
