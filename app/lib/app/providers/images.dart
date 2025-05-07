import 'package:app/app/internal/core/images.dart';
import 'package:app/app/internal/core/pagination.dart';
import 'package:app/app/internal/data/database.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

//============================================================================
// Images Repository
//============================================================================
Future<ImageJobRepository> initImagesRepository(ArcConnection db) async {
  final repository = ImageJobRepository(db: db);
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
// Load images
//============================================================================
final imagesPod = FutureProvider.autoDispose((ref) async {
  final repository = ref.watch(imagesRepositoryPod);
  final pagination = ref.watch(imagesPaginationPod);

  return repository.load(pagination: pagination);
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
