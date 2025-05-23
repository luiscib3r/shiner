// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:app/app/internal/frb_generated.dart';
import 'package:app/app/internal/lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SettingsRepository>>
abstract class SettingsRepository implements RustOpaqueInterface {
  factory SettingsRepository({required ArcConnection db}) =>
      RustLib.instance.api.crateCoreSettingsSettingsRepositoryNew(db: db);
  ArcSettingsRepository getArc();

  Future<void> init();

  Future<Settings?> load();

  Future<void> save({required Settings settings});
}

class Settings {
  const Settings({required this.openaiApiKey});
  final String openaiApiKey;

  @override
  int get hashCode => openaiApiKey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Settings &&
          runtimeType == other.runtimeType &&
          openaiApiKey == other.openaiApiKey;
}
