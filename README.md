# Oh No!

[![Tests](https://github.com/Vandesm14/oh-no/actions/workflows/tests.yml/badge.svg)](https://github.com/Vandesm14/oh-no/actions/workflows/tests.yml)

[![Checks](https://github.com/Vandesm14/oh-no/actions/workflows/check.yml/badge.svg)](https://github.com/Vandesm14/oh-no/actions/workflows/check.yml)

Simulate devices on a virtual network and see how they interact with each other.

## Abstract

Oh No is a graph-based networking simulator built in Rust. It provides a way of testing infrastructure handling and digital topology.

### Device

A device is a node on the graph. Devices have an update function that runs every tick. At the beginning of a tick, they are given a list of messages that they have received, and can send messages at the end of a tick.

### Interface

Devices are connected by interfaces (edges). There is a one-to-one mapping between interfaces and devices. To connect multiple devices together, one can use a prebuilt Hub device.
