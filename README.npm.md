# @blueyerobotics/protocol-definitions

TypeScript protobuf definitions for Blueye Robotics protocols generated using [ts-proto](https://github.com/stephenh/ts-proto).

## Installation

```bash
npm install @blueyerobotics/protocol-definitions
```

## Module format

This package ships ES modules only — there is no separate CommonJS build. Both
`import` and `require()` work on current Node.js releases.

```ts
import { blueye } from "@blueyerobotics/protocol-definitions";
```

`require()` is supported on Node.js 22.12+ (or 20.19+), which can load an ES
module from CommonJS:

```js
const { blueye } = require("@blueyerobotics/protocol-definitions");
```

On older Node.js versions `require()` fails with `ERR_REQUIRE_ESM` — use `import`
or a dynamic `await import()` instead.

The package root is the supported entry point. Individual generated modules stay
reachable under `./dist/` (for example
`@blueyerobotics/protocol-definitions/dist/telemetry.js`) if you want to import a
single protocol file to keep bundles small.

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
