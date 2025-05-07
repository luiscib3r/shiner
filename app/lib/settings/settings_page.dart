import 'package:app/settings/settings_view.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class SettingsPage extends GoRoute {
  SettingsPage()
    : super(path: _path, builder: (context, state) => const SettingsView());

  static const _path = 'settings';
  static String get fullPath => '/$_path';
  static void open(BuildContext context) => context.go(fullPath);
}
