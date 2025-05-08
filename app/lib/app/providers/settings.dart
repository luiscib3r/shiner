import 'package:app/app/internal/core/settings.dart';
import 'package:app/app/internal/lib.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

//============================================================================
// Settings repository
//============================================================================
Future<SettingsRepository> initSettingsRepository(ArcConnection db) async {
  final repository = SettingsRepository(db: db);
  await repository.init();
  return repository;
}

final settingsRepositoryPod = Provider<SettingsRepository>(
  (ref) => throw UnimplementedError(),
);

//============================================================================
// Settings
//============================================================================
final settingsPod = FutureProvider((ref) async {
  final repository = ref.watch(settingsRepositoryPod);

  return repository.load();
});

//============================================================================
// Save settings
//============================================================================
final _saveSettingsPod = FutureProvider.family.autoDispose<void, Settings>((
  ref,
  settings,
) async {
  final repository = ref.watch(settingsRepositoryPod);

  await repository.save(settings: settings);
  ref.invalidate(settingsPod);
});

class _SaveSettingsNotifier extends AutoDisposeNotifier<AsyncValue<void>> {
  @override
  AsyncValue<void> build() => const AsyncValue.data(null);

  void call(Settings settings) => state = ref.watch(_saveSettingsPod(settings));
}

final saveSettingsPod =
    NotifierProvider.autoDispose<_SaveSettingsNotifier, AsyncValue<void>>(
      _SaveSettingsNotifier.new,
    );
