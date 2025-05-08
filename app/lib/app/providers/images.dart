import 'package:app/app/internal/core/images.dart';
import 'package:app/app/internal/core/pagination.dart';
import 'package:app/app/internal/lib.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

//============================================================================
// Images Repository
//============================================================================
Future<ImageJobRepository> initImagesRepository(
  String appDirectory,
  ArcConnection db,
  ArcSettingsRepository settingsRepository,
) async {
  final repository = ImageJobRepository(
    appDirectory: appDirectory,
    db: db,
    settingsRepository: settingsRepository,
  );

  await repository.init();

  return repository;
}

final imagesRepositoryPod = Provider<ImageJobRepository>(
  (ref) => throw UnimplementedError(),
);

//============================================================================
// Images
//============================================================================
// Pagination
//============================================================================
class ImagesPaginationNotifier extends AutoDisposeNotifier<Pagination> {
  @override
  Pagination build() => Pagination();

  void setLimit(int limit) {
    state = state.setLimit(limit: limit);
  }

  void setOffset(int offset) {
    state = state.setOffset(offset: offset);
  }

  void nextPage() {
    state = state.setOffset(offset: state.offset + state.limit);
  }

  void previousPage() {
    state = state.setOffset(offset: state.offset - state.limit);
  }
}

final imagesPaginationPod =
    AutoDisposeNotifierProvider<ImagesPaginationNotifier, Pagination>(
      ImagesPaginationNotifier.new,
    );

//============================================================================
// Images stream
// ============================================================================
final _imagesStreamPod = StreamProvider((ref) {
  final repository = ref.watch(imagesRepositoryPod);
  return repository.listen();
});

//============================================================================
// Load images
//============================================================================
final imagesPod = FutureProvider.autoDispose((ref) async {
  final repository = ref.watch(imagesRepositoryPod);
  final pagination = ref.watch(imagesPaginationPod);
  final images = await repository.load(pagination: pagination);

  ref.listen(_imagesStreamPod, (prev, state) {
    final value = state.value;
    if (value != null) {
      if (images.map((e) => e.id).contains(value)) {
        ref.invalidateSelf();
      }
    }
  });

  return images;
});

//============================================================================
// Generate image
//============================================================================
final _generateImagePod = FutureProvider.family.autoDispose<int, String>((
  ref,
  prompt,
) async {
  final repository = ref.watch(imagesRepositoryPod);

  final jobId = await repository.createJob(prompt: prompt);
  ref.invalidate(imagesPod);

  return jobId;
});

class _GenerateImageNotifier extends AutoDisposeNotifier<AsyncValue<int?>> {
  @override
  AsyncValue<int?> build() => const AsyncValue.data(null);

  void call(String prompt) => state = ref.watch(_generateImagePod(prompt));
}

final generateImagePod =
    NotifierProvider.autoDispose<_GenerateImageNotifier, AsyncValue<int?>>(
      _GenerateImageNotifier.new,
    );
