There are some setting you can apply on your system that may help with
this. Try running these commands in Terminal:

Find the name of the camera's ethernet interface:     ifconfig -a

will be something like "eth1"

replace [device] in the following commands with this name

Enable jumbo frames:         sudo ifconfig [device] mtu 9000

Increase receive buffer size:       sudo ethtool -g [device]     (to find max RX)
                                                    sudo ethtool -G [device] rx [max RX]
Increase socket buffer size:     sudo sh -c "echo 'net.core.rmem_default=1048576' >> /etc/sysctl.conf"
                                                 sudo sh -c "echo 'net.core.rmem_max=1048576' >> /etc/sysctl.conf"
                                                 sudo sysctl -p
increase to 32MB if needed by changing the number in the commands to 33554432

Set link speed, duplex, and auto-negotiation:      ethtool [device]       (see what max link speed is supported)
                                                                             sudo ethtool –s [device] speed [link speed] duplex full autoneg on

[link speed] will be 1000 for a 1GBps interface, 5000 for 5GBps, etc.

Enable IPv4, TCP, and UDP Offloading:    sudo ethtool --offload [device] rx on tx on

Enable adaptive interrupt moderation:    sudo ethtool -C [device] adaptive-tx on adaptive-rx on

this doesn't work on all systems, but it's worth trying