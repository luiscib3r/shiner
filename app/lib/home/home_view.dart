import 'package:app/app/providers/providers.dart';
import 'package:app/generation/generation.dart';
import 'package:app/home/widgets/widgets.dart';
import 'package:app/settings/settings.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';

class HomeView extends HookConsumerWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final images = ref.watch(imagesPod);

    return Scaffold(
      appBar: AppBar(
        actions: [
          IconButton(
            onPressed: () => SettingsPage.open(context),
            icon: const Icon(LucideIcons.settings),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => GenerationPage.open(context),
        child: const Icon(LucideIcons.pencil),
      ),
      body: switch (images) {
        AsyncData(:final value) => Padding(
          padding: const EdgeInsets.all(8),
          child: ImagesGrid(images: value),
        ),
        AsyncError(:final error) => SingleChildScrollView(
          child: Text(
            'Error: $error',
            style: const TextStyle(color: Colors.red),
          ),
        ),
        _ => const Center(child: CircularProgressIndicator()),
      },
    );
  }
}
