# @blueyerobotics/protocol-definitions

TypeScript protobuf definitions for Blueye Robotics protocols generated using [ts-proto](https://github.com/stephenh/ts-proto).

## Installation

```bash
npm install @blueyerobotics/protocol-definitions
```

## Usage

```ts
import { blueye } from "@blueyerobotics/protocol-definitions";

// Create a new GetBatteryReq message
const request = blueye.protocol.GetBatteryReq.create();

// Serialize the message to a Uint8Array (binary)
const binary = blueye.protocol.GetBatteryReq.encode(request).finish();

// ...

// For demonstration, we will simulate a response from the device
const response = blueye.protocol.GetBatteryRep.create({
  battery: {
    level: 85,
    voltage: 12.5,
    temperature: 25,
  },
});

const binaryResponse = blueye.protocol.GetBatteryRep.encode(response).finish();

// Decode a binary response back into a message
const decoded = blueye.protocol.GetBatteryRep.decode(binaryResponse);

// Access fields
console.log(decoded.battery?.level);
```
