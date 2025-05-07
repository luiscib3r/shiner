import 'package:app/generation/generation.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class GenerationPage extends GoRoute {
  GenerationPage()
    : super(path: _path, builder: (context, state) => const GenerationView());

  static const _path = 'generation';
  static const fullPath = '/$_path';
  static void open(BuildContext context) => context.go(fullPath);
}
