import 'package:flutter/widgets.dart';

extension BuildContextX on BuildContext {
  void unfocus() {
    FocusScope.of(this).unfocus();
  }
}
