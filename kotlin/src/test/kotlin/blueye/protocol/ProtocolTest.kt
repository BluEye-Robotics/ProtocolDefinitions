package blueye.protocol

import com.google.protobuf.Any as ProtoAny
import com.google.protobuf.DynamicMessage
import com.google.protobuf.InvalidProtocolBufferException
import com.google.protobuf.TypeRegistry
import com.google.protobuf.timestamp
import com.google.protobuf.util.JsonFormat
import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

class ProtocolTest {
  @Test
  fun attitudeMatchesWireFixture() {
    val original = attitude {
      roll = 1f
      pitch = 2f
      yaw = 3f
    }
    // The same fixed32 wire fixture is checked by the Swift client tests.
    val expected = byteArrayOf(
      0x0d, 0x00, 0x00, 0x80.toByte(), 0x3f,
      0x15, 0x00, 0x00, 0x00, 0x40,
      0x1d, 0x00, 0x00, 0x40, 0x40,
    )
    assertContentEquals(expected, original.toByteArray())
    assertEquals(original, Attitude.parseFrom(expected))
  }

  @Test
  fun anyAndTimestampRoundTrip() {
    val original = attitude { yaw = 90f }
    val record = binlogRecord {
      payload = ProtoAny.pack(original)
      unixTimestamp = timestamp {
        seconds = 1_700_000_000
        nanos = 123_456_789
      }
    }
    val decoded = BinlogRecord.parseFrom(record.toByteArray())
    assertEquals(record, decoded)
    assertEquals("type.googleapis.com/blueye.protocol.Attitude", decoded.payload.typeUrl)
    assertEquals(original, decoded.payload.unpack(Attitude::class.java))
  }

  @Test
  fun fullRuntimeSupportsReflectionAndAnyJson() {
    val original = attitude { roll = 1.5f }
    val descriptor = Attitude.getDescriptor()
    assertEquals("blueye.protocol.Attitude", descriptor.fullName)
    val dynamic = DynamicMessage.parseFrom(descriptor, original.toByteArray())
    assertEquals(1.5f, dynamic.getField(descriptor.findFieldByName("roll")))

    val registry = TypeRegistry.newBuilder().add(descriptor).build()
    val packed = ProtoAny.pack(original)
    val json = JsonFormat.printer().usingTypeRegistry(registry).print(packed)
    val decoded = ProtoAny.newBuilder()
    JsonFormat.parser().usingTypeRegistry(registry).merge(json, decoded)
    assertEquals(packed, decoded.build())
  }

  @Test
  fun missionOneofRoundTrip() {
    val original = mission {
      id = 42
      name = "Inspection"
      instructions += instruction {
        id = 1
        depthSetPointCommand = depthSetPointCommand {
          depthSetPoint = depthSetPoint { depth = 10f }
        }
      }
    }
    val decoded = Mission.parseFrom(original.toByteArray())
    assertEquals(original, decoded)
    assertTrue(decoded.getInstructions(0).hasDepthSetPointCommand())
  }

  @Test
  fun unknownFieldsSurviveRoundTrip() {
    val bytes = byteArrayOf(0xa0.toByte(), 0x06, 0x07) // Field 100, varint 7.
    assertContentEquals(bytes, Attitude.parseFrom(bytes).toByteArray())
    assertFailsWith<InvalidProtocolBufferException> {
      Attitude.parseFrom(byteArrayOf(0x0d))
    }
  }
}
