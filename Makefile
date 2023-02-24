JAVAC 	:= javac
JAVACFLAGS := -d testy/build --patch-module java.base=./
JAR		:= jar

SOURCEDIR := testy

OUT		:= test.jar
MAIN	:= com.vastatrix.tests.Main

SOURCES := $(shell find $(SOURCEDIR) -name '*.java')
CLASSES := $(patsubst $(SOURCEDIR)/%.java, $(SOURCEDIR)/%.class, ${SOURCES})

all: clean $(OUT)

$(OUT): $(CLASSES)
	$(JAR) cvfe $(OUT) $(MAIN) testy/build/*

%.class: %.java
	$(JAVAC) $(JAVACFLAGS) $<

clean: 
	rm -rf testy/build/*