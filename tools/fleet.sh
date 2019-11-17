 #!/bin/bash

 set -ex

 if [[ $# -ne 1 ]]; then
     echo "Pass number of devices"
     echo "Usage: ./createvehicle 10"
     exit
 fi

 count=$1

 for i in $(seq 1 $count)
 do
     RUST_LOG=rumqtt,uplink=debug cargo run -- -c ../config/uplink.toml -i device-$i -a ../certs/ &
 done

 sleep 1000
