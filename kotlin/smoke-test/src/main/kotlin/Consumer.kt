import blueye.protocol.Attitude
import blueye.protocol.attitude
import com.google.protobuf.Any as ProtoAny

fun main() {
  val original = attitude { yaw = 90f }
  check(Attitude.parseFrom(original.toByteArray()) == original)
  check(ProtoAny.pack(original).unpack(Attitude::class.java) == original)
  check(Attitude.getDescriptor().fullName == "blueye.protocol.Attitude")
  println("Published Kotlin package: DSL, full runtime, and transitive dependencies OK")
}
