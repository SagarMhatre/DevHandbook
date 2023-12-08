
/src/main/resources/logback-spring.xml

```xml
<configuration>

    <!-- Define file appenders -->

    <appender name="STDOUT" class="ch.qos.logback.core.ConsoleAppender">
        <encoder>
            <pattern>%d{yyyy-MM-dd HH:mm:ss.SSS} [%thread] %-5level %logger{36} - %msg%n</pattern>
        </encoder>
    </appender>

    <appender name="FILE" class="ch.qos.logback.core.rolling.RollingFileAppender">
        <file>application.log</file>
        <encoder>
            <pattern>%date [%thread] %-5level %logger{36} - %msg%n</pattern>
        </encoder>
        <rollingPolicy class="ch.qos.logback.core.rolling.TimeBasedRollingPolicy">
            <fileNamePattern>application.%d{yyyy-MM-dd}.log</fileNamePattern>
        </rollingPolicy>
    </appender>

    <appender name="SPECIAL_FILE" class="ch.qos.logback.core.rolling.RollingFileAppender">
        <file>special.log</file>
        <encoder>
            <pattern>%date [%thread] %-5level %logger{36} - %msg%n</pattern>
        </encoder>
        <rollingPolicy class="ch.qos.logback.core.rolling.TimeBasedRollingPolicy">
            <fileNamePattern>special.%d{yyyy-MM-dd}.log</fileNamePattern>
        </rollingPolicy>
    </appender>

    <!-- Loggers -->

    <logger name="com.mycompany.SpecialClass" level="TRACE" additivity="false">
        <appender-ref ref="SPECIAL_FILE" />
    </logger>

    <logger name="com.mycompany" level="TRACE" additivity="false">
        <appender-ref ref="FILE" />
    </logger>

    <root level="ERROR">
        <appender-ref ref="STDOUT" />
    </root>

    

</configuration>

```

pom.xml
```xml
<!-- https://mvnrepository.com/artifact/org.projectlombok/lombok -->
<dependency>
    <groupId>org.projectlombok</groupId>
    <artifactId>lombok</artifactId>
    <version>1.18.30</version>
    <scope>provided</scope>
</dependency>

```

```java
import lombok.extern.slf4j.Slf4j;
//import org.slf4j.Logger;
//import org.slf4j.LoggerFactory;

@Slf4j
public class MyClass {
    public void someMethod() {
        log.info("This log message will go to the appropriate log file.");
    }
}

```