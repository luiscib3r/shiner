import 'package:app/app/internal/data/database.dart';
import 'package:app/app/internal/lib.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:path_provider/path_provider.dart';

Future<String> getAppDirectory() async {
  final appDirectory = await getApplicationSupportDirectory();
  return appDirectory.path;
}

Future<ArcConnection> databaseConnection(String appDirectory) async {
  return Database.connection(appDirectory: appDirectory);
}

final appDirectoryPod = Provider<String>((ref) => throw UnimplementedError());
