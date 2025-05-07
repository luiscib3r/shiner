import 'package:app/home/home.dart';
import 'package:flutter/foundation.dart';
import 'package:go_router/go_router.dart';

final appRouter = GoRouter(
  routes: [HomePage()],
  debugLogDiagnostics: kDebugMode,
);
