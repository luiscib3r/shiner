import 'package:app/app/app.dart';
import 'package:app/home/home_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GenerationForm extends HookConsumerWidget {
  const GenerationForm({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final formKey = useMemoized(GlobalKey<FormState>.new);
    final promptController = useTextEditingController();
    final state = ref.watch(generateImagePod);

    useEffect(() {
      ref.listen(generateImagePod, (prev, state) {
        switch (state) {
          case AsyncError():
            context.showError('Unknow error');
          case AsyncData():
            HomePage.open(context);
          case _:
        }
      });

      return null;
    });

    return SafeArea(
      child: Form(
        key: formKey,
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              TextFormField(
                enabled: !state.isLoading,
                controller: promptController,
                keyboardType: TextInputType.multiline,
                textInputAction: TextInputAction.newline,
                textCapitalization: TextCapitalization.sentences,
                decoration: const InputDecoration(labelText: 'Prompt'),
                validator: (prompt) {
                  if (prompt == null || prompt.isEmpty) {
                    return 'Please enter a prompt';
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
                                  final generateImage = ref.read(
                                    generateImagePod.notifier,
                                  );
                                  generateImage(promptController.text);
                                }
                              }
                              : null,
                      child: const Text('Generate'),
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
