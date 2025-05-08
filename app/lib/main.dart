import 'package:app/app/app.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize the Rust library
  await RustLib.init();

  // Initialize the database connection
  final appDirectory = await getAppDirectory();
  final db = await databaseConnection(appDirectory);

  // Initialize the repositories
  final settingsRepository = await initSettingsRepository(db);
  final imagesRepository = await initImagesRepository(
    appDirectory,
    db,
    settingsRepository.getArc(),
  );

  runApp(
    ProviderScope(
      overrides: [
        appDirectoryPod.overrideWithValue(appDirectory),
        settingsRepositoryPod.overrideWithValue(settingsRepository),
        imagesRepositoryPod.overrideWithValue(imagesRepository),
      ],
      child: const App(),
    ),
  );
}
