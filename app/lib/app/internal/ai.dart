// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:app/app/internal/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ImageGenerator>>
abstract class ImageGenerator implements RustOpaqueInterface {
  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<ImageGenerator> newInstance({required String apiKey}) =>
      RustLib.instance.api.crateAiImageGeneratorNew(apiKey: apiKey);

  Future<Uint8List> run({required String prompt});
}
