// ignore_for_file: require_trailing_commas

import 'package:app/app/app.dart';
import 'package:app/app/internal/core/settings.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class SettingsForm extends HookConsumerWidget {
  const SettingsForm({required this.settings, super.key});

  final Settings? settings;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final apiKeyController = useTextEditingController();
    final saveSettings = ref.read(saveSettingsPod.notifier);
    final state = ref.watch(saveSettingsPod);

    useEffect(() {
      apiKeyController.text = settings?.openaiApiKey ?? '';

      return null;
    }, [settings]);

    return Form(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          children: [
            TextField(
              enabled: !state.isLoading,
              controller: apiKeyController,
              minLines: 1,
              maxLines: 3,
              decoration: const InputDecoration(labelText: 'OpenAI API Key'),
            ),
            const Spacer(),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed:
                        !state.isLoading
                            ? () {
                              context.unfocus();
                              saveSettings(
                                Settings(openaiApiKey: apiKeyController.text),
                              );
                            }
                            : null,
                    child: const Text('Save'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
