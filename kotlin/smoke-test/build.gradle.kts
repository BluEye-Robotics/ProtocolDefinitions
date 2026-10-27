plugins {
  application
  kotlin("jvm") version "2.2.21"
}

repositories {
  maven {
    url = uri("../build/smoke-repository")
    content { includeGroup("com.blueyerobotics") }
  }
  mavenCentral()
}

dependencies {
  // Resolve the publication, including its transitive runtime dependencies.
  implementation("com.blueyerobotics:protocol-definitions:${providers.gradleProperty("packageVersion").get()}")
}

kotlin {
  jvmToolchain(17)
}

application {
  mainClass.set("ConsumerKt")
}
