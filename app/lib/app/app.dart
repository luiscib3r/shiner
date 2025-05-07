import 'package:app/app/router/router.dart';
import 'package:app/app/theme/theme.dart';
import 'package:flutter/material.dart';

export 'extensions/extensions.dart';
export 'internal/frb_generated.dart';
export 'providers/providers.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      themeMode: ThemeMode.dark,
      theme: appTheme,
      darkTheme: appTheme,
      routerConfig: appRouter,
      debugShowCheckedModeBanner: false,
    );
  }
}
