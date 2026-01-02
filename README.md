# rust-lldp
Aims to be a CLI interface to parse, interact with, and inject LLDP protocol traffic on the network.
  
## About LLDP   

LLDP (Link Layer Discovery Protocol) is a vendor-neutral Layer 2 protocol used to discover
devices that are directly connected on the same Ethernet link. It enables network devices
to advertise and learn information about their neighbors, such as system name, software
version, device capabilities (e.g., switch, router, or both), and interface details.

LLDP provides a standardized mechanism for topology discovery by allowing each device
to periodically exchange discovery information with its immediate neighbors, making it
possible to infer the physical and logical structure of a Layer 2 network.
