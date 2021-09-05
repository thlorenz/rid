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

cp $TEMPLATE_ROOT/flutter/lib/main.dart $APP_ROOT/lib/main.dart

# gitignore
cp $TEMPLATE_ROOT/gitignore $APP_ROOT/.gitignore

# Build all Targets that are most likely supported on the host OS
# and have binding files copied and setup flutter plugin to hook things up
$APP_ROOT/sh/bindgen

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # Linux
  $APP_ROOT/sh/linux
elif [[ "$OSTYPE" == "darwin"* ]]; then
  # Mac OSX
  $APP_ROOT/sh/ios
  $APP_ROOT/sh/macos
fi

# Android builds are supported as long as the cargo ndk and the supporting Android sdk are
# installed
if command -v cargo-ndk &> /dev/null
then
  $APP_ROOT/sh/android
else
  echo "Not initializing Android build since the cargo-ndk dependency was not found."
  echo "Install it following instructions here:"
  echo "    https://github.com/bbqsrc/cargo-ndk"
  echo
  echo "Install the Android NDK following one of the below instructions:"
  echo "    https://developers.google.com/ar/develop/c/quickstart"
  echo "    https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html"
fi

cd $APP_ROOT/plugin
flutter clean && flutter create .
rm -rf plugin.dart example test CHANGELOG.md README.md .idea

cd $APP_ROOT
flutter pub get

$APP_ROOT/sh/bindgen
