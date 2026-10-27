# Kotlin/JVM package

Maven coordinate: `com.blueyerobotics:protocol-definitions`.

The library generates Java message classes and Kotlin builder DSLs from the
shared schemas. It uses the **full** `protobuf-java` and `protobuf-kotlin` runtimes,
including descriptors, reflection, and `Any`. It targets Java 8 bytecode, uses
Kotlin 2.2.21, and builds with JDK 17 and the included Gradle 8.14.3 wrapper.
It is a JVM/Android library, not a Kotlin Multiplatform library, and contains no
network transport implementation.

## Consume

Add the repository to `dependencyResolutionManagement.repositories` in your
`settings.gradle.kts` (or your project's `repositories` block):

```kotlin
maven {
  url = uri("https://maven.pkg.github.com/blueye-robotics/ProtocolDefinitions")
  credentials {
    username = providers.gradleProperty("gpr.user").orNull ?: System.getenv("GITHUB_ACTOR")
    password = providers.gradleProperty("gpr.key").orNull ?: System.getenv("GITHUB_TOKEN")
  }
  content { includeGroup("com.blueyerobotics") }
}
mavenCentral()
```

Store `gpr.user` and `gpr.key` in your user-level `~/.gradle/gradle.properties`,
not in source control. Use a GitHub personal access token (classic) with
`read:packages` and access to the repository. In Actions, a workflow's
`GITHUB_TOKEN` can be used where it has the required package access.

After the first release is published, use its full version from the package workflow
(the run number below is an example):

```kotlin
dependencies {
  implementation("com.blueyerobotics:protocol-definitions:6.5.815")
}
```

Both Protobuf runtimes are transitive API dependencies. Consumers do not need a
Protobuf Gradle plugin or `protoc` installation.

```kotlin
import blueye.protocol.Attitude
import blueye.protocol.attitude
import com.google.protobuf.Any as ProtoAny

val original = attitude { yaw = 90f }
val decoded = Attitude.parseFrom(original.toByteArray())
val packed = ProtoAny.pack(original)
val unpacked = packed.unpack(Attitude::class.java)
val descriptor = Attitude.getDescriptor()
```

Java classes live directly in `blueye.protocol` because the schemas enable
`java_multiple_files`. This does not change protobuf wire names or field numbers.
For Protobuf JSON, add `com.google.protobuf:protobuf-java-util` at the same runtime
version (currently `4.36.2`); tests exercise JSON with an `Any` type registry.

## Develop and test

From `kotlin/`:

```sh
./gradlew build
./gradlew publishProtocolPublicationToSmokeTestRepository -PpackageVersion=6.5.0-local
./gradlew -p smoke-test run -PpackageVersion=6.5.0-local
```

The Gradle build downloads the matching `protoc` artifact and runtimes using the
version in `../protobuf.version`. Java and Kotlin sources are generated only in
the build directory. The separate smoke-test project resolves the actual local
Maven publication to verify the JAR, Kotlin DSL, and transitive dependencies.
The publication includes a sources JAR, POM, and Gradle module metadata.

## Publishing

`ci-kotlin.yaml` is called by the **Protocol packages** workflow in
`ci-dotnet.yaml`. The caller supplies the same version to NuGet, Maven, and Swift;
this reusable workflow does not calculate its own build number. Builds and tests
run on pushes and pull requests. Upstream branch pushes publish to GitHub Packages
using `GITHUB_TOKEN` with `packages: write`; pull requests and forks do not publish.

The shared `version.txt` contains the Blueye App major/minor version, currently
`6.5`. Master publishes stable `6.5.<run-number>` versions automatically. Other
branches publish `6.5.<run-number>-branch-<sanitized-branch>`. Local builds default
to `6.5.0-local`. The run counter belongs to `ci-dotnet.yaml`, so merges to master
do not require a version-file edit. NuGet and Maven publish first, then the same
version and source SHA are dispatched to Swift for generation and tagging.

Published versions are immutable. A workflow rerun keeps its original version;
prefer rerunning failed jobs so successful publications are not repeated.

To publish manually after testing, set credentials as above and run
`./gradlew publishProtocolPublicationToGitHubPackagesRepository -PpackageVersion=<version>`.
The generic `publish` task also writes to the local smoke-test repository.
