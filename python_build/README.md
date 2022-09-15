# blueye.protocol

`blueye.protocol` is a python package generated from the Protobuf API defined in [ProtocolDefintions](https://github.com/BluEye-Robotics/ProtocolDefinitions).

It is generated with the [proto-plus library](https://proto-plus-python.readthedocs.io/en/latest/) create a more Pythonic way to interact with protocol buffers.

## Usage
The primary user of this package is the [blueye.sdk](https://github.com/blueye-robotics/blueye.sdk). The SDK configures the necessary ZeroMQ sockets and wraps everything up in an easy to use Python-object, allowing you to control the Blueye drones remotely.
