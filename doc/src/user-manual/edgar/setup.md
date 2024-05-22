# Setup

- Download the opendut-cleo binary for your target from the openDuT GitHub project: https://github.com/eclipse-opendut/opendut/releases
- Unpack the archive on your target system.

- EDGAR comes with a scripted setup, which you can initiate by running:  
```shell
opendut-edgar setup managed <SETUP-STRING>
```  
You can get the `<SETUP-STRING>` from LEA after creating a Peer.

This will configure your operating system and start the *EDGAR Service*, which will receive its configuration from *CARL*.

## Download EDGAR from CARL
You can download the EDGAR binary as tarball from one of CARLs endpoints.

The archive can be requested at `https://{CARL-HOST}/api/edgar/{architecture}/download`.

Available architectures are:
- x86_64-unknown-linux-gnu
- armv7-unknown-linux-gnueabihf
- aarch64-unknown-linux-gnu

Once downloaded, extract the files with the command `tar xvf opendut-edgar-{architecture}.tar.gz`. It will then be extracted into
the folder which is the current work directory. You might want to use another directory of your choice.
