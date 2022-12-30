# Remote Operation Service Element (ROSE) Transport Abstraction

This library defines an abstraction for the transport of Remote Operation
Service Element (ROSE) PDUs. This was defined so that
[ITU Recommendation X.519 (2019)](https://www.itu.int/rec/T-REC-X.519/en)'s
Internet Directly-Mapped (IDM) protocol, ISO Transport over TCP (ITOT),
Lightweight Presentation Protocol (LPP), Transport Protocol Class 4 over IP
(TP4), and other network stacks could be used to transmit and receive ROSE PDUs.
