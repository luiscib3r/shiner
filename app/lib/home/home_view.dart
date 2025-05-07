import 'package:app/settings/settings.dart';
import 'package:flutter/material.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';

class HomeView extends StatelessWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        actions: [
          IconButton(
            onPressed: () => SettingsPage.open(context),
            icon: const Icon(LucideIcons.settings),
          ),
        ],
      ),
      body: const SafeArea(child: Center(child: Text('Home View'))),
    );
  }
}
