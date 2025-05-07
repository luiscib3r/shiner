import 'package:app/app/internal/data/database.dart';
import 'package:path_provider/path_provider.dart';

Future<String> appDirectory() async {
  final appDirectory = await getApplicationSupportDirectory();
  return appDirectory.path;
}

Future<ArcConnection> databaseConnection() async {
  final value = await appDirectory();
  return Database.connection(appDirectory: value);
}
