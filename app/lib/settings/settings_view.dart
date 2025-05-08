import 'package:app/app/app.dart';
import 'package:app/settings/widgets/settings_form.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class SettingsView extends HookConsumerWidget {
  const SettingsView({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsPod);

    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: SafeArea(
        child: switch (settings) {
          AsyncError(:final error) => Padding(
            padding: const EdgeInsets.all(24),
            child: Center(child: Text('Error loading settings $error')),
          ),
          AsyncData(:final value) => SettingsForm(settings: value),
          _ => const Center(child: CircularProgressIndicator()),
        },
      ),
    );
  }
}
