import 'package:app/generation/widgets/widgets.dart';
import 'package:flutter/material.dart';

class GenerationView extends StatelessWidget {
  const GenerationView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(appBar: AppBar(), body: const GenerationForm());
  }
}
