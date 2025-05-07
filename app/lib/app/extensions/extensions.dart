import 'package:flutter/material.dart';

extension BuildContextX on BuildContext {
  void unfocus() {
    FocusScope.of(this).unfocus();
  }

  void showError(String message) =>
      ScaffoldMessenger.of(this).showSnackBar(SnackBar(content: Text(message)));
}
