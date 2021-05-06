#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
TEMPLATE_ROOT=$DIR

APP_NAME=$1

mkdir $APP_NAME && cd $APP_NAME
APP_ROOT=`pwd`

# Rust
cd $APP_ROOT
cargo init --lib
cp -R $TEMPLATE_ROOT/sh $APP_ROOT/sh
cp $TEMPLATE_ROOT/rust/src/lib.rs $APP_ROOT/src/lib.rs
cp $TEMPLATE_ROOT/rust/rid_build.rs $APP_ROOT/rid_build.rs
cp $TEMPLATE_ROOT/rust/Cargo.toml $APP_ROOT/Cargo.toml
perl -pi -w -e "s/<package>/$APP_NAME/;" $APP_ROOT/Cargo.toml

# Plugin
flutter create --platforms=android,ios,macos --template=plugin $APP_ROOT/plugin

cd $APP_ROOT/plugin
rm -rf example test CHANGELOG.md README.md .idea

echo "export './generated/rid_generated.dart';" > lib/plugin.dart
cp $TEMPLATE_ROOT/flutter/plugin/ios/Classes/SwiftPlugin.swift ios/Classes/SwiftPlugin.swift
cp $TEMPLATE_ROOT/flutter/plugin/ios/plugin.podspec ios/plugin.podspec
cp $TEMPLATE_ROOT/flutter/plugin/macos/Classes/Plugin.swift macos/Classes/Plugin.swift 
cp $TEMPLATE_ROOT/flutter/plugin/macos/plugin.podspec macos/plugin.podspec
cp $TEMPLATE_ROOT/flutter/plugin/pubspec.yaml pubspec.yaml

flutter pub get

# Flutter Project

cd $APP_ROOT
flutter create --platforms=android,ios,macos $APP_ROOT

cp $TEMPLATE_ROOT/flutter/README.md $APP_ROOT/README.md
perl -pi -w -e "s/<package>/$APP_NAME/;" $APP_ROOT/README.md

cp $TEMPLATE_ROOT/flutter/pubspec.yaml $APP_ROOT/pubspec.yaml
perl -pi -w -e "s/<package>/$APP_NAME/;" $APP_ROOT/pubspec.yaml

cp $TEMPLATE_ROOT/flutter/lib/adder.dart $APP_ROOT/lib/adder.dart
cp $TEMPLATE_ROOT/flutter/lib/main.dart $APP_ROOT/lib/main.dart

echo .DS_Store   >> $APP_ROOT/.gitignore
echo .dart_tool/ >> $APP_ROOT/.gitignore
echo .packages   >> $APP_ROOT/.gitignore
echo .pub/       >> $APP_ROOT/.gitignore
echo build/      >> $APP_ROOT/.gitignore

# Build all Targets and have binding files copied and setup flutter plugin to hook things up
$APP_ROOT/sh/bindgen
$APP_ROOT/sh/ffigen

$APP_ROOT/sh/android
$APP_ROOT/sh/ios
$APP_ROOT/sh/macos

cd $APP_ROOT/plugin
flutter clean && flutter create .
rm -rf example test CHANGELOG.md README.md .idea
