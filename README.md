# KCTRL
This is KRPC / Arduino connection program for an arduino controller
There is a build script that creates everything for communication through protobuf with KRPC and the whole protocol to communicate with the arduino, including functions to convert valuees into bytes and a parser for the reverse.
The serial protocol is established through protocol.ron (Rusty Object Notation) and the build.rs script builds the necessary steps