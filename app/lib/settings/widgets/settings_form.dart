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
    final formKey = useMemoized(GlobalKey<FormState>.new);
    final apiKeyController = useTextEditingController();
    final state = ref.watch(saveSettingsPod);

    useEffect(() {
      apiKeyController.text = settings?.openaiApiKey ?? '';

      return null;
    }, [settings]);

    return SafeArea(
      child: Form(
        key: formKey,
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              TextFormField(
                enabled: !state.isLoading,
                controller: apiKeyController,
                minLines: 1,
                maxLines: 3,
                decoration: const InputDecoration(labelText: 'OpenAI API Key'),
                validator: (apiKey) {
                  if (apiKey == null || apiKey.isEmpty) {
                    return 'Please enter an OpenAI API Key';
                  }
                  return null;
                },
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
                                if (formKey.currentState?.validate() ?? false) {
                                  final saveSettings = ref.read(
                                    saveSettingsPod.notifier,
                                  );
                                  saveSettings(
                                    Settings(
                                      openaiApiKey: apiKeyController.text,
                                    ),
                                  );
                                }
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
      ),
    );
  }
}
