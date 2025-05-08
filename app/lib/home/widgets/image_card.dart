import 'dart:io';

import 'package:app/app/internal/core/images.dart';
import 'package:app/app/providers/database.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:skeletonizer/skeletonizer.dart';

class ImageCard extends ConsumerWidget {
  const ImageCard({required this.image, super.key});

  final ImageJob image;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final appDirectory = ref.watch(appDirectoryPod);

    return InkWell(
      onTap: () {},
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      child: Card(
        child: Column(
          children: [
            Expanded(
              child: ClipRRect(
                borderRadius: const BorderRadius.vertical(
                  top: Radius.circular(8),
                ),
                child: Column(
                  children: [
                    if (image.imagePath != null)
                      Expanded(
                        child: Image.file(
                          File('$appDirectory/${image.imagePath!}'),
                        ),
                      )
                    else
                      Expanded(
                        child: Skeletonizer(
                          child: Container(
                            height: double.infinity,
                            width: double.infinity,
                            color: Colors.grey.shade300,
                          ),
                        ),
                      ),
                  ],
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(8),
              child: Column(
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          image.prompt,
                          overflow: TextOverflow.ellipsis,
                          maxLines: 2,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
