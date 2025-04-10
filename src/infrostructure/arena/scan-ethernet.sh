#!/bin/bash

printf '%10s %32s %32s\n' interface ipaddress macaddress
printf '%s\n' '----------------------------------------------------------------------------'
for each in $(ip address | grep -oP '(^[\d]+:\s)\K[\d\w]+'); do
  mac=$(ip address show ${each} | grep -oP '(?<=link/ether\s)\K[\da-f:]+|(?<=link/loopback\s)\K[\da-f:]+')
  for address in $(ip address show ${each} | grep -oP '(?<=inet\s)\K[\d.]+|(?<=inet6\s)\K[\da-f:]+'); do
    printf '%10s %32s %32s\n' ${each} ${address} ${mac}
  done
done
