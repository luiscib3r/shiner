// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:app/app/internal/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class Pagination {
  factory Pagination() =>
      RustLib.instance.api.crateCorePaginationPaginationNew();

  const Pagination.raw({required this.limit, required this.offset});
  final PlatformInt64 limit;
  final PlatformInt64 offset;

  Pagination setLimit({required PlatformInt64 limit}) => RustLib.instance.api
      .crateCorePaginationPaginationSetLimit(that: this, limit: limit);

  Pagination setOffset({required PlatformInt64 offset}) => RustLib.instance.api
      .crateCorePaginationPaginationSetOffset(that: this, offset: offset);

  @override
  int get hashCode => limit.hashCode ^ offset.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Pagination &&
          runtimeType == other.runtimeType &&
          limit == other.limit &&
          offset == other.offset;
}
