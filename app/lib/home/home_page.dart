import 'package:app/home/home.dart';
import 'package:app/settings/settings.dart';
import 'package:go_router/go_router.dart';

class HomePage extends GoRoute {
  HomePage()
    : super(
        path: fullPath,
        builder: (context, state) => const HomeView(),
        routes: [SettingsPage()],
      );

  static const fullPath = '/';
}
