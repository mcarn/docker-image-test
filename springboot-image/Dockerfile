FROM eclipse-temurin:17-jre-focal

ARG JAR_FILE=target/app.jar

WORKDIR /opt/app

COPY ${JAR_FILE} app.jar

ENTRYPOINT ["java","-jar","app.jar"]