JAVAC 	:= javac
JAVACFLAGS := -d testy/build --patch-module java.base=./
JAR		:= jar

SOURCEDIR := testy

OUT		:= test.jar
MAIN	:= com.vastatrix.tests.Main

SOURCES := $(shell find $(SOURCEDIR) -name '*.java')
CLASSES := $(patsubst $(SOURCEDIR)/%.java, $(SOURCEDIR)/%.class, ${SOURCES})

all: clean $(OUT) test

test:
	cargo run -- --jar $(OUT)

$(OUT): $(CLASSES)
	cd testy/build && $(JAR) cvfe $(OUT) $(MAIN) *
	mv testy/build/$(OUT) ./$(OUT)

%.class: %.java
	$(JAVAC) $(JAVACFLAGS) $<

clean: 
	rm -rf testy/build/*