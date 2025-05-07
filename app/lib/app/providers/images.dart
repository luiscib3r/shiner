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
final imagesPod = FutureProvider((ref) async {
  final repository = ref.watch(imagesRepositoryPod);
  final pagination = ref.watch(imagesPaginationPod);

  return repository.load(pagination: pagination);
});
