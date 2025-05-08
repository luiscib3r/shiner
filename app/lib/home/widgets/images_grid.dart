import 'package:app/app/internal/core/images.dart';
import 'package:app/home/widgets/widgets.dart';
import 'package:flutter/material.dart';

class ImagesGrid extends StatelessWidget {
  const ImagesGrid({required this.images, super.key});

  final List<ImageJob> images;

  @override
  Widget build(BuildContext context) {
    return GridView.count(
      crossAxisCount: 2,
      children: images
          .map((image) => ImageCard(image: image))
          .toList(growable: false),
    );
  }
}
