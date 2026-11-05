import com.google.protobuf.gradle.*
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
  `java-library`
  `maven-publish`
  kotlin("jvm") version "2.2.21"
  id("com.google.protobuf") version "0.9.5"
}

group = "com.blueyerobotics"
val baseVersion = file("../version.txt").readText().trim()
version = providers.gradleProperty("packageVersion").getOrElse("$baseVersion.0-local")

val protocVersion = file("../protobuf.version").readText().trim()
val protobufVersion = "4.$protocVersion"

repositories {
  mavenCentral()
}

java {
  toolchain.languageVersion.set(JavaLanguageVersion.of(17))
  sourceCompatibility = JavaVersion.VERSION_1_8
  targetCompatibility = JavaVersion.VERSION_1_8
  withSourcesJar()
}

kotlin {
  compilerOptions.jvmTarget.set(JvmTarget.JVM_1_8)
}

tasks.withType<JavaCompile>().configureEach {
  options.release.set(8)
}

dependencies {
  // Public APIs expose the full Java message classes and Kotlin DSL types.
  api("com.google.protobuf:protobuf-java:$protobufVersion")
  api("com.google.protobuf:protobuf-kotlin:$protobufVersion")
  testImplementation(kotlin("test-junit5"))
  testImplementation("com.google.protobuf:protobuf-java-util:$protobufVersion")
  testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.11.4")
  testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.11.4")
}

sourceSets {
  main {
    proto {
      setSrcDirs(listOf("../protobuf_definitions"))
    }
  }
}

protobuf {
  protoc {
    artifact = "com.google.protobuf:protoc:$protobufVersion"
  }
  generateProtoTasks {
    all().configureEach {
      builtins {
        // The Java builtin is enabled by default. Kotlin builds on that output.
        id("kotlin")
      }
    }
  }
}

tasks.test {
  useJUnitPlatform()
}

publishing {
  publications {
    register<MavenPublication>("protocol") {
      from(components["java"])
      artifactId = "protocol-definitions"
      pom {
        name.set("Blueye Protocol Definitions")
        description.set("Protocol Buffer messages and Kotlin DSLs for Blueye underwater drones.")
        url.set("https://github.com/BluEye-Robotics/ProtocolDefinitions")
        licenses {
          license {
            name.set("GNU Lesser General Public License v3.0 only")
            url.set("https://www.gnu.org/licenses/lgpl-3.0.html")
          }
        }
        scm {
          url.set("https://github.com/BluEye-Robotics/ProtocolDefinitions")
          connection.set("scm:git:https://github.com/BluEye-Robotics/ProtocolDefinitions.git")
        }
      }
    }
  }
  repositories {
    maven {
      name = "GitHubPackages"
      url = uri("https://maven.pkg.github.com/blueye-robotics/ProtocolDefinitions")
      credentials {
        username = providers.gradleProperty("gpr.user").orNull ?: System.getenv("GITHUB_ACTOR")
        password = providers.gradleProperty("gpr.key").orNull ?: System.getenv("GITHUB_TOKEN")
      }
    }
    // Exercise the published artifact without credentials or remote writes.
    maven {
      name = "SmokeTest"
      url = uri(layout.buildDirectory.dir("smoke-repository"))
    }
  }
}
