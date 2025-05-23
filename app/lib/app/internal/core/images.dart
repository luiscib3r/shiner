// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:app/app/internal/core/job.dart';
import 'package:app/app/internal/core/pagination.dart';
import 'package:app/app/internal/frb_generated.dart';
import 'package:app/app/internal/lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `remove_images`, `save_image`, `start_image_generation`, `try_generate_image`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ImageJobRepository>>
abstract class ImageJobRepository implements RustOpaqueInterface {
  factory ImageJobRepository({
    required String appDirectory,
    required ArcConnection db,
    required ArcSettingsRepository settingsRepository,
  }) => RustLib.instance.api.crateCoreImagesImageJobRepositoryNew(
    appDirectory: appDirectory,
    db: db,
    settingsRepository: settingsRepository,
  );
  Future<void> cleanup();

  Future<PlatformInt64> createJob({required String prompt});

  Future<void> dropAll();

  Future<ImageJob?> findById({required PlatformInt64 id});

  Future<List<ImageJob>> findByStatus({
    required JobStatus status,
    required Pagination pagination,
  });

  Future<void> init();

  Stream<PlatformInt64> listen();

  Future<List<ImageJob>> load({required Pagination pagination});

  Future<void> removeJob({required PlatformInt64 id});

  Future<void> setErrorMsg({
    required PlatformInt64 id,
    required String errorMsg,
  });

  Future<void> setImagePath({
    required PlatformInt64 id,
    required String imagePath,
  });

  Future<void> updateStatus({
    required PlatformInt64 id,
    required JobStatus status,
  });
}

class ImageJob {
  const ImageJob({
    required this.id,
    required this.prompt,
    required this.status,
    this.imagePath,
    this.errMsg,
  });
  final PlatformInt64 id;
  final String prompt;
  final JobStatus status;
  final String? imagePath;
  final String? errMsg;

  @override
  int get hashCode =>
      id.hashCode ^
      prompt.hashCode ^
      status.hashCode ^
      imagePath.hashCode ^
      errMsg.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ImageJob &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          prompt == other.prompt &&
          status == other.status &&
          imagePath == other.imagePath &&
          errMsg == other.errMsg;
}
