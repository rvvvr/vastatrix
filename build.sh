#!/bin/fish
rm -rf ./testy/build
rm test.jar
cd testy
mkdir build
javac -d ./build --patch-module java.base=./ **/*.java
cd build
jar cvfe test.jar com.vastatrix.tests.Main *
mv test.jar ../..
cd ../../vtx-std
cargo build


