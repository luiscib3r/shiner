import 'package:app/generation/generation.dart';
import 'package:app/home/home.dart';
import 'package:app/settings/settings.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class HomePage extends GoRoute {
  HomePage()
    : super(
        path: fullPath,
        builder: (context, state) => const HomeView(),
        routes: [SettingsPage(), GenerationPage()],
      );

  static const fullPath = '/';
  static void open(BuildContext context) => context.go(fullPath);
}
